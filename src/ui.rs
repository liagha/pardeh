// reusable ui building blocks: a signal-driven list, a titled card, and a
// labeled form field. these are app-agnostic so any pardeh page can use them.

use crate::html::{Node, button, div, form, h1, input, span, table, tbody, td, th, tr};
use crate::signal::Signals;

#[derive(Clone)]
pub struct Item {
    pub key: String,
    pub label: String,
    pub meta: String,
}

pub fn card(title: &'static str, region: Node, tools: Option<Node>) -> Node {
    div()
        .class("card")
        .kid(div().class("head").kid(h1().text(title)))
        .kid(tools.unwrap_or_else(div))
        .kid(region)
}

pub fn field(label: &str, name: &str, kind: &str, placeholder: &str) -> Node {
    div()
        .class("field")
        .kid(span().text(label))
        .kid(
            input()
                .attr("type", kind)
                .attr("name", name)
                .attr("placeholder", placeholder)
                .attr("autocomplete", "off"),
        )
}

pub fn list(
    key: &'static str,
    empty: &'static str,
    action: &'static str,
    action_label: &'static str,
) -> impl Fn(&Signals) -> Node {
    move |signals: &Signals| {
        let rows = signals.get::<Vec<Item>>(key);
        let body = if rows.is_empty() {
            tr().kid(td().attr("colspan", "4").kid(span().class("muted").text(empty)))
        } else {
            tbody().kids(rows.iter().map(|item| {
                tr().kid(td().text(item.label.clone()))
                    .kid(td().kid(span().class("muted").text(item.meta.clone())))
                    .kid(td().class("mono").text(item.key.clone()))
                    .kid(
                        td().kid(
                            form()
                                .class("inline")
                                .attr("method", "post")
                                .attr("action", format!("{action}/{}", item.key))
                                .kid(button().attr("type", "submit").text(action_label)),
                        ),
                    )
            }))
        };
        table()
            .kid(tr().kid(th()).kid(th()).kid(th()).kid(th()))
            .kid(tbody().kid(body))
    }
}
