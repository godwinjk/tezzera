//! Phase 11 Demo — 1400×900 static PNG showcasing Phase 11 systems.
//!
//! Four panels arranged 2×2:
//!   Panel 1 (top-left)     — Macros       (#[component], #[state], view! expansions)
//!   Panel 2 (top-right)    — Analyze       (AnalyzeReport member chips, stats)
//!   Panel 3 (bottom-left)  — Snapshot CLI  (tzr snapshot flow, copy diagram)
//!   Panel 4 (bottom-right) — DX Summary    (all crates by layer, phase timeline)
//!
//! Run:    cargo run -p tezzera-examples --bin phase11_demo
//! Output: phase11_demo.png (1400×900)

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::{Color, FontCache, SkiaCanvas};

// ── Canvas dimensions ─────────────────────────────────────────────────────────
const W: u32 = 1400;
const H: u32 = 900;

const HEADER_H: f32 = 72.0;
const PANEL_W:  f32 = W as f32 / 2.0;
const PANEL_H:  f32 = (H as f32 - HEADER_H - 14.0) / 2.0;

// ── Color palette ─────────────────────────────────────────────────────────────
const BG:           Color = Color::rgb(10,  12,  20);
const PANEL_BG:     Color = Color::rgb(16,  18,  30);
const DIVIDER:      Color = Color::rgb(40,  44,  64);
const ACCENT:       Color = Color::rgb(107,  80, 200);
const ACCENT2:      Color = Color::rgb( 72, 199, 116);
const ACCENT3:      Color = Color::rgb(255, 160,  60);
const ACCENT4:      Color = Color::rgb( 80, 180, 255);
const TEXT_PRIMARY: Color = Color::rgb(230, 232, 250);
const TEXT_MUTED:   Color = Color::rgb(110, 115, 145);
const TEXT_DIM:     Color = Color::rgb( 70,  74, 100);
const CHIP_DARK:    Color = Color::rgb( 22,  26,  44);
const CODE_BG:      Color = Color::rgb(  8,  10,  18);

// ── Helpers ───────────────────────────────────────────────────────────────────

fn r(x: f32, y: f32, w: f32, h: f32) -> Rect {
    Rect { origin: Point { x, y }, size: Size { width: w, height: h } }
}
fn p(x: f32, y: f32) -> Point { Point { x, y } }

fn txt(c: &mut SkiaCanvas, f: &FontCache, s: &str, x: f32, y: f32, col: Color, px: f32) {
    c.draw_text(s, p(x, y), col, f, px);
}

fn code_line(c: &mut SkiaCanvas, f: &FontCache, s: &str, x: f32, y: f32, col: Color, px: f32) {
    txt(c, f, s, x, y, col, px);
}

fn section_label(c: &mut SkiaCanvas, f: &FontCache, label: &str, x: f32, y: f32) {
    txt(c, f, label, x, y, TEXT_MUTED, 9.0);
    c.fill_rect(r(x, y + 13.0, label.len() as f32 * 5.6, 1.0), TEXT_DIM);
}

fn chip(c: &mut SkiaCanvas, f: &FontCache, label: &str, x: f32, y: f32, accent: Color) {
    let w = label.len() as f32 * 6.5 + 12.0;
    c.fill_rect(r(x, y, w, 16.0), CHIP_DARK);
    c.fill_rect(r(x, y, 3.0, 16.0), accent);
    txt(c, f, label, x + 6.0, y + 4.0, TEXT_PRIMARY, 9.0);
}

fn bullet(c: &mut SkiaCanvas, f: &FontCache, s: &str, x: f32, y: f32, col: Color) {
    c.fill_rect(r(x, y + 4.0, 3.0, 3.0), col);
    txt(c, f, s, x + 8.0, y, TEXT_PRIMARY, 10.0);
}

fn arrow_right(c: &mut SkiaCanvas, x: f32, y: f32, w: f32) {
    c.fill_rect(r(x, y + 3.0, w, 1.0), TEXT_DIM);
    c.fill_rect(r(x + w - 4.0, y, 4.0, 7.0), TEXT_DIM);
}

// ── Panel 1 — Macros ──────────────────────────────────────────────────────────

fn panel_macros(c: &mut SkiaCanvas, f: &FontCache, ox: f32, oy: f32) {
    c.fill_rect(r(ox, oy, PANEL_W, PANEL_H), PANEL_BG);
    txt(c, f, "Macros", ox + 16.0, oy + 14.0, ACCENT, 13.0);
    txt(c, f, "tezzera-macros  \u{2014}  #[component]  \u{2022}  #[state]  \u{2022}  view!",
        ox + 16.0, oy + 30.0, TEXT_MUTED, 9.0);

    let cx = ox + 16.0;
    let mut cy = oy + 52.0;

    // ── #[component] expansion ───────────────────────────────────────────────
    section_label(c, f, "#[COMPONENT]  EXPANSION", cx, cy); cy += 18.0;

    let left_w = 230.0;
    let right_w = 350.0;
    let code_h = 58.0;

    c.fill_rect(r(cx, cy, left_w, code_h), CODE_BG);
    code_line(c, f, "#[component]",              cx + 4.0, cy + 4.0,  ACCENT,   9.5);
    code_line(c, f, "pub fn Button(",             cx + 4.0, cy + 16.0, TEXT_PRIMARY, 9.5);
    code_line(c, f, "  label: String,",           cx + 4.0, cy + 27.0, TEXT_MUTED, 9.5);
    code_line(c, f, ") -> Element { … }",         cx + 4.0, cy + 38.0, TEXT_PRIMARY, 9.5);
    code_line(c, f, "INPUT",                      cx + 4.0, cy + code_h - 6.0, TEXT_DIM, 7.5);

    arrow_right(c, cx + left_w + 4.0, cy + code_h / 2.0, 28.0);

    let rx = cx + left_w + 36.0;
    c.fill_rect(r(rx, cy, right_w, code_h), CODE_BG);
    code_line(c, f, "pub struct Button { label: String }",  rx + 4.0, cy + 4.0,  ACCENT2,  9.5);
    code_line(c, f, "impl Button {",                        rx + 4.0, cy + 16.0, TEXT_PRIMARY, 9.5);
    code_line(c, f, "  pub fn new() -> Self { … }",         rx + 4.0, cy + 27.0, TEXT_MUTED, 9.5);
    code_line(c, f, "  pub fn label(mut self, v) -> Self",  rx + 4.0, cy + 38.0, TEXT_MUTED, 9.5);
    code_line(c, f, "  pub fn build(self) -> Element",      rx + 4.0, cy + 48.0, TEXT_MUTED, 9.5);
    code_line(c, f, "OUTPUT",                               rx + 4.0, cy + code_h - 6.0, TEXT_DIM, 7.5);
    cy += code_h + 14.0;

    // ── #[state] expansion ───────────────────────────────────────────────────
    section_label(c, f, "#[STATE]  EXPANSION", cx, cy); cy += 18.0;

    c.fill_rect(r(cx, cy, left_w, 42.0), CODE_BG);
    code_line(c, f, "#[state]",             cx + 4.0, cy + 4.0,  ACCENT,       9.5);
    code_line(c, f, "pub count: i32 = 0;",  cx + 4.0, cy + 16.0, TEXT_PRIMARY, 9.5);
    code_line(c, f, "INPUT",                cx + 4.0, cy + 34.0, TEXT_DIM,     7.5);

    arrow_right(c, cx + left_w + 4.0, cy + 20.0, 28.0);

    c.fill_rect(r(rx, cy, right_w, 42.0), CODE_BG);
    code_line(c, f, "pub count:",                    rx + 4.0, cy + 4.0,  TEXT_PRIMARY, 9.5);
    code_line(c, f, "    tezzera_state::Atom<i32>",  rx + 4.0, cy + 15.0, ACCENT3,      9.5);
    code_line(c, f, "    = tezzera_state::Atom::new(0);", rx + 4.0, cy + 26.0, ACCENT3, 9.5);
    code_line(c, f, "OUTPUT",                        rx + 4.0, cy + 34.0, TEXT_DIM,     7.5);
    cy += 56.0;

    // ── view! expansion ───────────────────────────────────────────────────────
    section_label(c, f, "VIEW!  MACRO  \u{2192}  BUILDER CHAIN", cx, cy); cy += 18.0;

    c.fill_rect(r(cx, cy, left_w, 68.0), CODE_BG);
    code_line(c, f, "view! {",           cx + 4.0, cy + 4.0,  ACCENT,       9.5);
    code_line(c, f, "  Column {",        cx + 4.0, cy + 15.0, TEXT_PRIMARY, 9.5);
    code_line(c, f, "    Text {",        cx + 4.0, cy + 26.0, TEXT_PRIMARY, 9.5);
    code_line(c, f, "      content: s",  cx + 4.0, cy + 37.0, TEXT_MUTED,   9.5);
    code_line(c, f, "    }",             cx + 4.0, cy + 48.0, TEXT_PRIMARY, 9.5);
    code_line(c, f, "  }",              cx + 4.0, cy + 58.0, TEXT_PRIMARY, 9.5);
    code_line(c, f, "}",               cx + left_w - 10.0, cy + 58.0, ACCENT, 9.5);

    arrow_right(c, cx + left_w + 4.0, cy + 32.0, 28.0);

    c.fill_rect(r(rx, cy, right_w, 68.0), CODE_BG);
    code_line(c, f, "Column::new()",               rx + 4.0, cy + 4.0,  ACCENT2,      9.5);
    code_line(c, f, "  .child(",                   rx + 4.0, cy + 15.0, TEXT_PRIMARY, 9.5);
    code_line(c, f, "    Text::new()",              rx + 4.0, cy + 26.0, ACCENT4,      9.5);
    code_line(c, f, "      .content(s)",            rx + 4.0, cy + 37.0, TEXT_MUTED,   9.5);
    code_line(c, f, "  )",                          rx + 4.0, cy + 48.0, TEXT_PRIMARY, 9.5);
    code_line(c, f, ".build()",                     rx + 4.0, cy + 58.0, TEXT_MUTED,   9.5);
}

// ── Panel 2 — Analyze ─────────────────────────────────────────────────────────

fn panel_analyze(c: &mut SkiaCanvas, f: &FontCache, ox: f32, oy: f32) {
    c.fill_rect(r(ox, oy, PANEL_W, PANEL_H), PANEL_BG);
    txt(c, f, "Analyze", ox + 16.0, oy + 14.0, ACCENT2, 13.0);
    txt(c, f, "tzr analyze  \u{2014}  workspace health \u{2022} crate count \u{2022} member list",
        ox + 16.0, oy + 30.0, TEXT_MUTED, 9.0);

    let cx = ox + 16.0;
    let mut cy = oy + 52.0;

    // Command
    section_label(c, f, "COMMAND", cx, cy); cy += 18.0;
    c.fill_rect(r(cx, cy - 2.0, PANEL_W - 32.0, 18.0), CODE_BG);
    code_line(c, f, "$ tzr analyze --verbose", cx + 4.0, cy + 1.0, ACCENT4, 9.5);
    cy += 24.0;

    // Output
    section_label(c, f, "ANALYZE REPORT  OUTPUT", cx, cy); cy += 18.0;
    c.fill_rect(r(cx, cy - 2.0, PANEL_W - 32.0, 52.0), CODE_BG);
    code_line(c, f, "Workspace: tezzera",        cx + 4.0, cy + 2.0,  TEXT_PRIMARY, 9.5);
    code_line(c, f, "  Crates:  27",             cx + 4.0, cy + 14.0, ACCENT2,      9.5);
    code_line(c, f, "  Status:  OK",             cx + 4.0, cy + 26.0, ACCENT2,      9.5);
    code_line(c, f, "  Members: tezzera-core, tezzera-state, …", cx + 4.0, cy + 38.0, TEXT_MUTED, 9.5);
    cy += 62.0;

    // Member chips (simulated — actual crates in the workspace)
    section_label(c, f, "WORKSPACE MEMBERS  (27 crates)", cx, cy); cy += 18.0;

    let all_crates = [
        ("tezzera-core",       ACCENT),
        ("tezzera-state",      ACCENT),
        ("tezzera-layout",     ACCENT),
        ("tezzera-render",     ACCENT),
        ("tezzera-trace",      ACCENT),
        ("tezzera-macros",     ACCENT2),
        ("tezzera-widgets",    ACCENT2),
        ("tezzera-forms",      ACCENT2),
        ("tezzera-style",      ACCENT2),
        ("tezzera-theme",      ACCENT2),
        ("tezzera-platform",   ACCENT3),
        ("tezzera-nav",        ACCENT3),
        ("tezzera-nav-anim",   ACCENT3),
        ("tezzera-scroll",     ACCENT3),
        ("tezzera-gesture",    ACCENT3),
        ("tezzera-net",        ACCENT4),
        ("tezzera-ws",         ACCENT4),
        ("tezzera-media",      ACCENT4),
        ("tezzera-i18n",       ACCENT4),
        ("tezzera-bidi",       ACCENT4),
        ("tezzera-text",       ACCENT4),
        ("tezzera-shaping",    ACCENT4),
        ("tezzera-anim",       Color::rgb(200, 100, 200)),
        ("tezzera-animate",    Color::rgb(200, 100, 200)),
        ("tezzera-a11y",       Color::rgb(200, 100, 200)),
        ("tezzera-test-utils", Color::rgb(200, 100, 200)),
        ("tezzera-cli",        Color::rgb(160, 160, 80)),
    ];

    let mut chip_x = cx;
    let chip_row_max_x = ox + PANEL_W - 16.0;
    for (name, color) in &all_crates {
        let w = name.len() as f32 * 6.5 + 16.0;
        if chip_x + w > chip_row_max_x {
            chip_x = cx;
            cy += 20.0;
        }
        chip(c, f, name, chip_x, cy, *color);
        chip_x += w + 4.0;
    }
    cy += 28.0;

    // Stats row
    let stats: &[(&str, &str, Color)] = &[
        ("27",   "total crates",     ACCENT2),
        ("100%", "build success",    ACCENT2),
        ("OK",   "workspace status", ACCENT2),
    ];
    for (i, (val, label, col)) in stats.iter().enumerate() {
        let sx = cx + i as f32 * 140.0;
        txt(c, f, val, sx, cy, *col, 18.0);
        txt(c, f, label, sx, cy + 20.0, TEXT_MUTED, 9.0);
    }
}

// ── Panel 3 — Snapshot CLI ────────────────────────────────────────────────────

fn panel_snapshot(c: &mut SkiaCanvas, f: &FontCache, ox: f32, oy: f32) {
    c.fill_rect(r(ox, oy, PANEL_W, PANEL_H), PANEL_BG);
    txt(c, f, "Snapshot CLI", ox + 16.0, oy + 14.0, ACCENT3, 13.0);
    txt(c, f, "tzr snapshot  \u{2014}  run example \u{2022} copy PNG \u{2022} visual regression ready",
        ox + 16.0, oy + 30.0, TEXT_MUTED, 9.0);

    let cx = ox + 16.0;
    let mut cy = oy + 52.0;

    // Command
    section_label(c, f, "COMMAND", cx, cy); cy += 18.0;
    c.fill_rect(r(cx, cy - 2.0, PANEL_W - 32.0, 30.0), CODE_BG);
    code_line(c, f, "$ tzr snapshot --example phase10_demo --out snapshots/",
              cx + 4.0, cy + 1.0, ACCENT4, 9.5);
    code_line(c, f, "Saved: snapshots/phase10_demo.png",
              cx + 4.0, cy + 15.0, ACCENT2, 9.5);
    cy += 38.0;

    // Flow diagram
    section_label(c, f, "SNAPSHOT FLOW", cx, cy); cy += 18.0;

    let steps: &[(&str, Color)] = &[
        ("\u{25b6}  parse SnapshotOptions { example, out_dir, package }", ACCENT4),
        ("\u{25b6}  cargo run -p tezzera-examples --bin <example> --release", ACCENT4),
        ("\u{25b6}  wait for example to write <example>.png to cwd", ACCENT4),
        ("\u{25b6}  fs::copy(<example>.png → <out_dir>/<example>.png)", ACCENT4),
        ("\u{2713}  CommandResult { exit_code: 0, success: true }", ACCENT2),
    ];
    for (desc, col) in steps {
        bullet(c, f, desc, cx, cy, *col);
        cy += 14.0;
    }
    cy += 10.0;

    // Before/After mockup
    section_label(c, f, "BEFORE / AFTER  COMPARISON FLOW", cx, cy); cy += 18.0;

    // "Run" box
    let box_w = 90.0;
    let box_h = 30.0;
    let box_y = cy;
    c.fill_rect(r(cx, box_y, box_w, box_h), CHIP_DARK);
    c.fill_rect(r(cx, box_y, 3.0, box_h), ACCENT4);
    txt(c, f, "tzr snapshot",  cx + 6.0, box_y + 6.0,  ACCENT4, 8.5);
    txt(c, f, "runs example",  cx + 6.0, box_y + 17.0, TEXT_MUTED, 8.0);

    arrow_right(c, cx + box_w + 2.0, box_y + 14.0, 20.0);

    // "PNG" box
    let p2x = cx + box_w + 26.0;
    c.fill_rect(r(p2x, box_y, box_w, box_h), CHIP_DARK);
    c.fill_rect(r(p2x, box_y, 3.0, box_h), ACCENT3);
    txt(c, f, "example.png",  p2x + 6.0, box_y + 6.0,  ACCENT3, 8.5);
    txt(c, f, "cwd output",   p2x + 6.0, box_y + 17.0, TEXT_MUTED, 8.0);

    arrow_right(c, p2x + box_w + 2.0, box_y + 14.0, 20.0);

    // "Save" box
    let p3x = p2x + box_w + 26.0;
    c.fill_rect(r(p3x, box_y, 110.0, box_h), CHIP_DARK);
    c.fill_rect(r(p3x, box_y, 3.0, box_h), ACCENT2);
    txt(c, f, "snapshots/",        p3x + 6.0, box_y + 6.0,  ACCENT2, 8.5);
    txt(c, f, "example.png saved", p3x + 6.0, box_y + 17.0, TEXT_MUTED, 8.0);

    cy += box_h + 16.0;

    // Integration with SnapshotAssert
    section_label(c, f, "INTEGRATION WITH TEZZERA-TEST-UTILS", cx, cy); cy += 18.0;

    let integrate: &[(&str, Color)] = &[
        ("run `tzr snapshot` to produce golden PNG",  ACCENT3),
        ("use SnapshotAssert::assert_snapshot() in tests", ACCENT3),
        ("pixel_diff_count = 0  \u{21d2}  visual regression guard", ACCENT2),
    ];
    for (s, col) in integrate {
        bullet(c, f, s, cx, cy, *col);
        cy += 14.0;
    }
}

// ── Panel 4 — DX Summary ──────────────────────────────────────────────────────

fn panel_dx(c: &mut SkiaCanvas, f: &FontCache, ox: f32, oy: f32) {
    c.fill_rect(r(ox, oy, PANEL_W, PANEL_H), PANEL_BG);
    txt(c, f, "DX Summary", ox + 16.0, oy + 14.0, ACCENT4, 13.0);
    txt(c, f, "Full crate map by layer  \u{2022}  Phase 1\u{2192}11 timeline",
        ox + 16.0, oy + 30.0, TEXT_MUTED, 9.0);

    let cx = ox + 16.0;
    let mut cy = oy + 52.0;

    section_label(c, f, "CRATE LAYERS", cx, cy); cy += 18.0;

    let layers: &[(&str, &[&str], Color)] = &[
        ("Foundation",  &["tezzera-trace", "tezzera-core", "tezzera-state", "tezzera-layout", "tezzera-render", "tezzera-macros"], ACCENT),
        ("Widgets",     &["tezzera-widgets", "tezzera-forms", "tezzera-style", "tezzera-theme", "tezzera-text"], ACCENT2),
        ("Platform",    &["tezzera-platform", "tezzera-gesture", "tezzera-scroll", "tezzera-nav", "tezzera-nav-anim"], ACCENT3),
        ("Network/I/O", &["tezzera-net", "tezzera-ws", "tezzera-media", "tezzera-clipboard", "tezzera-ime"], ACCENT4),
        ("Intl/Text",   &["tezzera-i18n", "tezzera-bidi", "tezzera-shaping"], Color::rgb(180, 100, 220)),
        ("DX",          &["tezzera-anim", "tezzera-animate", "tezzera-a11y", "tezzera-test-utils", "tezzera-devtools", "tezzera-hot-reload"], Color::rgb(200, 80, 140)),
        ("CLI",         &["tezzera-cli", "tezzera-examples"], Color::rgb(160, 160, 80)),
    ];

    for (layer_name, crates, color) in layers {
        txt(c, f, layer_name, cx, cy, *color, 9.5);
        let mut chip_x = cx + 92.0;
        for name in *crates {
            let w = name.len() as f32 * 6.0 + 10.0;
            if chip_x + w > ox + PANEL_W - 16.0 { break; } // clip to panel
            c.fill_rect(r(chip_x, cy - 1.0, w, 14.0), CHIP_DARK);
            c.fill_rect(r(chip_x, cy - 1.0, 2.0, 14.0), *color);
            txt(c, f, name, chip_x + 4.0, cy + 2.0, TEXT_MUTED, 7.5);
            chip_x += w + 3.0;
        }
        cy += 18.0;
    }
    cy += 8.0;

    // Phase timeline
    section_label(c, f, "PHASE TIMELINE  (1 \u{2192} 11)", cx, cy); cy += 18.0;

    let phases: &[(&str, &str, Color)] = &[
        ("P1",  "Foundation",         ACCENT),
        ("P2",  "Widgets",            ACCENT),
        ("P3",  "Layout+",            ACCENT),
        ("P4",  "Platform",           ACCENT2),
        ("P5",  "Network",            ACCENT2),
        ("P6",  "Forms+Gesture",      ACCENT2),
        ("P7",  "Media+Clipboard",    ACCENT3),
        ("P8",  "Hot-reload+DevT",    ACCENT3),
        ("P9",  "Shaping+Style+CLI",  ACCENT4),
        ("P10", "Anim+A11y+Test",     Color::rgb(200, 80, 140)),
        ("P11", "Macros+DX \u{2713}", Color::rgb(80, 220, 120)),
    ];

    let bar_total = PANEL_W - 32.0;
    let bar_w = bar_total / phases.len() as f32 - 2.0;
    for (i, (phase, label, color)) in phases.iter().enumerate() {
        let bx = cx + i as f32 * (bar_w + 2.0);
        let is_current = i == phases.len() - 1;
        let bh = if is_current { 28.0 } else { 22.0 };
        c.fill_rect(r(bx, cy, bar_w, bh), if is_current { *color } else { CHIP_DARK });
        c.fill_rect(r(bx, cy, bar_w, 2.0), *color);
        txt(c, f, phase, bx + 2.0, cy + 4.0, if is_current { Color::rgb(10, 12, 20) } else { *color }, 8.5);
        txt(c, f, label, bx, cy + bh + 3.0, TEXT_DIM, 7.0);
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let font = FontCache::system_mono().expect("no system mono font");
    let mut c = SkiaCanvas::new(W, H);
    c.clear(BG);

    // Header
    c.fill_rect(r(0.0, 0.0, W as f32, 3.0), ACCENT);
    txt(&mut c, &font, "TEZZERA \u{2014} Phase 11 Showcase",
        28.0, 22.0, TEXT_PRIMARY, 20.0);
    txt(&mut c, &font,
        "Macros  \u{2022}  Analyze  \u{2022}  Snapshot CLI  \u{2022}  DX Summary",
        28.0, 50.0, TEXT_MUTED, 10.0);

    // Panels
    let top_y    = HEADER_H;
    let bottom_y = HEADER_H + PANEL_H;
    panel_macros  (&mut c, &font, 0.0,     top_y);
    panel_analyze (&mut c, &font, PANEL_W, top_y);
    panel_snapshot(&mut c, &font, 0.0,     bottom_y);
    panel_dx      (&mut c, &font, PANEL_W, bottom_y);

    // Grid lines
    c.fill_rect(r(PANEL_W, HEADER_H, 1.0, PANEL_H * 2.0), DIVIDER);
    c.fill_rect(r(0.0, bottom_y, W as f32, 1.0), DIVIDER);
    c.fill_rect(r(0.0, HEADER_H - 1.0, W as f32, 1.0), DIVIDER);

    // Status bar
    let sb_y = H as f32 - 14.0;
    c.fill_rect(r(0.0, sb_y - 4.0, W as f32, 18.0), Color::rgb(8, 10, 16));
    txt(&mut c, &font,
        "TEZZERA  \u{2022}  Phase 11  \u{2022}  Macros \u{2713}  Analyze \u{2713}  Snapshot \u{2713}  DX \u{2713}  27 Crates",
        W as f32 / 2.0 - 300.0, sb_y - 1.0, TEXT_MUTED, 10.0);

    let png = c.encode_png().expect("png encode failed");
    std::fs::write("phase11_demo.png", &png).expect("write phase11_demo.png");
    println!("Saved phase11_demo.png ({}x{})", W, H);
}
