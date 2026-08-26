use std::fmt::Write as _;

pub struct Node {
    tag: &'static str,
    attrs: Vec<(&'static str, String)>,
    kids: Vec<Kid>,
}

enum Kid {
    Node(Node),
    Text(String),
    Raw(String),
}

pub fn el(tag: &'static str) -> Node {
    Node {
        tag,
        attrs: Vec::new(),
        kids: Vec::new(),
    }
}

macro_rules! tags {
    ($($name:ident => $tag:literal),* $(,)?) => {
        $(
            #[must_use]
            pub fn $name() -> Node {
                el($tag)
            }
        )*
    };
}

tags! {
    a => "a", article => "article", aside => "aside", b => "b",
    body => "body", button => "button", code => "code", dd => "dd",
    div => "div", em => "em", footer => "footer", form => "form",
    h1 => "h1", h2 => "h2", h3 => "h3", h4 => "h4", head => "head",
    header => "header", html_tag => "html", i => "i", input => "input", label => "label",
    li => "li", main => "main", nav => "nav", ol => "ol",
    option => "option", p => "p", pre => "pre", s => "s",
    script => "script", section => "section", select => "select",
    small => "small", span => "span", strong => "strong",
    style => "style", sub => "sub", summary => "summary",
    sup => "sup", table => "table", tbody => "tbody", td => "td",
    textarea => "textarea", th => "th", thead => "thead",
    title => "title", tr => "tr", ul => "ul",
}

impl Node {
    #[must_use]
    pub fn attr(mut self, key: &'static str, value: impl Into<String>) -> Self {
        self.attrs.push((key, escape(&value.into())));
        self
    }

    #[must_use]
    pub fn class(self, value: impl Into<String>) -> Self {
        self.attr("class", value)
    }

    #[must_use]
    pub fn id(self, value: impl Into<String>) -> Self {
        self.attr("id", value)
    }

    #[must_use]
    pub fn text(mut self, value: impl AsRef<str>) -> Self {
        self.kids.push(Kid::Text(value.as_ref().to_string()));
        self
    }

    #[must_use]
    pub fn raw(mut self, value: impl Into<String>) -> Self {
        self.kids.push(Kid::Raw(value.into()));
        self
    }

    #[must_use]
    pub fn kid(mut self, node: Node) -> Self {
        self.kids.push(Kid::Node(node));
        self
    }

    #[must_use]
    pub fn kids<I: IntoIterator<Item = Node>>(mut self, nodes: I) -> Self {
        self.kids.extend(nodes.into_iter().map(Kid::Node));
        self
    }

    #[must_use]
    pub fn render(&self) -> String {
        let mut out = String::new();
        self.write(&mut out);
        out
    }

    fn write(&self, out: &mut String) {
        let _ = write!(out, "<{}", self.tag);
        for (key, value) in &self.attrs {
            let _ = write!(out, " {key}=\"{value}\"");
        }
        if void(self.tag) {
            out.push('>');
            return;
        }
        out.push('>');
        for kid in &self.kids {
            match kid {
                Kid::Node(node) => node.write(out),
                Kid::Text(text) => out.push_str(&escape(text)),
                Kid::Raw(html) => out.push_str(html),
            }
        }
        let _ = write!(out, "</{}>", self.tag);
    }
}

pub fn escape(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for c in value.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&#34;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

fn void(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "source"
            | "track"
            | "wbr"
    )
}
