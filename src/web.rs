use axum::Router;
use axum::http::{HeaderValue, header};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use futures_util::stream::Stream;
use serde_json::to_string;
use tokio_stream::StreamExt;

use crate::html::{Node, html_tag as html, script};
use crate::signal::{Patch, Signals};

const SCRIPT_PATH: &str = "/__pardeh/pardeh.js";
pub const SCRIPT: &str = include_str!("../assets/pardeh.js");

#[derive(Clone)]
pub struct App {
    signals: Signals,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self {
            signals: Signals::new(),
        }
    }

    #[must_use]
    pub fn signals(&self) -> &Signals {
        &self.signals
    }

    #[must_use]
    pub fn page(&self, title: &str, body: Node) -> Response {
        let shell = html()
            .kid(
                crate::html::head()
                    .kid(crate::html::title().text(title))
                    .kid(script().attr("src", SCRIPT_PATH).attr("defer", "defer")),
            )
            .kid(body);
        Html(format!("<!doctype html>{}", shell.render())).into_response()
    }

    pub fn router(&self) -> Router {
        let kind = HeaderValue::from_static("text/javascript; charset=utf-8");
        Router::new()
            .route(
                SCRIPT_PATH,
                get(|| async { ([(header::CONTENT_TYPE, kind)], SCRIPT) }),
            )
            .route(
                "/__pardeh/events",
                get({
                    let signals = self.signals.clone();
                    move || {
                        let signals = signals.clone();
                        async move { stream(signals) }
                    }
                }),
            )
    }
}

fn stream(signals: Signals) -> Sse<impl Stream<Item = Result<Event, std::convert::Infallible>>> {
    let patches = signals.subscribe();
    let frames = unfold_recv(patches).filter_map(|patch| match patch {
        Ok(Patch { region, html }) => {
            let data = to_string(&Patch { region, html }).unwrap_or_default();
            Some(Ok(Event::default().event("pd").data(data)))
        }
        Err(_) => None,
    });
    Sse::new(frames).keep_alive(KeepAlive::default())
}

fn unfold_recv(
    rx: tokio::sync::broadcast::Receiver<Patch>,
) -> impl Stream<Item = Result<Patch, tokio::sync::broadcast::error::RecvError>> {
    futures_util::stream::unfold(
        Some(rx),
        |rx: Option<tokio::sync::broadcast::Receiver<Patch>>| async move {
            let mut rx = rx?;
            let patch = rx.recv().await;
            Some((patch, Some(rx)))
        },
    )
}
