// reusable ui building blocks: a signal-driven list, a titled card, and a
// labeled form field. these are app-agnostic so any pardeh page can use them.

use crate::html::{Node, a, button, div, form, h1, input, span, table, tbody, td, th, tr};
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

pub struct Field {
    label: String,
    name: String,
    kind: String,
    auto: String,
    placeholder: String,
    id: Option<String>,
    need: bool,
    min: Option<usize>,
}

pub fn field(label: &str) -> Field {
    Field {
        label: label.to_string(),
        name: String::new(),
        kind: "text".to_string(),
        auto: "off".to_string(),
        placeholder: String::new(),
        id: None,
        need: false,
        min: None,
    }
}

impl Field {
    #[must_use]
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }

    #[must_use]
    pub fn kind(mut self, kind: &str) -> Self {
        self.kind = kind.to_string();
        self
    }

    #[must_use]
    pub fn auto(mut self, auto: &str) -> Self {
        self.auto = auto.to_string();
        self
    }

    #[must_use]
    pub fn placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    #[must_use]
    pub fn id(mut self, id: &str) -> Self {
        self.id = Some(id.to_string());
        self
    }

    #[must_use]
    pub fn need(mut self) -> Self {
        self.need = true;
        self
    }

    #[must_use]
    pub fn min(mut self, min: usize) -> Self {
        self.min = Some(min);
        self
    }

    #[must_use]
    pub fn node(self) -> Node {
        let mut bare = input()
            .attr("type", self.kind.clone())
            .attr("name", self.name.clone())
            .attr("placeholder", self.placeholder.clone())
            .attr("autocomplete", self.auto.clone());
        if self.need {
            bare = bare.attr("required", "");
        }
        if let Some(min) = self.min {
            bare = bare.attr("minlength", min.to_string());
        }
        if let Some(id) = self.id {
            bare = bare.attr("id", id);
        }
        div().class("field").kid(span().text(&self.label)).kid(bare)
    }
}

pub struct Slice {
    delete: Option<Delete>,
    href: Option<Href>,
}

pub struct Delete {
    route: &'static str,
    label: &'static str,
    ask: bool,
}

pub struct Href {
    template: &'static str,
    label: &'static str,
}

pub fn slice() -> Slice {
    Slice {
        delete: None,
        href: None,
    }
}

pub fn delete(route: &'static str, label: &'static str, ask: bool) -> Delete {
    Delete { route, label, ask }
}

pub fn href(template: &'static str, label: &'static str) -> Href {
    Href { template, label }
}

impl Slice {
    #[must_use]
    pub fn remove(mut self, delete: Delete) -> Self {
        self.delete = Some(delete);
        self
    }

    #[must_use]
    pub fn link(mut self, href: Href) -> Self {
        self.href = Some(href);
        self
    }
}

pub fn list(key: &'static str, empty: &'static str, slice: Slice) -> impl Fn(&Signals) -> Node {
    move |signals: &Signals| {
        let rows = signals.get::<Vec<Item>>(key);
        let body = if rows.is_empty() {
            tr().kid(td().attr("colspan", "4").kid(span().class("muted").text(empty)))
        } else {
            tbody().kids(rows.iter().map(|item| {
                tr().kid(td().text(item.label.clone()))
                    .kid(td().kid(span().class("muted").text(item.meta.clone())))
                    .kid(td().class("mono").text(item.key.clone()))
                    .kid(td().kid(cells(&slice, item)))
            }))
        };
        table()
            .kid(tr().kid(th()).kid(th()).kid(th()).kid(th()))
            .kid(tbody().kid(body))
    }
}

fn cells(slice: &Slice, item: &Item) -> Node {
    div().class("acts").kids(
        [slice
            .delete
            .as_ref()
            .map(|delete| post(delete, item)),
        slice
            .href
            .as_ref()
            .map(|href| link(href, item))]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
    )
}

fn post(delete: &Delete, item: &Item) -> Node {
    let mut form = form()
        .class("inline")
        .attr("method", "post")
        .attr("action", format!("{}/{}", delete.route, item.key))
        .kid(button().attr("type", "submit").text(delete.label));
    if delete.ask {
        form = form.attr("data-ask", format!("{} {}?", delete.label, item.label));
    }
    form
}

fn link(href: &Href, item: &Item) -> Node {
    a().class("link")
        .attr("href", href.template.replacen("{}", &item.key, 1))
        .attr("download", "")
        .attr("target", "_blank")
        .attr("rel", "noopener")
        .text(href.label)
}