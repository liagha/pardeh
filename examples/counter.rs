use axum::routing::{get, post};
use pardeh::{App, button, div, form, span};

#[tokio::main]
async fn main() {
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
                    app.page("pardeh counter", div().kid(count).kid(actions))
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

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7600")
        .await
        .unwrap();
    println!("http://127.0.0.1:7600");
    axum::serve(listener, router).await.unwrap();
}
