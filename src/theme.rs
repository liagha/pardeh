// the default pardeh look: a dark tokenized theme applied by App::page to
// every page. apps override the tokens or add their own rules on top.

use crate::html::{Node, style};

pub const FAVICON: &str = "data:image/svg+xml,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20viewBox%3D%220%200%2016%2016%22%3E%3Crect%20width%3D%2216%22%20height%3D%2216%22%20rx%3D%223.5%22%20fill%3D%22%23141920%22%2F%3E%3Cpath%20d%3D%22M8%203.2%2012.8%208%208%2012.8%203.2%208z%22%20fill%3D%22%239fc3ec%22%2F%3E%3C%2Fsvg%3E";

pub const THEME: &str = r#"
:root {
    color-scheme: dark;
    --bg: #0e1116;
    --panel: #151a21;
    --hover: #181f27;
    --ink: #10151b;
    --line: #232a33;
    --line2: #1d242c;
    --text: #d7dde6;
    --muted: #8b96a5;
    --faint: #7d8894;
    --accent: #6f9bd1;
    --good: #8bd3a7;
    --bad: #f2a5b5;
    --radius: 12px;
}
* { box-sizing: border-box }
body { margin: 0; background: var(--bg); color: var(--text); font: 15px/1.55 system-ui, sans-serif; padding: 2.75rem 1rem 3rem }
h1 { font-size: 1.3rem; letter-spacing: .04em; margin: 0 }
.wrap { max-width: 52rem; margin: auto; display: grid; gap: 1.1rem }
.card { background: var(--panel); border: 1px solid var(--line); border-radius: var(--radius); overflow: hidden }
.card .head { display: flex; justify-content: space-between; align-items: baseline; padding: .95rem 1.15rem .55rem }
.card .head h1 { font-size: .78rem; text-transform: uppercase; letter-spacing: .14em; color: var(--muted) }
.card .tools { display: grid; gap: .6rem; padding: .8rem 1.15rem 1.05rem; border-top: 1px solid var(--line2) }
table { width: 100%; border-collapse: collapse }
th { display: none }
td { padding: .65rem .75rem .65rem 1.15rem; border-top: 1px solid var(--line2); font-size: .92rem; vertical-align: middle }
td:first-child { width: 30% }
td:last-child { width: 1%; padding-right: 1.15rem; text-align: right; white-space: nowrap }
tbody tr { transition: background .12s }
tbody tr:hover td { background: var(--hover) }
td[colspan] { padding: 1.6rem 1rem; text-align: center; color: var(--faint) }
td[colspan] .muted { color: inherit }
.muted { color: var(--muted); font-size: .85rem }
.mono { font-family: ui-monospace, monospace; font-size: .72rem; color: #5c6773; max-width: 7rem; overflow-wrap: anywhere }
form.inline { display: inline; margin: 0 }
form.inline button { color: var(--muted) }
.acts { display: inline-flex; gap: .35rem }
button, .link { background: var(--panel); border: 1px solid var(--line); color: #cfe0f5; border-radius: 8px; padding: .32rem .8rem; cursor: pointer; font-size: .82rem; line-height: 1.45; text-decoration: none; display: inline-block; vertical-align: middle; transition: background .12s, border-color .12s, color .12s }
button:hover, .link:hover { background: var(--hover); border-color: #33414f }
button:disabled { opacity: .5; cursor: default }
.tools button[type=submit] { width: 100%; background: var(--accent); border-color: #2f5d8c; color: #0b1420 }
.tools button[type=submit]:hover { background: #82b2e8 }
input[type=text], input[type=password], textarea, input[type=email], input[type=url] { width: 100%; padding: .6rem .75rem; background: var(--ink); color: var(--text); border: 1px solid var(--line); border-radius: 8px; font-size: .95rem; font-family: inherit; transition: border-color .12s }
input[type=text]:focus, input[type=password]:focus, textarea:focus, input[type=email]:focus, input[type=url]:focus { border-color: var(--accent) }
input[type=file] { width: 100%; padding: .55rem .75rem; background: var(--ink); color: var(--muted); border: 1px dashed #33414f; border-radius: 8px; font-size: .85rem }
textarea { resize: vertical; min-height: 3.6rem }
.row { display: grid; gap: .5rem; grid-template-columns: 1fr auto; align-items: center }
.row button[type=submit] { width: auto; min-width: 4.5rem }
[data-pardeh] { overflow-x: auto }
.field { display: grid; gap: .35rem; padding: 0 0 .45rem }
.tools .field { padding-bottom: 0 }
.field span { font-size: .8rem; color: var(--muted) }
.err { color: var(--bad); font-size: .82rem; min-height: 1.25rem; padding-left: .55rem; border-left: 2px solid rgba(232, 121, 140, .4); line-height: 1.4 }
dialog { background: var(--panel); color: var(--text); border: 1px solid var(--line); border-radius: 14px; padding: 0; width: 21rem; max-width: calc(100vw - 2rem); box-shadow: 0 18px 48px rgba(0, 0, 0, .45) }
dialog::backdrop { background: rgba(4, 6, 10, .65); backdrop-filter: blur(2px) }
dialog .tools { padding: 1.25rem 1.25rem 1.15rem; display: grid; gap: .8rem; border: 0 }
dialog .title { font-size: .95rem; font-weight: 600; color: #e7eef7 }
dialog .row { grid-template-columns: 1fr 1fr }
dialog button[type=submit] { width: auto }
@media (prefers-reduced-motion: no-preference) {
  @keyframes open { from { opacity: 0; transform: translateY(6px) } to { opacity: 1; transform: none } }
  dialog[open] { animation: open .16s ease-out }
}
:focus-visible { outline: 2px solid var(--accent); outline-offset: 2px }
::selection { background: #2b3f5c; color: #f2f6fb }
* { scrollbar-width: thin; scrollbar-color: #33414f transparent }
*::-webkit-scrollbar { width: 10px; height: 10px }
*::-webkit-scrollbar-thumb { background: #33414f; border-radius: 5px; border: 2px solid transparent; background-clip: content-box }
@media (max-width: 620px) {
  body { padding: 1.3rem .8rem 2rem }
  input[type=text], input[type=password], textarea { font-size: 16px }
  td { padding: .55rem .6rem .55rem .9rem }
  td:last-child { padding-right: .9rem }
  td[colspan] { padding: 1.2rem .8rem }
  table { min-width: 24rem }
  .row { grid-template-columns: 1fr }
}
"#;

#[must_use]
pub fn theme() -> Node {
    style().raw(THEME)
}