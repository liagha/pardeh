use pardeh::{Node, div, el, escape};

#[test]
fn text_is_escaped() {
    let node = div().text("<script>alert(\"x\")</script>");
    assert_eq!(
        node.render(),
        "<div>&lt;script&gt;alert(&#34;x&#34;)&lt;/script&gt;</div>"
    );
}

#[test]
fn attributes_are_escaped() {
    let node = div().attr("title", "a\"b'c&d");
    assert!(node.render().contains("title=\"a&#34;b&#39;c&amp;d\""));
}

#[test]
fn raw_html_passes_through() {
    let node = div().raw("<b>bold</b>");
    assert_eq!(node.render(), "<div><b>bold</b></div>");
}

#[test]
fn void_elements_do_not_close() {
    assert_eq!(el("br").render(), "<br>");
    assert_eq!(
        el("input").attr("type", "text").render(),
        "<input type=\"text\">"
    );
}

#[test]
fn nesting_renders_depth_first() {
    let node = div()
        .kid(el("span").text("one"))
        .kid(div().text("two"))
        .text("tail");
    assert_eq!(
        node.render(),
        "<div><span>one</span><div>two</div>tail</div>"
    );
}

#[test]
fn escape_is_idempotent_for_plain_text() {
    assert_eq!(escape("plain text 123"), "plain text 123");
}

#[test]
fn unknown_tag_still_renders() {
    let node: Node = el("custom-widget").class("fancy");
    assert_eq!(
        node.render(),
        "<custom-widget class=\"fancy\"></custom-widget>"
    );
}
