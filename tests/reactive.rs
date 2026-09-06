use std::time::Duration;

use axum::routing::{get, post};
use pardeh::{App, button, div, form, span};
use serde_json::from_str;

#[tokio::test]
async fn set_pushes_patch_to_subscriber() {
    let app = App::new();
    app.signals().define("count", 7i64);
    app.signals().region("count", |s| {
        span().text(format!("count is {}", s.get::<i64>("count")))
    });

    let mut rx = app.signals().subscribe();

    app.signals().set("count", 42i64);
    let patch = tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("patch in time")
        .unwrap();
    assert_eq!(patch.region, "count");
    assert_eq!(patch.html, "<span>count is 42</span>");
}

#[tokio::test]
async fn region_tracks_only_signals_it_reads() {
    let app = App::new();
    app.signals().define("left", 1i64);
    app.signals().define("right", 2i64);

    let _ = app.signals().region("both", |s| {
        div().text(format!("{}", s.get::<i64>("left") + s.get::<i64>("right")))
    });
    let _ = app.signals().region("only-left", |s| {
        div().text(format!("{}", s.get::<i64>("left")))
    });

    let mut rx = app.signals().subscribe();
    app.signals().set("right", 10i64);

    let patch = tokio::time::timeout(Duration::from_secs(2), rx.recv())
        .await
        .expect("patch in time")
        .unwrap();
    assert_eq!(patch.region, "both");

    app.signals().set("left", 100i64);
    let regions = tokio::time::timeout(Duration::from_secs(2), async {
        let mut seen = Vec::new();
        while seen.len() < 2 {
            seen.push(rx.recv().await.unwrap().region);
        }
        seen.sort();
        seen
    })
    .await
    .expect("patches in time");
    assert_eq!(regions, vec!["both".to_string(), "only-left".to_string()]);
}

#[tokio::test]
async fn browser_loop_end_to_end() {
    let app = App::new();
    app.signals().define("count", 0i64);

    let page_app = app.clone();
    let bump_app = app.clone();

    let router = axum::Router::new()
        .route(
            "/",
            get(move || {
                let app = page_app.clone();
                async move {
                    let count = app.signals().region("count", |s| {
                        span().text(format!("count: {}", s.get::<i64>("count")))
                    });
                    let actions = form()
                        .attr("method", "post")
                        .attr("action", "/bump")
                        .kid(button().attr("type", "submit").text("+1"));
                    app.page("counter", div().kid(count).kid(actions))
                }
            }),
        )
        .route(
            "/bump",
            post(move || async move {
                let next = bump_app.signals().get::<i64>("count") + 1;
                bump_app.signals().set("count", next);
                "ok"
            }),
        )
        .merge(app.router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    let base = format!("http://{addr}");

    let client = reqwest::Client::new();
    let home = client
        .get(&base)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    assert!(home.contains("data-pardeh=\"count\""));
    assert!(home.contains("__pardeh/pardeh.js"));
    assert!(home.contains("count: 0"));
    assert!(home.contains("<meta charset=\"utf-8\""));
    assert!(home.contains("name=\"viewport\""));
    assert!(home.contains("<style>"));

    use tokio::io::AsyncWriteExt;
    let mut stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    stream
        .write_all(
            format!(
                "GET /__pardeh/events HTTP/1.1\r\nHost: {addr}\r\nAccept: text/event-stream\r\n\r\n"
            )
            .as_bytes(),
        )
        .await
        .unwrap();

    let mut headers = Vec::new();
    while !headers.windows(4).any(|w| w == b"\r\n\r\n") {
        let n = read_some(&mut stream).await.expect("header read");
        headers.extend_from_slice(&n);
    }

    client.post(format!("{base}/bump")).send().await.unwrap();

    let mut buffer = Vec::new();
    let deadline = tokio::time::Instant::now() + Duration::from_secs(5);
    while !buffer.windows(8).any(|w| w == b"count: 1") {
        assert!(
            tokio::time::Instant::now() < deadline,
            "never saw patch, got: {}",
            String::from_utf8_lossy(&buffer)
        );
        let chunk = tokio::time::timeout(Duration::from_secs(2), read_some(&mut stream))
            .await
            .expect("slow sse")
            .expect("read");
        buffer.extend_from_slice(&chunk);
    }

    let text = String::from_utf8_lossy(&buffer).to_string();
    let data_line = text.lines().find(|l| l.starts_with("data: ")).unwrap();
    let patch: serde_json::Value = from_str(data_line.trim_start_matches("data: ")).unwrap();
    assert_eq!(patch["region"], "count");
    assert_eq!(patch["html"], "<span>count: 1</span>");
}

async fn read_some(stream: &mut tokio::net::TcpStream) -> std::io::Result<Vec<u8>> {
    use tokio::io::AsyncReadExt;
    let mut chunk = [0u8; 512];
    let n = stream.read(&mut chunk).await?;
    Ok(chunk[..n].to_vec())
}
