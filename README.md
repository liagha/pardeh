# Pardeh

Native server-rendered web pages for axum, with signal-driven regions.
All UI logic lives in Rust; the browser is a dumb terminal that swaps HTML
fragments pushed over server-sent events. No client framework, no WASM —
one 20-line script.

```text
┌─ your axum app ─────────────────────────────┐
│ Pardeh App                                  │
│  ├─ Signals   named values, typed get/set   │
│  ├─ regions   div[data-pardeh] + draw fn    │
│  ├─ tracking  reads inside a draw register  │
│  └─ /__pardeh/events  SSE patch stream      │
└─────────────────────────────────────────────┘
        │  set("count", n+1)
        ▼
   affected regions re-render server-side
        │
        ▼  event: pd {region, html}
   browser swaps innerHTML — nothing else
```

## Quickstart

```rust
use axum::routing::{get, post};
use pardeh::{button, div, form, span, App};

#[tokio::main]
async fn main() {
    let app = App::new();
    app.signals().define("count", 0i64);

    let page = app.clone();
    let bump = app.clone();

    let router = axum::Router::new()
        .route(
            "/",
            get(move || {
                let app = page.clone();
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
                let next = bump.signals().get::<i64>("count") + 1;
                bump.signals().set("count", next);
                "ok"
            }),
        )
        .merge(app.router());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:7600").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
```

```bash
cargo run --example counter   # then open http://127.0.0.1:7600
```

Clicking **+1** posts a plain form; the handler sets a signal; every region
that read that signal re-renders on the server and the new HTML lands in the
browser over SSE. No page reload, no client code written by you.

## How it fits together

- `html` — builder tree (`div().class("card").text("hi")`), everything
  escaped by default, `raw()` as the explicit escape hatch
- `Signals` — `define`/`get`/`set` typed values behind one shared struct;
  `get` is cheap and registers interest when called inside a region draw
- `region(id, draw)` — marks a dynamic fragment; renders inline on first
  paint, re-renders automatically when any signal it read changes
- `App` — `page(title, body)` wraps a document with the runtime script,
  `router()` mounts `/__pardeh/events` (SSE) and `/__pardeh/pardeh.js`;
  merge it into your own router

Single process by design: signals live in memory, patches fan out to every
connected browser. That matches personal tools exactly and would need a
redis-style bus before it ever matched a fleet.

## Tests

```bash
cargo test
```

10 tests: escaping, void elements, nesting, signal round-trips, per-region
tracking, and an end-to-end browser loop (page load → SSE subscribe → form
action → patch frame) against a real axum server.

## Status

v0.1 — the foundation is deliberately small: rendering, signals, regions,
push. Deliberately absent until needed: routing helpers, client-side state,
diffing beyond whole-region swap, multi-node fanout.
