//! Phase 9 Demo — 1400×900 static PNG showcasing Phase 9 systems.
//!
//! Four panels arranged 2×2:
//!   Panel 1 (top-left)     — Text Shaping    (FallbackShaper, GlyphRun, Script, ShapingPipeline)
//!   Panel 2 (top-right)    — Style System    (StyleSheet, StyleRule, ComputedStyle, InlineStyle)
//!   Panel 3 (bottom-left)  — CLI Commands    (tzr check, test, lint, fmt + CommandResult)
//!   Panel 4 (bottom-right) — 25-Crate Map    (layer-grouped chip grid)
//!
//! Run:    cargo run -p tezzera-examples --bin phase9_demo
//! Output: phase9_demo.png (1400×900)

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::{Color, FontCache, SkiaCanvas};

// ── Phase 9 crate imports ─────────────────────────────────────────────────────

// Text shaping
use tezzera_shaping::{ShapingPipeline, Script};
use tezzera_text::TextDirection;

// Style system
use tezzera_style::{
    ComputedStyle, InlineStyle, Selector, StyleProperty, StyleRule, StyleSheet, StyleValue,
};
use tezzera_theme::Color as ThemeColor;

// ── Local CommandResult ── tezzera-cli is a binary crate, not a lib ───────────
#[derive(Debug, Clone)]
struct CommandResult {
    command:     String,
    exit_code:   i32,
    duration_ms: u64,
    success:     bool,
}

impl CommandResult {
    fn mock(command: &str, exit_code: i32, duration_ms: u64) -> Self {
        Self { command: command.to_string(), exit_code, duration_ms, success: exit_code == 0 }
    }
    fn summary(&self) -> String {
        if self.success {
            format!("{} \u{2713} {}ms", self.command, self.duration_ms)
        } else {
            format!("{} \u{2717} exit {}", self.command, self.exit_code)
        }
    }
}

// ── Canvas dimensions ─────────────────────────────────────────────────────────
const W: u32 = 1400;
const H: u32 = 900;

// ── Color palette ─────────────────────────────────────────────────────────────
const BG:           Color = Color::rgb(10,  12,  20);
const PANEL_BG:     Color = Color::rgb(16,  18,  30);
const DIVIDER:      Color = Color::rgb(40,  44,  64);
const ACCENT:       Color = Color::rgb(107,  80, 200);
const ACCENT2:      Color = Color::rgb( 72, 199, 116);
const ACCENT3:      Color = Color::rgb(255, 160,  60);
const ACCENT4:      Color = Color::rgb( 80, 180, 255);
const ACCENT5:      Color = Color::rgb(200,  80, 255);
const TEXT_PRIMARY: Color = Color::rgb(230, 230, 245);
const TEXT_MUTED:   Color = Color::rgb(120, 125, 155);
const CARD_BG:      Color = Color::rgb(22,  24,  40);

// ── Layout (2×2 grid) ─────────────────────────────────────────────────────────
const HEADER_H: f32 = 80.0;
const PANEL_W:  f32 = 700.0;
const PANEL_H:  f32 = 410.0;

// ── Shared helpers ────────────────────────────────────────────────────────────

fn lbl(c: &mut SkiaCanvas, font: &FontCache, text: &str, x: f32, y: f32) {
    c.draw_text(text, Point { x, y }, TEXT_MUTED, font, 10.0);
}

fn section_label(c: &mut SkiaCanvas, font: &FontCache, text: &str, x: f32, y: f32, color: Color) {
    c.draw_text(text, Point { x, y }, color, font, 13.0);
}

fn card_box(
    c: &mut SkiaCanvas,
    x: f32, y: f32, w: f32, h: f32,
    fill: Color, stroke: Color, stroke_w: f32,
) {
    c.fill_rect(
        Rect { origin: Point { x, y }, size: Size { width: w, height: h } },
        fill,
    );
    c.stroke_rect(
        Rect { origin: Point { x, y }, size: Size { width: w, height: h } },
        stroke,
        stroke_w,
    );
}

fn divider(c: &mut SkiaCanvas, x: f32, y: f32, w: f32) {
    c.fill_rect(
        Rect { origin: Point { x, y }, size: Size { width: w, height: 1.0 } },
        DIVIDER,
    );
}

// ── Panel 1 — Text Shaping ────────────────────────────────────────────────────

fn panel_shaping(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32) {
    c.fill_rect(
        Rect { origin: Point { x, y }, size: Size { width: PANEL_W, height: PANEL_H } },
        PANEL_BG,
    );

    let ix = x + 20.0;
    let content_w = PANEL_W - 30.0;
    let mut cy = y + 14.0;

    section_label(c, font, "Text Shaping  (D042 — HarfBuzz prep stub)", ix, cy, ACCENT);
    cy += 24.0;

    // ── GlyphRun demo ─────────────────────────────────────────────────────────
    lbl(c, font, "shape(\"Hello TEZZERA\", 16px, LTR) — one glyph per character:", ix, cy);
    cy += 13.0;

    let demo_text = "Hello TEZZERA";
    let pipeline = ShapingPipeline::new();
    let run = pipeline.shape(demo_text, 16.0, TextDirection::Ltr);

    let glyph_bw: f32 = 44.0;
    let glyph_bh: f32 = 52.0;
    let palette = [ACCENT, ACCENT2, ACCENT3, ACCENT4, ACCENT5];

    // If font is available use real data; else fall back to data-model display.
    let display_chars: Vec<(char, f32, u32)> = if run.is_empty() {
        demo_text
            .chars()
            .enumerate()
            .map(|(i, ch)| (ch, 9.0_f32, i as u32))
            .collect()
    } else {
        run.glyphs
            .iter()
            .map(|g| (g.ch, g.x_advance, g.cluster))
            .collect()
    };

    for (i, (ch, adv, cluster)) in display_chars.iter().enumerate() {
        let gx = ix + i as f32 * (glyph_bw + 2.0);
        let col = palette[i % palette.len()];
        card_box(c, gx, cy, glyph_bw, glyph_bh, CARD_BG, col, 1.0);
        c.draw_text(
            &ch.to_string(),
            Point { x: gx + 15.0, y: cy + 7.0 },
            TEXT_PRIMARY, font, 13.0,
        );
        c.draw_text(
            &format!("{:.1}px", adv),
            Point { x: gx + 4.0, y: cy + 29.0 },
            col, font, 7.5,
        );
        c.draw_text(
            &format!("b{}", cluster),
            Point { x: gx + 4.0, y: cy + 41.0 },
            TEXT_MUTED, font, 7.0,
        );
    }
    cy += glyph_bh + 8.0;

    let source = if run.is_empty() { "(data model — no system font)" } else { "(FallbackShaper via fontdue)" };
    lbl(
        c, font,
        &format!(
            "glyphs={}  total_advance={:.1}px  script={:?}  {}",
            if run.is_empty() { demo_text.chars().count() } else { run.glyph_count() },
            if run.is_empty() { demo_text.chars().count() as f32 * 9.0 } else { run.total_advance() },
            run.script,
            source,
        ),
        ix, cy,
    );
    cy += 16.0;

    // ── Script detection ──────────────────────────────────────────────────────
    divider(c, ix, cy, content_w); cy += 10.0;
    lbl(c, font, "Script::detect() — Unicode script identification:", ix, cy); cy += 13.0;

    let script_samples: &[(&str, &str, Color)] = &[
        ("Hello",
         "Latin",
         ACCENT4),
        // Arabic "مرحبا"
        ("\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}",
         "Arabic",
         ACCENT3),
        // Hebrew "שלום"
        ("\u{05E9}\u{05DC}\u{05D5}\u{05DD}",
         "Hebrew",
         ACCENT3),
        // Hiragana "こんにちは" — falls through to Unknown (no Han/Kana range yet)
        ("\u{3053}\u{3093}\u{306B}\u{3061}\u{306F}",
         "Unknown",
         TEXT_MUTED),
    ];

    let chip_w = (content_w - 10.0) / 2.0;
    for (i, (sample, _expected, color)) in script_samples.iter().enumerate() {
        let col_x = ix + (i % 2) as f32 * (chip_w + 10.0);
        let row_y = cy + (i / 2) as f32 * 30.0;
        let detected = Script::detect(sample);
        let display = match i {
            1 => "\"\\u{0645}\\u{0631}\\u{062D}...\"",
            2 => "\"\\u{05E9}\\u{05DC}\\u{05D5}...\"",
            3 => "\"\\u{3053}\\u{3093}...\"",
            _ => sample,
        };
        card_box(c, col_x, row_y, chip_w, 26.0, CARD_BG, *color, 1.0);
        c.draw_text(
            &format!("{} \u{2192} {:?}", display, detected),
            Point { x: col_x + 8.0, y: row_y + 8.0 },
            *color, font, 9.0,
        );
    }
    cy += 2.0 * 30.0 + 8.0;

    // ── ShapingPipeline ───────────────────────────────────────────────────────
    divider(c, ix, cy, content_w); cy += 10.0;
    lbl(c, font, "ShapingPipeline:", ix, cy); cy += 13.0;

    let pipeline2 = ShapingPipeline::new();
    let sample_run = pipeline2.shape("TEZZERA", 14.0, TextDirection::Ltr);
    card_box(c, ix, cy, content_w, 48.0, CARD_BG, ACCENT, 1.0);
    c.draw_text(
        "ShapingPipeline::new()  —  zero engines + FallbackShaper slot",
        Point { x: ix + 10.0, y: cy + 6.0 },
        TEXT_MUTED, font, 9.0,
    );
    c.draw_text(
        &format!(
            "engine_count={}  has_fallback={}  shape(\"TEZZERA\", 14px) \u{2192} {} glyphs",
            pipeline2.engine_count(),
            pipeline2.has_fallback(),
            sample_run.glyph_count(),
        ),
        Point { x: ix + 10.0, y: cy + 22.0 },
        ACCENT, font, 9.0,
    );
    c.draw_text(
        "v1.0: HarfBuzzShaper implements ShapingEngine and is pushed in here",
        Point { x: ix + 10.0, y: cy + 36.0 },
        TEXT_MUTED, font, 8.0,
    );

    // Footer
    let footer_y = y + PANEL_H - 22.0;
    lbl(c, font, "FallbackShaper (fontdue) \u{2022} 1 glyph/char \u{2022} HarfBuzz slot D042", ix, footer_y);
}

// ── Panel 2 — Style System ────────────────────────────────────────────────────

fn panel_style(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32) {
    c.fill_rect(
        Rect { origin: Point { x, y }, size: Size { width: PANEL_W, height: PANEL_H } },
        PANEL_BG,
    );

    let ix = x + 20.0;
    let content_w = PANEL_W - 30.0;
    let mut cy = y + 14.0;

    section_label(c, font, "Style System  (CSS-like stylesheet + computed values)", ix, cy, ACCENT2);
    cy += 24.0;

    // ── Build StyleSheet ──────────────────────────────────────────────────────
    lbl(c, font, "StyleSheet with 3 rules:", ix, cy); cy += 13.0;

    // ThemeColor values for use with tezzera_style
    let btn_bg_theme  = ThemeColor::rgb(103.0/255.0, 80.0/255.0, 164.0/255.0); // #6750A4
    let white_theme   = ThemeColor::rgb(1.0, 1.0, 1.0);                         // #FFFFFF

    let mut sheet = StyleSheet::new();
    sheet.add_rule(
        StyleRule::new(Selector::class("btn"))
            .set(StyleProperty::Background,   StyleValue::color(btn_bg_theme))
            .set(StyleProperty::Padding,      StyleValue::px(12.0))
            .set(StyleProperty::BorderRadius, StyleValue::px(8.0)),
    );
    sheet.add_rule(
        StyleRule::new(Selector::class("text"))
            .set(StyleProperty::Color,    StyleValue::color(white_theme))
            .set(StyleProperty::FontSize, StyleValue::px(16.0)),
    );
    sheet.add_rule(
        StyleRule::new(Selector::id("title"))
            .set(StyleProperty::FontSize,   StyleValue::px(24.0))
            .set(StyleProperty::FontWeight, StyleValue::Number(700.0)),
    );

    // Render color swatch + rule card for each rule
    let rules_display: &[(&str, &str, Color, &[(&str, &str)])] = &[
        (
            ".btn",
            "#6750A4",
            Color::rgb(103, 80, 164),
            &[("background", "#6750A4"), ("padding", "12px"), ("border-radius", "8px")],
        ),
        (
            ".text",
            "#FFFFFF",
            Color::rgb(200, 200, 220),
            &[("color", "#FFFFFF"), ("font-size", "16px")],
        ),
        (
            "#title",
            "",
            ACCENT4,
            &[("font-size", "24px"), ("font-weight", "700")],
        ),
    ];

    for (selector, _hex, accent, props) in rules_display {
        let rule_h = 14.0 + props.len() as f32 * 13.0 + 8.0;
        card_box(c, ix, cy, content_w, rule_h, CARD_BG, *accent, 1.0);
        // Swatch + selector
        c.fill_rect(
            Rect { origin: Point { x: ix + 2.0, y: cy + 2.0 }, size: Size { width: 3.0, height: rule_h - 4.0 } },
            *accent,
        );
        c.draw_text(selector, Point { x: ix + 10.0, y: cy + 4.0 }, *accent, font, 10.0);
        // Properties
        for (i, (prop, val)) in props.iter().enumerate() {
            c.draw_text(
                &format!("  {}: {}", prop, val),
                Point { x: ix + 16.0, y: cy + 16.0 + i as f32 * 13.0 },
                TEXT_MUTED, font, 8.5,
            );
        }
        cy += rule_h + 5.0;
    }

    // ── ComputedStyle::resolve ────────────────────────────────────────────────
    divider(c, ix, cy, content_w); cy += 10.0;
    lbl(c, font, "ComputedStyle::resolve(&sheet, Selector::class(\"btn\"), None):", ix, cy); cy += 13.0;

    let computed = ComputedStyle::resolve(&sheet, &Selector::class("btn"), None);

    let bg_str = computed.background()
        .map(|col| format!("rgb({:.0},{:.0},{:.0})", col.r*255.0, col.g*255.0, col.b*255.0))
        .unwrap_or_else(|| "—".to_string());
    let pad_str  = computed.padding_px().map(|v| format!("{}px", v)).unwrap_or_else(|| "—".to_string());
    let brad_str = computed.border_radius().map(|v| format!("{}px", v)).unwrap_or_else(|| "—".to_string());

    card_box(c, ix, cy, content_w, 52.0, CARD_BG, ACCENT2, 1.0);
    c.fill_rect(
        Rect { origin: Point { x: ix + 2.0, y: cy + 2.0 }, size: Size { width: 3.0, height: 48.0 } },
        ACCENT2,
    );
    c.draw_text(
        &format!("background \u{2192} {}", bg_str),
        Point { x: ix + 10.0, y: cy + 6.0 },
        ACCENT2, font, 9.0,
    );
    c.draw_text(
        &format!("padding    \u{2192} {}  |  border-radius \u{2192} {}", pad_str, brad_str),
        Point { x: ix + 10.0, y: cy + 20.0 },
        TEXT_MUTED, font, 9.0,
    );
    c.draw_text(
        &format!("property_count = {}  opacity = {:.1}", computed.property_count(), computed.opacity()),
        Point { x: ix + 10.0, y: cy + 34.0 },
        TEXT_MUTED, font, 8.5,
    );
    cy += 58.0;

    // ── InlineStyle override ──────────────────────────────────────────────────
    lbl(c, font, "InlineStyle override (highest specificity):", ix, cy); cy += 13.0;

    let mut inline = InlineStyle::new();
    inline.set(StyleProperty::Padding, StyleValue::px(20.0));
    inline.set(StyleProperty::BorderRadius, StyleValue::px(16.0));

    let computed2 = ComputedStyle::resolve(&sheet, &Selector::class("btn"), Some(&inline));
    let new_pad   = computed2.padding_px().map(|v| format!("{}px", v)).unwrap_or_else(|| "—".to_string());
    let new_brad  = computed2.border_radius().map(|v| format!("{}px", v)).unwrap_or_else(|| "—".to_string());

    card_box(c, ix, cy, content_w, 44.0, CARD_BG, ACCENT5, 1.0);
    c.fill_rect(
        Rect { origin: Point { x: ix + 2.0, y: cy + 2.0 }, size: Size { width: 3.0, height: 40.0 } },
        ACCENT5,
    );
    c.draw_text(
        "inline: { padding: 20px, border-radius: 16px }",
        Point { x: ix + 10.0, y: cy + 6.0 },
        ACCENT5, font, 9.0,
    );
    c.draw_text(
        &format!("resolved: padding={} border-radius={}  (overrides sheet)", new_pad, new_brad),
        Point { x: ix + 10.0, y: cy + 22.0 },
        TEXT_MUTED, font, 8.5,
    );
    c.draw_text(
        &format!("sheet had padding=12px border-radius=8px \u{2192} both overridden"),
        Point { x: ix + 10.0, y: cy + 34.0 },
        TEXT_MUTED, font, 8.0,
    );

    // Footer
    let footer_y = y + PANEL_H - 22.0;
    lbl(c, font, "StyleSheet \u{2022} StyleRule \u{2022} ComputedStyle \u{2022} InlineStyle \u{2022} Selector", ix, footer_y);
}

// ── Panel 3 — CLI Commands ────────────────────────────────────────────────────

fn panel_cli(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32) {
    c.fill_rect(
        Rect { origin: Point { x, y }, size: Size { width: PANEL_W, height: PANEL_H } },
        PANEL_BG,
    );

    let ix = x + 20.0;
    let content_w = PANEL_W - 30.0;
    let mut cy = y + 14.0;

    section_label(c, font, "CLI Commands  (tzr workspace commands)", ix, cy, ACCENT3);
    cy += 24.0;

    // ── 4 command boxes in a 2×2 grid ────────────────────────────────────────
    let cmds: &[(&str, &str, u64, Color)] = &[
        ("tzr check", "cargo check --workspace",              1180, ACCENT),
        ("tzr test",  "cargo test  --workspace",              4250, ACCENT2),
        ("tzr lint",  "cargo clippy --workspace -- -D warns", 2310, ACCENT3),
        ("tzr fmt",   "cargo fmt   --workspace --check",       620, ACCENT4),
    ];

    let box_w   = (content_w - 10.0) / 2.0;
    let box_h   = 80.0_f32;
    let box_gap = 10.0_f32;

    for (i, (cmd, runs, dur_ms, color)) in cmds.iter().enumerate() {
        let bx = ix + (i % 2) as f32 * (box_w + box_gap);
        let by = cy + (i / 2) as f32 * (box_h + box_gap);

        let result = CommandResult::mock(cmd, 0, *dur_ms);

        card_box(c, bx, by, box_w, box_h, CARD_BG, *color, 1.5);
        // Left color bar
        c.fill_rect(
            Rect { origin: Point { x: bx + 2.0, y: by + 2.0 }, size: Size { width: 3.0, height: box_h - 4.0 } },
            *color,
        );
        // Command name
        c.draw_text(cmd, Point { x: bx + 10.0, y: by + 8.0 }, *color, font, 11.0);
        // What it runs
        c.draw_text(runs, Point { x: bx + 10.0, y: by + 24.0 }, TEXT_MUTED, font, 8.0);
        // Result summary
        c.draw_text(&result.summary(), Point { x: bx + 10.0, y: by + 40.0 }, ACCENT2, font, 8.5);
        // exit_code + duration badges
        card_box(c, bx + 10.0, by + 54.0, 70.0, 18.0, Color::rgb(14, 24, 18), ACCENT2, 1.0);
        c.draw_text(
            &format!("exit={}", result.exit_code),
            Point { x: bx + 14.0, y: by + 58.0 },
            ACCENT2, font, 8.0,
        );
        card_box(c, bx + 86.0, by + 54.0, 80.0, 18.0, CARD_BG, TEXT_MUTED, 1.0);
        c.draw_text(
            &format!("{}ms", result.duration_ms),
            Point { x: bx + 90.0, y: by + 58.0 },
            TEXT_MUTED, font, 8.0,
        );
    }
    cy += 2.0 * (box_h + box_gap) + 5.0;

    // ── CommandResult struct fields ───────────────────────────────────────────
    divider(c, ix, cy, content_w); cy += 10.0;
    lbl(c, font, "CommandResult struct  (inlined — tezzera-cli/src/commands/workspace.rs):", ix, cy); cy += 13.0;

    let fields: &[(&str, &str, Color)] = &[
        ("command:     String",   "the tzr sub-command label",       ACCENT4),
        ("exit_code:   i32",      "process exit code (0 = success)", ACCENT2),
        ("stdout:      String",   "captured standard output",        TEXT_MUTED),
        ("stderr:      String",   "captured standard error",         TEXT_MUTED),
        ("duration_ms: u64",      "wall-clock ms from Instant",      ACCENT3),
        ("success:     bool",     "exit_code == 0",                  ACCENT2),
    ];

    let fbox_h = 18.0_f32;
    for (field, desc, color) in fields {
        card_box(c, ix, cy, content_w, fbox_h, CARD_BG, DIVIDER, 1.0);
        c.fill_rect(
            Rect { origin: Point { x: ix + 2.0, y: cy + 2.0 }, size: Size { width: 3.0, height: fbox_h - 4.0 } },
            *color,
        );
        c.draw_text(field, Point { x: ix + 10.0, y: cy + 4.0 }, *color, font, 8.5);
        c.draw_text(desc,  Point { x: ix + 195.0, y: cy + 4.0 }, TEXT_MUTED, font, 8.0);
        cy += fbox_h + 2.0;
    }

    // Footer
    let footer_y = y + PANEL_H - 22.0;
    lbl(c, font, "tzr check \u{2022} tzr test \u{2022} tzr lint \u{2022} tzr fmt \u{2022} CommandResult", ix, footer_y);
}

// ── Panel 4 — All 25 Crates ───────────────────────────────────────────────────

fn panel_crates(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32) {
    c.fill_rect(
        Rect { origin: Point { x, y }, size: Size { width: PANEL_W, height: PANEL_H } },
        PANEL_BG,
    );

    let ix = x + 20.0;
    let mut cy = y + 14.0;

    section_label(c, font, "TEZZERA — All 25 Crates  (layer architecture)", ix, cy, ACCENT4);
    cy += 24.0;

    // Each entry: (layer name, color, crates list)
    let layers: &[(&str, Color, &[&str])] = &[
        ("Core     (4)", ACCENT,  &["tezzera-core", "tezzera-trace", "tezzera-state", "tezzera-theme"]),
        ("Layout   (1)", ACCENT2, &["tezzera-layout"]),
        ("Render   (2)", ACCENT3, &["tezzera-render", "tezzera-renderer"]),
        ("Text     (5)", ACCENT4, &["tezzera-text", "tezzera-bidi", "tezzera-shaping", "tezzera-i18n", "tezzera-fonts"]),
        ("Input    (3)", ACCENT5, &["tezzera-platform", "tezzera-gesture", "tezzera-ime"]),
        ("Widgets  (3)", Color::rgb(255, 90, 130),  &["tezzera-widgets", "tezzera-nav", "tezzera-nav-anim"]),
        ("Style    (2)", Color::rgb(90, 220, 220),  &["tezzera-style", "tezzera-theme \u{2020}"]),
        ("IO       (3)", Color::rgb(255, 200, 60),  &["tezzera-net", "tezzera-ws", "tezzera-clipboard"]),
        ("Media    (1)", Color::rgb(180, 120, 255), &["tezzera-media"]),
        ("CLI      (1)", Color::rgb(100, 210, 180), &["tezzera-cli"]),
    ];

    let chip_w: f32 = 130.0;
    let chip_h: f32 = 22.0;
    let chip_gap: f32 = 6.0;
    let label_col_w: f32 = 100.0;
    let chips_start_x = ix + label_col_w + 8.0;
    let max_chips_per_row = ((PANEL_W - 30.0 - label_col_w - 8.0) / (chip_w + chip_gap)) as usize;

    for (layer_name, color, crates) in layers {
        // Layer label
        c.draw_text(layer_name, Point { x: ix, y: cy + 5.0 }, *color, font, 9.0);

        // Chips
        for (i, crate_name) in crates.iter().enumerate() {
            let row = i / max_chips_per_row;
            let col = i % max_chips_per_row;
            let cx_val = chips_start_x + col as f32 * (chip_w + chip_gap);
            let cy_val = cy + row as f32 * (chip_h + 3.0);

            card_box(c, cx_val, cy_val, chip_w, chip_h, CARD_BG, *color, 1.0);
            c.fill_rect(
                Rect { origin: Point { x: cx_val + 2.0, y: cy_val + 2.0 }, size: Size { width: 3.0, height: chip_h - 4.0 } },
                *color,
            );
            // Abbreviate long names to fit chip
            let display = if crate_name.len() > 16 {
                crate_name.trim_start_matches("tezzera-")
            } else {
                crate_name
            };
            c.draw_text(display, Point { x: cx_val + 8.0, y: cy_val + 6.0 }, *color, font, 7.5);
        }

        let rows = (crates.len() + max_chips_per_row - 1) / max_chips_per_row;
        cy += rows as f32 * (chip_h + 3.0) + 5.0;
    }

    // Footnote
    c.draw_text(
        "\u{2020} tezzera-theme listed under Core; shown here for Style grouping",
        Point { x: ix, y: cy + 2.0 },
        TEXT_MUTED, font, 7.5,
    );

    // Footer
    let footer_y = y + PANEL_H - 22.0;
    lbl(c, font, "25 crates \u{2022} 10 layers \u{2022} Core \u{2192} Layout \u{2192} Render \u{2192} Text \u{2192} Widgets", ix, footer_y);
}

// ── main ─────────────────────────────────────────────────────────────────────

fn main() {
    let font = FontCache::system_mono().expect("system font not found");
    let mut c = SkiaCanvas::new(W, H);

    // Global background
    c.clear(BG);

    // ── Header bar (80px) ─────────────────────────────────────────────────────
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: 0.0 }, size: Size { width: W as f32, height: HEADER_H } },
        Color::rgb(13, 15, 26),
    );
    // Accent top line
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: 0.0 }, size: Size { width: W as f32, height: 3.0 } },
        ACCENT,
    );
    c.draw_text(
        "TEZZERA \u{2014} Phase 9 Showcase",
        Point { x: 28.0, y: 28.0 },
        TEXT_PRIMARY, &font, 22.0,
    );
    c.draw_text(
        "Text Shaping  \u{2022}  Style System  \u{2022}  CLI Commands  \u{2022}  25-Crate Architecture",
        Point { x: 28.0, y: 56.0 },
        TEXT_MUTED, &font, 11.0,
    );

    // ── 2×2 Panel grid ────────────────────────────────────────────────────────
    let top_y    = HEADER_H;
    let bottom_y = HEADER_H + PANEL_H;

    panel_shaping(&mut c, &font, 0.0,     top_y);     // Panel 1 — top-left
    panel_style  (&mut c, &font, PANEL_W, top_y);     // Panel 2 — top-right
    panel_cli    (&mut c, &font, 0.0,     bottom_y);  // Panel 3 — bottom-left
    panel_crates (&mut c, &font, PANEL_W, bottom_y);  // Panel 4 — bottom-right

    // ── Grid lines ────────────────────────────────────────────────────────────
    // Vertical centre divider
    c.fill_rect(
        Rect { origin: Point { x: PANEL_W, y: HEADER_H }, size: Size { width: 1.0, height: PANEL_H * 2.0 } },
        DIVIDER,
    );
    // Horizontal mid divider
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: bottom_y }, size: Size { width: W as f32, height: 1.0 } },
        DIVIDER,
    );
    // Header bottom border
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: HEADER_H - 1.0 }, size: Size { width: W as f32, height: 1.0 } },
        DIVIDER,
    );

    // ── Status bar ────────────────────────────────────────────────────────────
    let sb_y = H as f32 - 14.0;
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: sb_y - 4.0 }, size: Size { width: W as f32, height: 18.0 } },
        Color::rgb(8, 10, 16),
    );
    c.draw_text(
        "TEZZERA  \u{2022}  Phase 9  \u{2022}  Shaping \u{2713}  Style \u{2713}  CLI \u{2713}  25 Crates \u{2713}",
        Point { x: W as f32 / 2.0 - 260.0, y: sb_y - 1.0 },
        TEXT_MUTED, &font, 10.0,
    );

    // Encode and write
    let png = c.encode_png().expect("png encode failed");
    std::fs::write("phase9_demo.png", &png).expect("write phase9_demo.png");
    println!("Saved phase9_demo.png ({}x{})", W, H);
}
