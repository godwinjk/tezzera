//! Phase 8 Demo — 1400×900 static PNG showcasing Phase 8 systems.
//!
//! Four panels (350 wide each, y=80–900):
//!   Panel 1 — Renderer Abstraction  (SkiaRenderer, RendererBackend, Renderer)
//!   Panel 2 — IME Input             (NoopIme, ImeHandler, ImeEvent, ImeComposition, ImeState)
//!   Panel 3 — Unicode Bidi          (BidiParagraph, bidi_class, BidiClass, paragraph_level)
//!   Panel 4 — Media Stubs           (AudioPlayer, VideoDecoder, AudioFormat, VideoFormat,
//!                                    MediaError, VideoFrame)
//!
//! Run:    cargo run -p tezzera-examples --bin phase8_demo
//! Output: phase8_demo.png (1400×900)

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::{Color, FontCache, SkiaCanvas};

// ── Phase 8 crate imports ─────────────────────────────────────────────────────

// Renderer abstraction
use tezzera_renderer::{RendererBackend, Renderer, SkiaRenderer};

// IME input
use tezzera_ime::{ImeComposition, ImeEvent, ImeHandler, ImeState, NoopIme};

// Unicode bidi
use tezzera_bidi::{bidi_class, BidiClass, BidiParagraph, paragraph_level, resolve_levels};

// Media stubs
use tezzera_media::{AudioFormat, AudioPlayer, MediaError, VideoDecoder, VideoFormat, VideoFrame};

// ── Canvas dimensions ─────────────────────────────────────────────────────────
const W: u32 = 1400;
const H: u32 = 900;

// ── Color palette ─────────────────────────────────────────────────────────────
const BG:           Color = Color::rgb(10,  12,  20);
const PANEL_BG:     Color = Color::rgb(16,  18,  30);
const DIVIDER:      Color = Color::rgb(40,  44,  64);
const ACCENT:       Color = Color::rgb(107, 80, 200);
const ACCENT2:      Color = Color::rgb( 72, 199, 116);
const ACCENT3:      Color = Color::rgb(255, 160,  60);
const ACCENT4:      Color = Color::rgb( 80, 180, 255);
const ACCENT5:      Color = Color::rgb(200,  80, 255);
const TEXT_PRIMARY: Color = Color::rgb(230, 230, 245);
const TEXT_MUTED:   Color = Color::rgb(120, 125, 155);
const CARD_BG:      Color = Color::rgb(22,  24,  40);

// ── Layout ────────────────────────────────────────────────────────────────────
const HEADER_H: f32 = 80.0;
const PANEL_W:  f32 = 350.0;
const PANEL_H:  f32 = 820.0;

fn px(n: usize) -> f32 { n as f32 * PANEL_W }

// ── Shared helpers ────────────────────────────────────────────────────────────

fn lbl(c: &mut SkiaCanvas, font: &FontCache, text: &str, x: f32, y: f32) {
    c.draw_text(text, Point { x, y }, TEXT_MUTED, font, 10.0);
}

fn section_label(c: &mut SkiaCanvas, font: &FontCache, text: &str, x: f32, y: f32, color: Color) {
    c.draw_text(text, Point { x, y }, color, font, 13.0);
}

fn card_box(c: &mut SkiaCanvas, x: f32, y: f32, w: f32, h: f32, fill: Color, stroke: Color, stroke_w: f32) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, fill);
    c.stroke_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, stroke, stroke_w);
}

// ── Panel 1 — Renderer Abstraction ───────────────────────────────────────────

fn panel_renderer(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: PANEL_W, height: PANEL_H } }, PANEL_BG);

    let ix = x + 20.0;
    let mut cy = y + 14.0;

    section_label(c, font, "Renderer Abstraction (D032)", ix, cy, ACCENT);
    cy += 24.0;

    // ── Trait hierarchy diagram ───────────────────────────────────────────────
    lbl(c, font, "Trait hierarchy:", ix, cy);
    cy += 14.0;

    let bw = 200.0_f32;
    let bh = 32.0_f32;
    let bx = ix + 20.0;

    // Top box — trait Renderer
    card_box(c, bx, cy, bw, bh, CARD_BG, ACCENT, 1.5);
    c.draw_text("trait Renderer", Point { x: bx + 10.0, y: cy + 9.0 }, ACCENT, font, 11.0);
    cy += bh;

    // Arrow + label
    let ax = bx + bw / 2.0;
    c.fill_rect(Rect { origin: Point { x: ax - 1.0, y: cy }, size: Size { width: 2.0, height: 14.0 } }, DIVIDER);
    c.draw_text("\u{21D3} implements", Point { x: ax + 4.0, y: cy + 2.0 }, TEXT_MUTED, font, 9.0);
    cy += 16.0;

    // Bottom box — SkiaRenderer
    card_box(c, bx, cy, bw, bh, CARD_BG, ACCENT2, 1.5);
    c.draw_text("SkiaRenderer", Point { x: bx + 10.0, y: cy + 9.0 }, ACCENT2, font, 11.0);
    cy += bh + 4.0;

    c.draw_text(
        "\u{2192} v1.0: SkiaSafeRenderer",
        Point { x: bx + 8.0, y: cy },
        TEXT_MUTED, font, 9.0,
    );
    cy += 20.0;

    // ── Backend card ──────────────────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    lbl(c, font, "backend().description():", ix, cy);
    cy += 14.0;

    let renderer = SkiaRenderer::new(200, 100);
    let backend: RendererBackend = renderer.backend();
    let desc = backend.description();

    card_box(c, ix, cy, 310.0, 44.0, Color::rgb(14, 16, 28), ACCENT, 1.0);
    c.draw_text(
        &format!("{:?}", backend),
        Point { x: ix + 8.0, y: cy + 6.0 },
        ACCENT4, font, 9.0,
    );
    // Description split across two lines (~30 chars each)
    let split = desc.len().min(32);
    c.draw_text(&desc[..split], Point { x: ix + 8.0, y: cy + 20.0 }, TEXT_MUTED, font, 8.0);
    if desc.len() > split {
        c.draw_text(&desc[split..], Point { x: ix + 8.0, y: cy + 31.0 }, TEXT_MUTED, font, 8.0);
    }
    cy += 52.0;

    // ── Trait method list ─────────────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    lbl(c, font, "Trait methods (Box<dyn Renderer>):", ix, cy);
    cy += 14.0;

    let methods: &[(&str, &str)] = &[
        ("clear(color)",               "clear entire surface"),
        ("fill_rect(rect, color)",     "filled rectangle"),
        ("stroke_rect(rect, color, w)","outlined rectangle"),
        ("fill_circle(center, r, col)","filled circle"),
        ("draw_text(text, pos, ...)",  "text at position"),
        ("encode_png() -> Vec<u8>",    "PNG encode surface"),
        ("width() -> u32",             "surface width in px"),
        ("height() -> u32",            "surface height in px"),
    ];

    for (sig, desc_m) in methods {
        card_box(c, ix, cy, 310.0, 26.0, CARD_BG, DIVIDER, 1.0);
        c.fill_rect(Rect { origin: Point { x: ix + 2.0, y: cy + 2.0 }, size: Size { width: 3.0, height: 22.0 } }, ACCENT);
        c.draw_text(sig,    Point { x: ix + 10.0, y: cy + 5.0  }, ACCENT4,    font, 8.0);
        c.draw_text(desc_m, Point { x: ix + 10.0, y: cy + 16.0 }, TEXT_MUTED, font, 7.5);
        cy += 28.0;
    }

    cy += 4.0;

    // ── Mini canvas demo ──────────────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    lbl(c, font, "Mini canvas encode demo:", ix, cy);
    cy += 13.0;

    let mut mini = SkiaRenderer::new(200, 100);
    mini.clear(Color::rgb(8, 10, 18));
    mini.fill_rect(
        Rect { origin: Point { x: 10.0, y: 10.0 }, size: Size { width: 80.0, height: 40.0 } },
        ACCENT,
    );
    mini.fill_circle(Point { x: 150.0, y: 50.0 }, 28.0, ACCENT2);
    let png_bytes = mini.encode_png();
    let kb = png_bytes.len() / 1024;

    card_box(c, ix, cy, 310.0, 40.0, CARD_BG, ACCENT2, 1.0);
    c.draw_text(
        "SkiaRenderer::new(200, 100)",
        Point { x: ix + 8.0, y: cy + 6.0 },
        TEXT_MUTED, font, 9.0,
    );
    c.draw_text(
        &format!("encode_png() \u{2192} {}KB ({} bytes)", kb, png_bytes.len()),
        Point { x: ix + 8.0, y: cy + 22.0 },
        ACCENT2, font, 9.0,
    );

    // Stats footer
    let footer_y = y + PANEL_H - 28.0;
    lbl(c, font, "30 tests \u{2022} Box<dyn Renderer> \u{2022} D032 swap point", ix, footer_y);
}

// ── Panel 2 — IME Input ───────────────────────────────────────────────────────

fn panel_ime(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: PANEL_W, height: PANEL_H } }, PANEL_BG);

    let ix = x + 20.0;
    let mut cy = y + 14.0;

    section_label(c, font, "IME Input", ix, cy, ACCENT2);
    cy += 24.0;

    // ── State machine diagram ─────────────────────────────────────────────────
    lbl(c, font, "ImeState machine:", ix, cy);
    cy += 14.0;

    let states: &[(&str, Color, &str)] = &[
        ("Idle",      TEXT_MUTED, "no active session"),
        ("Enabled",   ACCENT4,    "IME active, waiting"),
        ("Composing", ACCENT,     "preedit in progress"),
        ("Committed", ACCENT2,    "text ready to insert"),
    ];
    let arrow_labels = [
        "\u{2192} ImeEvent::Enabled",
        "\u{2192} Preedit { .. }",
        "\u{2192} Commit(..)",
    ];

    let bw = 130.0_f32;
    let bh = 38.0_f32;
    let bx = ix;

    for (i, (name, color, hint)) in states.iter().enumerate() {
        card_box(c, bx, cy, bw, bh, CARD_BG, *color, 1.5);
        c.draw_text(name, Point { x: bx + 8.0, y: cy + 8.0  }, *color,     font, 11.0);
        c.draw_text(hint, Point { x: bx + 8.0, y: cy + 24.0 }, TEXT_MUTED, font, 8.0);
        cy += bh;

        if i < states.len() - 1 {
            let mid_x = bx + bw / 2.0;
            c.fill_rect(
                Rect { origin: Point { x: mid_x - 1.0, y: cy }, size: Size { width: 2.0, height: 10.0 } },
                DIVIDER,
            );
            c.draw_text(arrow_labels[i], Point { x: bx + bw + 6.0, y: cy }, TEXT_MUTED, font, 8.0);
            cy += 12.0;
        }
    }
    cy += 10.0;

    // ── Preedit simulation box ────────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    lbl(c, font, "Preedit simulation:", ix, cy);
    cy += 14.0;

    card_box(c, ix, cy, 310.0, 38.0, CARD_BG, ACCENT, 1.5);
    // Japanese "nihon" preedit: にほん
    c.draw_text(
        "\u{306B}\u{307B}\u{3093}",
        Point { x: ix + 10.0, y: cy + 10.0 },
        TEXT_PRIMARY, font, 14.0,
    );
    // Underline simulation (preedit underline)
    c.fill_rect(
        Rect { origin: Point { x: ix + 10.0, y: cy + 26.0 }, size: Size { width: 44.0, height: 2.0 } },
        ACCENT,
    );
    // Cursor bar
    c.fill_rect(
        Rect { origin: Point { x: ix + 56.0, y: cy + 8.0 }, size: Size { width: 2.0, height: 18.0 } },
        ACCENT3,
    );
    c.draw_text("[preedit]", Point { x: ix + 200.0, y: cy + 12.0 }, TEXT_MUTED, font, 9.0);
    cy += 46.0;

    // ── Live NoopIme demo ─────────────────────────────────────────────────────
    lbl(c, font, "Live NoopIme demo:", ix, cy);
    cy += 14.0;

    let mut ime = NoopIme::new();
    ime.on_ime_event(&ImeEvent::Enabled);
    ime.on_ime_event(&ImeEvent::Preedit {
        text: "nihon".to_string(),
        cursor_range: Some((0, 5)),
    });

    // Capture composition during preedit
    let comp: &ImeComposition = ime.composition();
    let preedit_text = comp.text.clone();
    let preedit_active = comp.active;

    ime.on_ime_event(&ImeEvent::Commit("\u{65E5}\u{672C}".to_string())); // 日本

    let committed = ime.last_committed().unwrap_or("");
    let state: &ImeState = ime.state();
    let state_str = match state {
        ImeState::Committed { .. } => "Committed",
        ImeState::Composing { .. } => "Composing",
        ImeState::Enabled          => "Enabled",
        ImeState::Idle             => "Idle",
    };

    // Code block
    let code_lines = [
        "let mut ime = NoopIme::new();",
        "ime.on_ime_event(&ImeEvent::Enabled);",
        "ime.on_ime_event(&ImeEvent::Preedit {",
        "    text: \"nihon\",  cursor_range: Some((0,5))",
        "});",
        "ime.on_ime_event(&ImeEvent::Commit(\"\\u65E5\\u672C\"));",
    ];
    let code_h = code_lines.len() as f32 * 12.0 + 8.0;
    card_box(c, ix, cy, 310.0, code_h, CARD_BG, DIVIDER, 1.0);
    for (i, line) in code_lines.iter().enumerate() {
        c.draw_text(line, Point { x: ix + 6.0, y: cy + 4.0 + i as f32 * 12.0 }, TEXT_MUTED, font, 8.0);
    }
    cy += code_h + 8.0;

    // Preedit capture result
    lbl(c, font, &format!("preedit.text=\"{}\"  active={}", preedit_text, preedit_active), ix, cy);
    cy += 13.0;

    // Committed + state badges
    card_box(c, ix, cy, 148.0, 28.0, Color::rgb(14, 24, 18), ACCENT2, 1.0);
    c.draw_text(
        &format!("committed: \"{}\"", committed),
        Point { x: ix + 6.0, y: cy + 8.0 },
        ACCENT2, font, 9.0,
    );
    card_box(c, ix + 158.0, cy, 152.0, 28.0, CARD_BG, ACCENT4, 1.0);
    c.draw_text(
        &format!("state: {}", state_str),
        Point { x: ix + 164.0, y: cy + 8.0 },
        ACCENT4, font, 9.0,
    );
    cy += 36.0;

    // ── ImeEvent variants ─────────────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    lbl(c, font, "ImeEvent variants:", ix, cy);
    cy += 13.0;

    let events: &[(&str, Color, &str)] = &[
        ("Preedit { text, cursor_range }", ACCENT,     "text being composed"),
        ("Commit(String)",                 ACCENT2,    "text confirmed for insert"),
        ("Enabled",                        ACCENT4,    "IME became active"),
        ("Disabled",                       TEXT_MUTED, "IME became inactive"),
    ];
    for (variant, color, desc) in events {
        card_box(c, ix, cy, 310.0, 30.0, CARD_BG, *color, 1.0);
        c.draw_text(variant, Point { x: ix + 8.0, y: cy + 6.0  }, *color,     font, 9.0);
        c.draw_text(desc,    Point { x: ix + 8.0, y: cy + 19.0 }, TEXT_MUTED, font, 8.0);
        cy += 32.0;
    }

    // Stats footer
    let footer_y = y + PANEL_H - 28.0;
    lbl(c, font, "38 tests \u{2022} CJK composition model", ix, footer_y);
}

// ── Panel 3 — Unicode Bidi ────────────────────────────────────────────────────

fn panel_bidi(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: PANEL_W, height: PANEL_H } }, PANEL_BG);

    let ix = x + 20.0;
    let mut cy = y + 14.0;

    section_label(c, font, "Unicode Bidi (TR#9)", ix, cy, ACCENT3);
    cy += 24.0;

    // ── Paragraph analysis ────────────────────────────────────────────────────
    lbl(c, font, "BidiParagraph analysis:", ix, cy);
    cy += 14.0;

    let para_samples: &[(&str, &str)] = &[
        ("Hello World",                    "\"Hello World\""),
        ("\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}", "\"\\u{0645}\\u{0631}\\u{062D}...\" (Arabic)"),
    ];

    for (text, label) in para_samples {
        let para = BidiParagraph::new(*text);
        let dir_label = if para.is_ltr() { "LTR" } else { "RTL" };
        let dir_color = if para.is_ltr() { ACCENT4 } else { ACCENT3 };

        card_box(c, ix, cy, 310.0, 48.0, CARD_BG, DIVIDER, 1.0);
        c.draw_text(label, Point { x: ix + 8.0, y: cy + 6.0 }, TEXT_PRIMARY, font, 9.0);
        c.draw_text(
            &format!("base_level={}  rtl_chars={}", para.base_level, para.rtl_char_count()),
            Point { x: ix + 8.0, y: cy + 20.0 },
            TEXT_MUTED, font, 8.0,
        );
        // LTR/RTL badge
        c.fill_rect(
            Rect { origin: Point { x: ix + 222.0, y: cy + 12.0 }, size: Size { width: 60.0, height: 22.0 } },
            dir_color,
        );
        c.draw_text(dir_label, Point { x: ix + 237.0, y: cy + 16.0 }, Color::rgb(10, 12, 20), font, 11.0);
        cy += 52.0;
    }

    cy += 4.0;

    // ── Per-character level visualization ─────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    lbl(c, font, "Per-char level (\"Hi\" + Arabic \\u0628):", ix, cy);
    cy += 13.0;

    // "Hi" + Arabic letter ب (U+0628): levels should be [0, 0, 1]
    let mix = "Hi\u{0628}";
    let mix_levels = resolve_levels(mix);
    let mix_chars: Vec<char> = mix.chars().collect();

    let char_bw = 40.0_f32;
    let char_bh = 48.0_f32;

    for (i, ch) in mix_chars.iter().enumerate() {
        let level = mix_levels.get(i).copied().unwrap_or(0);
        let color = if level == 0 { ACCENT4 } else { ACCENT3 };
        let cx_val = ix + i as f32 * (char_bw + 6.0);
        card_box(c, cx_val, cy, char_bw, char_bh, CARD_BG, color, 1.5);
        c.draw_text(&ch.to_string(), Point { x: cx_val + 14.0, y: cy + 10.0 }, TEXT_PRIMARY, font, 12.0);
        c.draw_text(&format!("L{}", level), Point { x: cx_val + 12.0, y: cy + 30.0 }, color, font, 9.0);
    }
    // Legend
    let legend_x = ix + mix_chars.len() as f32 * (char_bw + 6.0) + 8.0;
    c.fill_rect(Rect { origin: Point { x: legend_x, y: cy + 8.0 }, size: Size { width: 10.0, height: 10.0 } }, ACCENT4);
    c.draw_text("L0=LTR", Point { x: legend_x + 14.0, y: cy + 8.0  }, ACCENT4, font, 8.0);
    c.fill_rect(Rect { origin: Point { x: legend_x, y: cy + 26.0 }, size: Size { width: 10.0, height: 10.0 } }, ACCENT3);
    c.draw_text("L1=RTL", Point { x: legend_x + 14.0, y: cy + 26.0 }, ACCENT3, font, 8.0);
    cy += char_bh + 12.0;

    // ── bidi_class table ──────────────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    lbl(c, font, "bidi_class() samples:", ix, cy);
    cy += 13.0;

    // Column headers
    c.draw_text("Char", Point { x: ix,          y: cy }, TEXT_MUTED, font, 9.0);
    c.draw_text("Class", Point { x: ix + 60.0,  y: cy }, TEXT_MUTED, font, 9.0);
    c.draw_text("Meaning", Point { x: ix + 120.0, y: cy }, TEXT_MUTED, font, 9.0);
    cy += 12.0;
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 5.0;

    let class_samples: &[(char, BidiClass, &str, Color)] = &[
        ('A',        BidiClass::L,  "Latin letter (LTR)",  ACCENT4),
        ('5',        BidiClass::EN, "European number",     ACCENT2),
        ('\u{0639}', BidiClass::AL, "Arabic letter (AL)",  ACCENT3),  // ع
        ('\u{05D0}', BidiClass::R,  "Hebrew letter (R)",   ACCENT3),  // א
        (' ',        BidiClass::WS, "Whitespace",          TEXT_MUTED),
    ];

    for (ch, expected_class, meaning, color) in class_samples {
        let actual_class = bidi_class(*ch);
        let class_str = format!("{:?}", actual_class);
        let ch_str = if *ch == ' ' { "' '".to_string() } else { format!("'{}'", ch) };
        // Highlight mismatch (should never happen)
        let row_color = if actual_class == *expected_class { TEXT_PRIMARY } else { ACCENT3 };
        c.draw_text(&ch_str,    Point { x: ix,          y: cy }, row_color,  font, 9.0);
        c.draw_text(&class_str, Point { x: ix + 60.0,   y: cy }, *color,     font, 9.0);
        c.draw_text(meaning,    Point { x: ix + 120.0,  y: cy }, TEXT_MUTED, font, 8.0);
        cy += 13.0;
    }

    cy += 8.0;

    // ── paragraph_level examples ──────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    lbl(c, font, "paragraph_level() examples:", ix, cy);
    cy += 13.0;

    let pl_hello  = paragraph_level("Hello");
    let pl_arabic = paragraph_level("\u{0645}\u{0631}\u{062D}\u{0628}\u{0627}");

    let pl_examples: &[(&str, u8, Color)] = &[
        ("\"Hello\"",    pl_hello,  ACCENT4),
        ("\"\\u{0645}\\u{0631}...\"", pl_arabic, ACCENT3),
    ];

    for (label, level, color) in pl_examples {
        card_box(c, ix, cy, 310.0, 26.0, CARD_BG, DIVIDER, 1.0);
        c.draw_text(
            &format!("paragraph_level({}) = {}", label, level),
            Point { x: ix + 8.0, y: cy + 7.0 },
            TEXT_PRIMARY, font, 9.0,
        );
        c.fill_rect(
            Rect { origin: Point { x: ix + 278.0, y: cy + 4.0 }, size: Size { width: 22.0, height: 18.0 } },
            *color,
        );
        c.draw_text(
            &level.to_string(),
            Point { x: ix + 286.0, y: cy + 7.0 },
            Color::rgb(10, 12, 20), font, 10.0,
        );
        cy += 28.0;
    }

    // Stats footer
    let footer_y = y + PANEL_H - 28.0;
    lbl(c, font, "40 tests \u{2022} TR#9 L2 reordering", ix, footer_y);
}

// ── Panel 4 — Media Stubs ─────────────────────────────────────────────────────

fn panel_media(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: PANEL_W, height: PANEL_H } }, PANEL_BG);

    let ix = x + 20.0;
    let mut cy = y + 14.0;

    section_label(c, font, "Media Stubs (v1.0)", ix, cy, ACCENT5);
    cy += 24.0;

    // ── AudioPlayer state machine ─────────────────────────────────────────────
    lbl(c, font, "AudioPlayer::load() — always Err:", ix, cy);
    cy += 13.0;

    let mut player = AudioPlayer::new();
    let load_result = player.load("music.mp3");
    let err_label = match &load_result {
        Err(MediaError::PlatformUnavailable) => "PlatformUnavailable",
        Err(MediaError::Unsupported)         => "Unsupported",
        Err(MediaError::NotFound(_))         => "NotFound",
        Err(MediaError::DecodeFailed(_))     => "DecodeFailed",
        Err(MediaError::InvalidData(_))      => "InvalidData",
        Ok(_)                                => "Ok(handle)",
    };

    // Flow: LOAD → PlatformUnavailable → v1.0: rodio/cpal
    let flow_bw  = 88.0_f32;
    let flow_bh  = 32.0_f32;
    let flow_gap = 8.0_f32;

    let boxes: &[(&str, Color)] = &[
        ("LOAD",               ACCENT5),
        ("PlatformUnavailable",ACCENT3),
        ("v1.0: rodio/cpal",   ACCENT2),
    ];

    for (i, (label, color)) in boxes.iter().enumerate() {
        let bx = ix + i as f32 * (flow_bw + flow_gap);
        card_box(c, bx, cy, flow_bw, flow_bh, CARD_BG, *color, 1.5);
        c.draw_text(label, Point { x: bx + 4.0, y: cy + 10.0 }, *color, font, 7.5);
        if i < boxes.len() - 1 {
            let arrow_x = bx + flow_bw;
            let arrow_y = cy + flow_bh / 2.0;
            c.fill_rect(
                Rect { origin: Point { x: arrow_x, y: arrow_y - 1.0 }, size: Size { width: flow_gap - 2.0, height: 2.0 } },
                DIVIDER,
            );
            c.fill_circle(Point { x: arrow_x + flow_gap - 3.0, y: arrow_y }, 2.0, DIVIDER);
        }
    }
    cy += flow_bh + 10.0;

    lbl(c, font, &format!("load(\"music.mp3\") \u{2192} Err({})", err_label), ix, cy);
    cy += 18.0;

    // ── VideoDecoder card ─────────────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    lbl(c, font, "VideoDecoder::open() demo:", ix, cy);
    cy += 13.0;

    let vresult = VideoDecoder::open("video.mp4");
    let verr_label = match &vresult {
        Err(MediaError::PlatformUnavailable) => "PlatformUnavailable",
        Err(_) => "Err(..)",
        Ok(_)  => "Ok(decoder)",
    };

    card_box(c, ix, cy, 310.0, 44.0, CARD_BG, ACCENT5, 1.0);
    c.draw_text(
        "VideoDecoder::open(\"video.mp4\")",
        Point { x: ix + 8.0, y: cy + 6.0 },
        TEXT_MUTED, font, 9.0,
    );
    c.draw_text(
        &format!("\u{2192} Err({})", verr_label),
        Point { x: ix + 8.0, y: cy + 22.0 },
        ACCENT5, font, 10.0,
    );
    c.draw_text(
        "v1.0: platform video decode API",
        Point { x: ix + 8.0, y: cy + 34.0 },
        TEXT_MUTED, font, 8.0,
    );
    cy += 52.0;

    // ── AudioFormat cards ─────────────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    lbl(c, font, "AudioFormat variants:", ix, cy);
    cy += 12.0;

    let audio_formats = [
        AudioFormat::Wav, AudioFormat::Mp3, AudioFormat::Ogg,
        AudioFormat::Aac, AudioFormat::Flac,
    ];
    for fmt in &audio_formats {
        let name = format!("{:?}", fmt);
        let mime = fmt.mime_type();
        card_box(c, ix, cy, 310.0, 22.0, CARD_BG, DIVIDER, 1.0);
        c.fill_rect(Rect { origin: Point { x: ix + 2.0, y: cy + 2.0 }, size: Size { width: 3.0, height: 18.0 } }, ACCENT4);
        c.draw_text(&name, Point { x: ix + 10.0, y: cy + 5.0 }, ACCENT4,    font, 9.0);
        c.draw_text(mime,  Point { x: ix + 68.0, y: cy + 5.0 }, TEXT_MUTED, font, 8.0);
        cy += 24.0;
    }

    cy += 4.0;

    // ── VideoFormat cards ─────────────────────────────────────────────────────
    lbl(c, font, "VideoFormat variants:", ix, cy);
    cy += 12.0;

    let video_formats = [
        VideoFormat::Mp4, VideoFormat::Webm, VideoFormat::Gif, VideoFormat::Avi,
    ];
    for fmt in &video_formats {
        let name = format!("{:?}", fmt);
        let mime = fmt.mime_type();
        let transp = fmt.supports_transparency();
        let transp_color = if transp { ACCENT2 } else { TEXT_MUTED };
        let transp_label = if transp { "\u{2713} alpha" } else { "no alpha" };
        card_box(c, ix, cy, 310.0, 22.0, CARD_BG, DIVIDER, 1.0);
        c.fill_rect(Rect { origin: Point { x: ix + 2.0, y: cy + 2.0 }, size: Size { width: 3.0, height: 18.0 } }, ACCENT5);
        c.draw_text(&name,        Point { x: ix + 10.0,  y: cy + 5.0 }, ACCENT5,      font, 9.0);
        c.draw_text(mime,         Point { x: ix + 68.0,  y: cy + 5.0 }, TEXT_MUTED,   font, 8.0);
        c.draw_text(transp_label, Point { x: ix + 232.0, y: cy + 5.0 }, transp_color, font, 8.0);
        cy += 24.0;
    }

    cy += 4.0;

    // ── VideoFrame info ───────────────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: 310.0, height: 1.0 } }, DIVIDER);
    cy += 10.0;

    let frame = VideoFrame::new(1920, 1080, 0);
    card_box(c, ix, cy, 310.0, 44.0, CARD_BG, DIVIDER, 1.0);
    c.draw_text(
        "VideoFrame::new(1920, 1080, 0)",
        Point { x: ix + 8.0, y: cy + 6.0 },
        TEXT_MUTED, font, 9.0,
    );
    c.draw_text(
        &format!("pixel_count() = {}", frame.pixel_count()),
        Point { x: ix + 8.0, y: cy + 20.0 },
        ACCENT4, font, 9.0,
    );
    c.draw_text(
        &format!("byte_count()  = {} (RGBA×4)", frame.byte_count()),
        Point { x: ix + 8.0, y: cy + 33.0 },
        TEXT_MUTED, font, 8.0,
    );
    cy += 52.0;

    // ── MediaError variants ───────────────────────────────────────────────────
    lbl(c, font, "MediaError variants:", ix, cy);
    cy += 12.0;

    let errors: &[(&str, &str)] = &[
        ("PlatformUnavailable",  "feature not implemented (v1.0)"),
        ("NotFound(String)",     "file or stream not found"),
        ("DecodeFailed(String)", "decode failed with reason"),
        ("Unsupported",          "format not supported"),
        ("InvalidData(String)",  "invalid or corrupt data"),
    ];
    for (variant, desc) in errors {
        card_box(c, ix, cy, 310.0, 26.0, CARD_BG, ACCENT5, 1.0);
        c.draw_text(variant, Point { x: ix + 8.0, y: cy + 5.0  }, ACCENT5,    font, 9.0);
        c.draw_text(desc,    Point { x: ix + 8.0, y: cy + 17.0 }, TEXT_MUTED, font, 7.5);
        cy += 28.0;
    }

    // Stats footer
    let footer_y = y + PANEL_H - 28.0;
    lbl(c, font, "32 tests \u{2022} all stub \u{2014} v1.0: rodio + ffmpeg", ix, footer_y);
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
        "TEZZERA \u{2014} Phase 8 Showcase",
        Point { x: 28.0, y: 28.0 },
        TEXT_PRIMARY, &font, 22.0,
    );
    c.draw_text(
        "Renderer Abstraction  \u{2022}  IME Input  \u{2022}  Unicode Bidi  \u{2022}  Media Stubs",
        Point { x: 28.0, y: 56.0 },
        TEXT_MUTED, &font, 11.0,
    );

    // ── Panels ────────────────────────────────────────────────────────────────
    let panel_y = HEADER_H;
    panel_renderer(&mut c, &font, px(0), panel_y);
    panel_ime      (&mut c, &font, px(1), panel_y);
    panel_bidi     (&mut c, &font, px(2), panel_y);
    panel_media    (&mut c, &font, px(3), panel_y);

    // ── Panel dividers (1px vertical lines) ──────────────────────────────────
    for i in 1..4 {
        let dx = px(i);
        c.fill_rect(
            Rect { origin: Point { x: dx, y: panel_y }, size: Size { width: 1.0, height: PANEL_H } },
            DIVIDER,
        );
    }

    // ── Status bar ────────────────────────────────────────────────────────────
    let sb_y = H as f32 - 14.0;
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: sb_y - 4.0 }, size: Size { width: W as f32, height: 18.0 } },
        Color::rgb(8, 10, 16),
    );
    c.draw_text(
        "TEZZERA  \u{2022}  Phase 8  \u{2022}  Renderer \u{2713}  IME \u{2713}  Bidi \u{2713}  Media \u{2713}",
        Point { x: W as f32 / 2.0 - 250.0, y: sb_y - 1.0 },
        TEXT_MUTED, &font, 10.0,
    );

    // Encode and write
    let png = c.encode_png().expect("png encode failed");
    std::fs::write("phase8_demo.png", &png).expect("write phase8_demo.png");
    println!("Saved phase8_demo.png ({}x{})", W, H);
}
