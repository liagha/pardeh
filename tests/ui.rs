use pardeh::{App, Item, delete, field, href, list, slice};

#[test]
fn field_renders_label_input_and_validation() {
    let node = field("password")
        .name("pass")
        .kind("password")
        .auto("current-password")
        .placeholder("your password")
        .id("pw")
        .need()
        .min(8)
        .node();
    let html = node.render();
    assert!(html.contains("class=\"field\""));
    assert!(html.contains("<span>password</span>"));
    assert!(html.contains("type=\"password\""));
    assert!(html.contains("required"));
    assert!(html.contains("minlength=\"8\""));
    assert!(html.contains("id=\"pw\""));
}

#[test]
fn list_renders_rows_with_delete_and_download() {
    let app = App::new();
    app.signals()
        .define("files", vec![Item { key: "a1".into(), label: "note.txt".into(), meta: "now".into() }]);
    let region = app.signals().region(
        "files",
        list(
            "files",
            "no files yet",
            slice()
                .remove(delete("/web/file", "delete", true))
                .link(href("/api/v1/files/{}/content", "download")),
        ),
    );
    let html = region.render();
    assert!(html.contains("note.txt"));
    assert!(html.contains("action=\"/web/file/a1\""));
    assert!(html.contains("data-ask=\"delete note.txt?\""));
    assert!(html.contains("href=\"/api/v1/files/a1/content\""));
    assert!(html.contains("download"));
}

#[test]
fn list_shows_empty_state_when_no_rows() {
    let app = App::new();
    app.signals().define("devices", Vec::<Item>::new());
    let region = app.signals().region(
        "devices",
        list("devices", "no devices yet", slice().remove(delete("/web/device", "revoke", true))),
    );
    let html = region.render();
    assert!(html.contains("no devices yet"));
    assert!(html.contains("colspan=\"4\""));
    assert!(!html.contains("revoke"));
}