//! Phase 2 Demo — 1200×900 static PNG showcasing all Phase 2 features.
//!
//! Four panels in a 2×2 grid:
//!   Panel 1 — Theme Gallery     Panel 2 — Widget Showcase
//!   Panel 3 — Animation Lab     Panel 4 — Scroll Preview
//!
//! Run:   cargo run -p tezzera-examples --bin phase2_demo
//! Output: phase2_demo.png (1200×900)

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::{Color, FontCache, SkiaCanvas};
use tezzera_theme::built_in::light_theme;
use tezzera_theme::Color as ThemeColor;
use tezzera_animate::{Easing, Spring};
use tezzera_scroll::{render_scrollbar, ScrollDirection};
use tezzera_widgets::{Button, ButtonVariant, Divider, TextInput};

// ── Canvas-space color constants (u8 0–255) ───────────────────────────────────

const BG:       Color = Color::rgb(12,  12,  20);
const PANEL_BG: Color = Color::rgb(20,  22,  36);
const BORDER:   Color = Color::rgb(44,  48,  72);
const ACCENT:   Color = Color::rgb(103, 80,  164); // #6750A4 — MD3 purple
const TEXT_HI:  Color = Color::rgb(200, 195, 220);
const TEXT_LO:  Color = Color::rgb(120, 115, 145);
const DIVCLR:   Color = Color::rgb(40,  40,  60);

// ── Helper: tezzera_theme::Color (f32 0–1) → tezzera_render::Color (u8 0–255) ─

fn tc(c: ThemeColor) -> Color {
    Color::rgba(
        (c.r * 255.0) as u8,
        (c.g * 255.0) as u8,
        (c.b * 255.0) as u8,
        (c.a * 255.0) as u8,
    )
}

// ── Helper: common panel chrome ───────────────────────────────────────────────

fn panel_bg(
    c: &mut SkiaCanvas,
    x: f32, y: f32, w: f32, h: f32,
    title: &str,
    font: &FontCache,
) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, PANEL_BG);
    c.stroke_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, BORDER, 1.0);
    // Accent top border
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: 3.0 } }, ACCENT);
    c.draw_text(title, Point { x: x + 16.0, y: y + 12.0 }, TEXT_HI, font, 13.0);
}

// ── Panel 1: Theme Gallery ────────────────────────────────────────────────────

fn panel_theme_gallery(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32) {
    panel_bg(c, x, y, w, h, "Theme Gallery", font);

    let theme = light_theme();

    // ── Color swatches: 8 swatches, 4 per row ──
    let swatches: &[(&str, ThemeColor)] = &[
        ("primary",    theme.colors.primary),
        ("on_primary", theme.colors.on_primary),
        ("secondary",  theme.colors.secondary),
        ("surface",    theme.colors.surface),
        ("background", theme.colors.background),
        ("error",      theme.colors.error),
        ("outline",    theme.colors.outline),
        ("shadow",     theme.colors.shadow),
    ];

    let sw_w    = 60.0_f32;
    let sw_h    = 38.0_f32;
    let sw_gap  = 10.0_f32;
    let sw_x0   = x + 16.0;
    let sw_y0   = y + 36.0;

    for (i, (name, color)) in swatches.iter().enumerate() {
        let col = (i % 4) as f32;
        let row = (i / 4) as f32;
        let sx = sw_x0 + col * (sw_w + sw_gap);
        let sy = sw_y0 + row * (sw_h + 20.0);

        c.fill_rect(
            Rect { origin: Point { x: sx, y: sy }, size: Size { width: sw_w, height: sw_h } },
            tc(*color),
        );
        c.stroke_rect(
            Rect { origin: Point { x: sx, y: sy }, size: Size { width: sw_w, height: sw_h } },
            BORDER, 1.0,
        );
        c.draw_text(name, Point { x: sx, y: sy + sw_h + 4.0 }, TEXT_LO, font, 8.0);
    }

    // ── Typography scale ──
    let typo_x  = x + 16.0;
    let typo_y0 = sw_y0 + 2.0 * (sw_h + 20.0) + 12.0;
    let typo_rows: &[(&str, f32)] = &[
        ("Display Large — 57px", 15.0),
        ("Headline — 32px",      13.0),
        ("Title — 22px",         12.0),
        ("Body — 16px",          11.0),
        ("Label — 14px",         10.0),
    ];
    c.draw_text("Typography Scale", Point { x: typo_x, y: typo_y0 }, TEXT_LO, font, 9.0);
    for (i, (label, fs)) in typo_rows.iter().enumerate() {
        c.draw_text(
            label,
            Point { x: typo_x, y: typo_y0 + 14.0 + i as f32 * 20.0 },
            TEXT_HI, font, *fs,
        );
    }

    // ── Spacing tokens — horizontal bars ──
    let sp     = &theme.spacing;
    let bar_x0 = x + 16.0;
    let bar_y0 = typo_y0 + 14.0 + typo_rows.len() as f32 * 20.0 + 10.0;
    let spacing_vals: &[(&str, f32)] = &[
        ("xs",  sp.xs),
        ("sm",  sp.sm),
        ("md",  sp.md),
        ("lg",  sp.lg),
        ("xl",  sp.xl),
        ("xxl", sp.xxl),
    ];
    c.draw_text("Spacing Tokens", Point { x: bar_x0, y: bar_y0 }, TEXT_LO, font, 9.0);
    for (i, (name, val)) in spacing_vals.iter().enumerate() {
        let by  = bar_y0 + 14.0 + i as f32 * 12.0;
        let bw  = (val * 4.0).min(w - 80.0).max(2.0);
        c.fill_rect(
            Rect { origin: Point { x: bar_x0, y: by }, size: Size { width: bw, height: 8.0 } },
            Color::rgba(103, 80, 164, 180),
        );
        c.draw_text(
            &format!("{} ({:.0}px)", name, val),
            Point { x: bar_x0 + bw + 6.0, y: by },
            TEXT_LO, font, 8.0,
        );
    }
}

// ── Panel 2: Widget Showcase ──────────────────────────────────────────────────

fn panel_widgets(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32) {
    panel_bg(c, x, y, w, h, "Widget Showcase", font);

    let ix = x + 16.0;
    let mut cur_y = y + 36.0;

    // ── Button row ──
    c.draw_text("Buttons", Point { x: ix, y: cur_y }, TEXT_LO, font, 9.0);
    cur_y += 14.0;

    let btns: &[(&str, ButtonVariant)] = &[
        ("Primary",   ButtonVariant::Primary),
        ("Secondary", ButtonVariant::Secondary),
        ("Danger",    ButtonVariant::Danger),
        ("Ghost",     ButtonVariant::Ghost),
    ];
    let btn_w   = 108.0_f32;
    let btn_h   = 32.0_f32;
    let btn_gap = 10.0_f32;
    for (i, (label, variant)) in btns.iter().enumerate() {
        Button::new(*label)
            .variant(*variant)
            .width(btn_w)
            .height(btn_h)
            .render(c, font, ix + i as f32 * (btn_w + btn_gap), cur_y);
    }
    cur_y += btn_h + 16.0;

    // ── TextInput row ──
    c.draw_text("Text Inputs", Point { x: ix, y: cur_y }, TEXT_LO, font, 9.0);
    cur_y += 14.0;

    let ti_w = (w - 48.0) / 2.0;
    c.draw_text("Normal", Point { x: ix, y: cur_y }, TEXT_HI, font, 10.0);
    TextInput::new()
        .value("example text")
        .width(ti_w)
        .render(c, font, ix, cur_y + 14.0);

    let ti2_x = ix + ti_w + 16.0;
    c.draw_text("Focused", Point { x: ti2_x, y: cur_y }, TEXT_HI, font, 10.0);
    TextInput::new()
        .placeholder("enter value\u{2026}")
        .focused(true)
        .width(ti_w)
        .render(c, font, ti2_x, cur_y + 14.0);
    cur_y += 14.0 + 36.0 + 14.0;

    // ── Divider ──
    Divider::horizontal(w - 32.0).render(c, ix, cur_y);
    cur_y += 12.0;

    // ── Text samples ──
    c.draw_text("Heading text", Point { x: ix, y: cur_y }, TEXT_HI, font, 18.0);
    cur_y += 26.0;
    c.draw_text(
        "Body text \u{2014} the quick brown fox jumps over the lazy dog",
        Point { x: ix, y: cur_y },
        TEXT_HI, font, 14.0,
    );
    cur_y += 22.0;
    c.draw_text(
        "Caption \u{2022} small text example",
        Point { x: ix, y: cur_y },
        TEXT_LO, font, 11.0,
    );
    cur_y += 22.0;

    // ── Padding demo — nested rects ──
    c.draw_text("Padding Demo", Point { x: ix, y: cur_y }, TEXT_LO, font, 9.0);
    cur_y += 14.0;
    let pd_w  = 220.0_f32;
    let pd_h  = 70.0_f32;
    let pad   = 16.0_f32;
    // Outer
    c.fill_rect(
        Rect { origin: Point { x: ix, y: cur_y }, size: Size { width: pd_w, height: pd_h } },
        Color::rgba(103, 80, 164, 40),
    );
    c.stroke_rect(
        Rect { origin: Point { x: ix, y: cur_y }, size: Size { width: pd_w, height: pd_h } },
        Color::rgba(103, 80, 164, 160), 1.0,
    );
    // Inner
    c.fill_rect(
        Rect {
            origin: Point { x: ix + pad, y: cur_y + pad },
            size: Size { width: pd_w - pad * 2.0, height: pd_h - pad * 2.0 },
        },
        Color::rgba(103, 80, 164, 110),
    );
    c.draw_text(
        "EdgeInsets: 16px",
        Point { x: ix + pad + 4.0, y: cur_y + pd_h / 2.0 - 5.0 },
        TEXT_HI, font, 10.0,
    );
    // Inset label arrows
    c.draw_text(
        "\u{2190} 16px \u{2192}",
        Point { x: ix + 2.0, y: cur_y + pd_h + 3.0 },
        TEXT_LO, font, 8.0,
    );
}

// ── Panel 3: Animation Lab ────────────────────────────────────────────────────

fn panel_animation_lab(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32) {
    panel_bg(c, x, y, w, h, "Animation Lab", font);

    let ix = x + 16.0;
    let mut cur_y = y + 36.0;

    // ── Easing curves ──
    c.draw_text("Easing Curves (t=0\u{2192}1 sampled at 40 points)", Point { x: ix, y: cur_y }, TEXT_LO, font, 9.0);
    cur_y += 14.0;

    let curves: &[(&str, Easing, Color)] = &[
        ("Linear",    Easing::Linear,       Color::rgb(100, 160, 255)),
        ("EaseIn",    Easing::EaseIn,        Color::rgb(72,  199, 116)),
        ("EaseOut",   Easing::EaseOut,       Color::rgb(255, 165,  55)),
        ("EaseInOut", Easing::EaseInOut,     Color::rgb(180, 100, 255)),
        ("OutBounce", Easing::EaseOutBounce, Color::rgb(255,  90,  80)),
    ];
    let plot_w   = ((w - 32.0) / curves.len() as f32 - 6.0).max(60.0);
    let plot_h   = 60.0_f32;
    let plot_gap = 6.0_f32;

    for (i, (name, easing, color)) in curves.iter().enumerate() {
        let px = ix + i as f32 * (plot_w + plot_gap);
        // Plot background
        c.fill_rect(
            Rect { origin: Point { x: px, y: cur_y }, size: Size { width: plot_w, height: plot_h } },
            Color::rgba(28, 30, 50, 220),
        );
        c.stroke_rect(
            Rect { origin: Point { x: px, y: cur_y }, size: Size { width: plot_w, height: plot_h } },
            BORDER, 1.0,
        );
        // Curve dots
        for j in 0..40usize {
            let t = j as f32 / 39.0;
            let v = easing.eval(t);
            let dot_x = px + t * plot_w;
            let dot_y = cur_y + plot_h - v.clamp(0.0, 1.0) * plot_h;
            c.fill_rect(
                Rect { origin: Point { x: dot_x, y: dot_y }, size: Size { width: 2.0, height: 2.0 } },
                *color,
            );
        }
        c.draw_text(name, Point { x: px, y: cur_y + plot_h + 4.0 }, TEXT_LO, font, 8.0);
    }
    cur_y += plot_h + 22.0;

    // ── Spring presets ──
    c.draw_text("Spring Presets (60 frames, dt=1/60s)", Point { x: ix, y: cur_y }, TEXT_LO, font, 9.0);
    cur_y += 14.0;

    struct Preset {
        name:      &'static str,
        stiffness: f32,
        damping:   f32,
        color:     Color,
    }
    let presets = [
        Preset { name: "Gentle",  stiffness: 100.0, damping: 15.0, color: Color::rgb(100, 160, 255) },
        Preset { name: "Bouncy",  stiffness: 300.0, damping:  8.0, color: Color::rgb(180, 100, 255) },
        Preset { name: "Stiff",   stiffness: 500.0, damping: 40.0, color: Color::rgb(72,  199, 116) },
        Preset { name: "Slow",    stiffness:  50.0, damping: 10.0, color: Color::rgb(255, 165,  55) },
    ];
    let chart_n  = presets.len() as f32;
    let chart_w  = ((w - 32.0 - (chart_n - 1.0) * 10.0) / chart_n).max(60.0);
    let chart_h  = 65.0_f32;
    let nframes  = 60usize;

    for (pi, preset) in presets.iter().enumerate() {
        let cx = ix + pi as f32 * (chart_w + 10.0);
        c.fill_rect(
            Rect { origin: Point { x: cx, y: cur_y }, size: Size { width: chart_w, height: chart_h } },
            Color::rgba(28, 30, 50, 220),
        );
        c.stroke_rect(
            Rect { origin: Point { x: cx, y: cur_y }, size: Size { width: chart_w, height: chart_h } },
            BORDER, 1.0,
        );
        // Baseline (target = 1.0, displayed at top 10% of chart)
        let baseline_y = cur_y + chart_h * 0.9;
        c.fill_rect(
            Rect { origin: Point { x: cx, y: baseline_y }, size: Size { width: chart_w, height: 1.0 } },
            Color::rgba(80, 85, 110, 150),
        );

        let mut spring = Spring::new(0.0, 1.0)
            .stiffness(preset.stiffness)
            .damping(preset.damping);
        let bar_w = chart_w / nframes as f32;

        for frame in 0..nframes {
            let pos = spring.update(1.0 / 60.0);
            // pos in [0,1] approx; map to chart space (baseline at 90% from top)
            let bar_h = (pos * chart_h * 0.85).clamp(-chart_h * 0.85, chart_h * 0.85);
            let bx = cx + frame as f32 * bar_w;
            if bar_h >= 0.0 {
                let by = baseline_y - bar_h;
                c.fill_rect(
                    Rect { origin: Point { x: bx, y: by }, size: Size { width: bar_w.max(1.0), height: bar_h } },
                    Color::rgba(preset.color.r, preset.color.g, preset.color.b, 180),
                );
            } else {
                // Overshoot below baseline — draw downward bar in muted color
                let bh = (-bar_h).min(chart_h * 0.09);
                c.fill_rect(
                    Rect { origin: Point { x: bx, y: baseline_y }, size: Size { width: bar_w.max(1.0), height: bh } },
                    Color::rgba(preset.color.r, preset.color.g, preset.color.b, 80),
                );
            }
        }
        c.draw_text(preset.name, Point { x: cx, y: cur_y + chart_h + 4.0 }, TEXT_LO, font, 8.0);
    }
    cur_y += chart_h + 24.0;

    // ── Easing value table ──
    c.draw_text(
        "EaseInOut spot values   t=0.00..1.00 \u{2192} v",
        Point { x: ix, y: cur_y },
        TEXT_LO, font, 9.0,
    );
    cur_y += 14.0;
    let easing = Easing::EaseInOut;
    let samples = [0.00_f32, 0.25, 0.50, 0.75, 1.00];
    for (i, &t) in samples.iter().enumerate() {
        let v = easing.eval(t);
        c.draw_text(
            &format!("{:.2}\u{2192}{:.3}", t, v),
            Point { x: ix + i as f32 * 96.0, y: cur_y },
            TEXT_HI, font, 10.0,
        );
    }
}

// ── Panel 4: Scroll Preview ───────────────────────────────────────────────────

fn panel_scroll_preview(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32) {
    panel_bg(c, x, y, w, h, "Scroll View", font);

    let ix = x + 16.0;
    let iy = y + 36.0;

    let viewport_w    = w - 32.0;
    let viewport_h    = 260.0_f32;
    let content_total = 480.0_f32; // 12 items × 40 px
    let scroll_offset = 80.0_f32;  // items 3–9 visible
    let item_h        = 40.0_f32;
    let item_count    = 12usize;

    // Viewport clip border
    c.fill_rect(
        Rect { origin: Point { x: ix, y: iy }, size: Size { width: viewport_w, height: viewport_h } },
        Color::rgb(16, 18, 30),
    );
    c.stroke_rect(
        Rect { origin: Point { x: ix, y: iy }, size: Size { width: viewport_w, height: viewport_h } },
        BORDER, 1.0,
    );

    let dot_palette = [
        Color::rgb(100, 160, 255),
        Color::rgb(72,  199, 116),
        Color::rgb(255, 165,  55),
        Color::rgb(180, 100, 255),
        Color::rgb(255,  90,  80),
        Color::rgb(100, 200, 200),
    ];

    for i in 0..item_count {
        let item_top_in_content = i as f32 * item_h;
        let screen_y = iy + item_top_in_content - scroll_offset;

        // Only draw if at least partially visible
        if screen_y + item_h < iy || screen_y > iy + viewport_h {
            continue;
        }

        // Alternating row bg (clipped to viewport)
        if i % 2 == 0 {
            let clip_y  = screen_y.max(iy);
            let clip_h  = ((screen_y + item_h).min(iy + viewport_h) - clip_y).max(0.0);
            c.fill_rect(
                Rect {
                    origin: Point { x: ix + 1.0, y: clip_y },
                    size: Size { width: viewport_w - 2.0, height: clip_h },
                },
                Color::rgba(255, 255, 255, 8),
            );
        }

        // Dot + label (only if center is inside viewport)
        let dot_cy = screen_y + item_h / 2.0;
        if dot_cy >= iy && dot_cy <= iy + viewport_h {
            let dot_color = dot_palette[i % dot_palette.len()];
            c.fill_circle(Point { x: ix + 20.0, y: dot_cy }, 5.0, dot_color);
            c.draw_text(
                &format!("Item {} \u{2014} example content", i + 1),
                Point { x: ix + 34.0, y: dot_cy - 6.0 },
                TEXT_HI, font, 11.0,
            );
        }
    }

    // Scrollbar overlay
    render_scrollbar(
        c,
        ScrollDirection::Vertical,
        ix, iy, viewport_w, viewport_h,
        scroll_offset, content_total, 1.0,
    );

    // ── Stats ──
    let stats_y = iy + viewport_h + 12.0;
    c.draw_text(
        &format!(
            "Offset: {:.0}px / {:.0}px max     Content: {:.0}px     Viewport: {:.0}px",
            scroll_offset,
            content_total - viewport_h,
            content_total,
            viewport_h,
        ),
        Point { x: ix, y: stats_y },
        TEXT_LO, font, 9.0,
    );
    c.draw_text(
        "ScrollDirection: Vertical     Physics: Momentum (friction = 0.92)",
        Point { x: ix, y: stats_y + 13.0 },
        TEXT_LO, font, 9.0,
    );

    // ── Momentum velocity decay chart ──
    let decay_label_y = stats_y + 32.0;
    c.draw_text(
        "Velocity decay  v[n] = v[n-1] \u{00D7} 0.92   (initial = 100 px/frame)",
        Point { x: ix, y: decay_label_y },
        TEXT_LO, font, 9.0,
    );
    let decay_y = decay_label_y + 14.0;
    let decay_h = 34.0_f32;
    let nframes = 50usize;
    let bar_w   = (viewport_w - 16.0) / nframes as f32;
    let mut vel = 100.0_f32;

    // Decay chart background
    c.fill_rect(
        Rect { origin: Point { x: ix, y: decay_y }, size: Size { width: viewport_w - 16.0, height: decay_h } },
        Color::rgba(28, 30, 50, 200),
    );

    for frame in 0..nframes {
        vel *= 0.92;
        let bh = ((vel / 100.0) * decay_h).max(1.0);
        c.fill_rect(
            Rect {
                origin: Point { x: ix + frame as f32 * bar_w, y: decay_y + decay_h - bh },
                size: Size { width: bar_w.max(1.0), height: bh },
            },
            Color::rgba(100, 160, 255, 180),
        );
    }
    c.stroke_rect(
        Rect { origin: Point { x: ix, y: decay_y }, size: Size { width: viewport_w - 16.0, height: decay_h } },
        BORDER, 1.0,
    );
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    let font = FontCache::system_mono().expect("system font not found");
    let mut c = SkiaCanvas::new(1200, 900);

    // ── Global background ──
    c.clear(BG);

    // ── Header bar ──
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: 0.0 }, size: Size { width: 1200.0, height: 60.0 } },
        Color::rgb(22, 22, 36),
    );
    c.draw_text(
        "TEZZERA  Phase 2 Demo",
        Point { x: 24.0, y: 20.0 },
        TEXT_HI, &font, 16.0,
    );
    c.draw_text(
        "Theme \u{00B7} Widgets \u{00B7} Animation \u{00B7} Scroll",
        Point { x: 830.0, y: 22.0 },
        TEXT_LO, &font, 12.0,
    );
    // Accent underline
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: 57.0 }, size: Size { width: 1200.0, height: 3.0 } },
        ACCENT,
    );

    // ── Grid dividers ──
    c.fill_rect(
        Rect { origin: Point { x: 598.0, y: 60.0 }, size: Size { width: 4.0, height: 800.0 } },
        DIVCLR,
    );
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: 460.0 }, size: Size { width: 1200.0, height: 4.0 } },
        DIVCLR,
    );

    // Panel geometry: header=60, gap=3, panels fill to status bar at 860
    //   Row 1: y=63,  h=394  (ends at 457, then 3px gap → 460)
    //   Row 2: y=464, h=396  (ends at 860)
    let pw = 595.0_f32;
    let r1y = 63.0_f32;
    let r1h = 394.0_f32;
    let r2y = 464.0_f32;
    let r2h = 396.0_f32;

    panel_theme_gallery  (&mut c, &font,   3.0, r1y, pw, r1h);
    panel_widgets        (&mut c, &font, 602.0, r1y, pw, r1h);
    panel_animation_lab  (&mut c, &font,   3.0, r2y, pw, r2h);
    panel_scroll_preview (&mut c, &font, 602.0, r2y, pw, r2h);

    // ── Status bar ──
    let sb_y = 860.0_f32;
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: sb_y }, size: Size { width: 1200.0, height: 40.0 } },
        Color::rgb(18, 18, 30),
    );
    c.draw_text(
        "Phase 2 Complete  \u{00B7}  tezzera-theme  tezzera-animate  tezzera-scroll  tezzera-widgets",
        Point { x: 24.0, y: sb_y + 12.0 },
        TEXT_LO, &font, 11.0,
    );

    // ── Encode and write ──
    let png = c.encode_png().expect("png encode failed");
    std::fs::write("phase2_demo.png", &png).expect("write phase2_demo.png");
    println!("Saved phase2_demo.png (1200\u{00D7}900)");
}
