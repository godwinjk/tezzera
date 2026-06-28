//! Phase 5 Demo — 1400×900 static PNG showcasing Phase 5 systems.
//!
//! Four panels (340×880 each, x = 10, 360, 710, 1060):
//!   Panel 1 — Image Rendering   (ImageWidget, ImageFit, ImageCache — D033)
//!   Panel 2 — Overlay System    (Modal, Dialog, Toast, ToastQueue)
//!   Panel 3 — Screen Transitions (tezzera-nav-anim, TransitionStyle)
//!   Panel 4 — Templates & A11y  (tzr new --template, accessibility tree)
//!
//! Run:    cargo run -p tezzera-examples --bin phase5_demo
//! Output: phase5_demo.png (1400×900)

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::{Color, FontCache, SkiaCanvas};

// Acknowledge nav-anim dep exists (types used in code snippet labels below).
use tezzera_nav_anim::{SlideDirection, TransitionStyle};

// ── Canvas dimensions ─────────────────────────────────────────────────────────
const W: u32 = 1400;
const H: u32 = 900;

// ── Color palette ─────────────────────────────────────────────────────────────
const BG:      Color = Color::rgb( 10,  11,  18);
const PANEL:   Color = Color::rgb( 18,  20,  30);
const ACCENT:  Color = Color::rgb(103,  80, 164);
const ACCENT2: Color = Color::rgb( 72, 199, 116);
const ACCENT3: Color = Color::rgb(100, 160, 255);
const TEXT:    Color = Color::rgb(230, 225, 229);
const MUTED:   Color = Color::rgb(120, 125, 155);
const ERROR:   Color = Color::rgb(207, 102, 121);
const SUCCESS: Color = Color::rgb( 72, 199, 116);
const SURFACE: Color = Color::rgb( 28,  30,  46);
const BORDER:  Color = Color::rgb( 45,  48,  70);

// ── Panel layout ──────────────────────────────────────────────────────────────
// 4 panels at x=10,360,710,1060; each 340×880, y=10
const PANEL_W: f32 = 340.0;
const PANEL_H: f32 = 880.0;
const PANEL_Y: f32 = 10.0;

fn panel_x(n: usize) -> f32 {
    10.0 + n as f32 * 350.0
}

// ── Shared helpers ────────────────────────────────────────────────────────────

fn panel_bg(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32, title: &str) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, PANEL);
    c.stroke_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, BORDER, 1.0);
    // 4px accent top border
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: 4.0 } }, ACCENT);
    c.draw_text(title, Point { x: x + 14.0, y: y + 12.0 }, TEXT, font, 13.0);
}

fn lbl(c: &mut SkiaCanvas, font: &FontCache, text: &str, x: f32, y: f32) {
    c.draw_text(text, Point { x, y }, MUTED, font, 9.0);
}

// ── Panel 1 — Image Rendering ─────────────────────────────────────────────────

fn panel_image_rendering(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32) {
    panel_bg(c, font, x, y, w, h, "Image Rendering (D033)");

    let ix = x + 14.0;
    let mut cy = y + 32.0;

    let img_w = 200.0_f32;
    let img_h = 110.0_f32;
    // center images horizontally in panel
    let img_x = ix + (w - 28.0 - img_w) / 2.0;

    // ── 1. Placeholder image ──────────────────────────────────────────────────
    {
        let bx = img_x;
        let by = cy;
        // Dark purple background
        c.fill_rect(
            Rect { origin: Point { x: bx, y: by }, size: Size { width: img_w, height: img_h } },
            Color::rgb(38, 32, 62),
        );
        c.stroke_rect(
            Rect { origin: Point { x: bx, y: by }, size: Size { width: img_w, height: img_h } },
            BORDER, 1.0,
        );
        // Mountain triangle via strips (widest at bottom, narrowing toward top)
        for i in 0..10u32 {
            let strip_w = (10 - i) as f32 * 10.0;
            let strip_x = bx + img_w / 2.0 - strip_w / 2.0;
            let strip_y = by + img_h - 28.0 - i as f32 * 6.5;
            let v = (60 + i * 14) as u8;
            c.fill_rect(
                Rect { origin: Point { x: strip_x, y: strip_y }, size: Size { width: strip_w, height: 7.0 } },
                Color::rgb(v, (v / 3) as u8, v.saturating_add(30)),
            );
        }
        // Filename in bottom corner
        c.draw_text(
            "photo.png \u{2197}",
            Point { x: bx + 4.0, y: by + img_h - 13.0 },
            Color::rgb(155, 150, 190), font, 9.0,
        );
        cy += img_h + 4.0;
        lbl(c, font, "Placeholder (no file)", ix, cy);
        cy += 18.0;
    }

    // ── 2. Contain fit ────────────────────────────────────────────────────────
    {
        let bx = img_x;
        let by = cy;
        // Outer letterbox border
        c.stroke_rect(
            Rect { origin: Point { x: bx, y: by }, size: Size { width: img_w, height: img_h } },
            MUTED, 1.0,
        );
        // Inner image area centered
        let inner_w = 155.0_f32;
        let inner_h = 82.0_f32;
        let inner_x = bx + (img_w - inner_w) / 2.0;
        let inner_y = by + (img_h - inner_h) / 2.0;
        c.fill_rect(
            Rect { origin: Point { x: inner_x, y: inner_y }, size: Size { width: inner_w, height: inner_h } },
            Color::rgba(ACCENT.r, ACCENT.g, ACCENT.b, 80),
        );
        c.stroke_rect(
            Rect { origin: Point { x: inner_x, y: inner_y }, size: Size { width: inner_w, height: inner_h } },
            ACCENT, 1.5,
        );
        c.draw_text("Contain", Point { x: inner_x + 44.0, y: inner_y + 33.0 }, TEXT, font, 10.0);
        cy += img_h + 4.0;
        lbl(c, font, "ImageFit::Contain", ix, cy);
        cy += 18.0;
    }

    // ── 3. Cover fit ──────────────────────────────────────────────────────────
    {
        let bx = img_x;
        let by = cy;
        c.fill_rect(
            Rect { origin: Point { x: bx, y: by }, size: Size { width: img_w, height: img_h } },
            Color::rgba(ACCENT.r, ACCENT.g, ACCENT.b, 105),
        );
        c.stroke_rect(
            Rect { origin: Point { x: bx, y: by }, size: Size { width: img_w, height: img_h } },
            BORDER, 1.0,
        );
        // Dashed crop indicator
        let margin = 9.0_f32;
        let dash = 8.0_f32;
        let gap = 5.0_f32;
        let period = dash + gap;
        // Top edge dashes
        let mut dx = bx + margin;
        while dx < bx + img_w - margin {
            let seg = dash.min(bx + img_w - margin - dx);
            c.fill_rect(Rect { origin: Point { x: dx, y: by + margin }, size: Size { width: seg, height: 1.5 } }, Color::WHITE);
            dx += period;
        }
        // Bottom edge dashes
        let mut dx = bx + margin;
        while dx < bx + img_w - margin {
            let seg = dash.min(bx + img_w - margin - dx);
            c.fill_rect(Rect { origin: Point { x: dx, y: by + img_h - margin }, size: Size { width: seg, height: 1.5 } }, Color::WHITE);
            dx += period;
        }
        c.draw_text("Cover", Point { x: bx + 80.0, y: by + 48.0 }, TEXT, font, 10.0);
        cy += img_h + 4.0;
        lbl(c, font, "ImageFit::Cover", ix, cy);
        cy += 18.0;
    }

    // ── 4. Fill fit ───────────────────────────────────────────────────────────
    {
        let bx = img_x;
        let by = cy;
        // 5 horizontal gradient bands purple → blue
        let bands: [Color; 5] = [
            Color::rgb(103, 80, 164),
            Color::rgb(95, 100, 175),
            Color::rgb(85, 120, 192),
            Color::rgb(92, 140, 210),
            Color::rgb(100, 160, 230),
        ];
        let bh = img_h / bands.len() as f32;
        for (i, col) in bands.iter().enumerate() {
            c.fill_rect(
                Rect { origin: Point { x: bx, y: by + i as f32 * bh }, size: Size { width: img_w, height: bh + 1.0 } },
                *col,
            );
        }
        c.stroke_rect(Rect { origin: Point { x: bx, y: by }, size: Size { width: img_w, height: img_h } }, BORDER, 1.0);
        c.draw_text("Fill (stretched)", Point { x: bx + 46.0, y: by + 48.0 }, TEXT, font, 10.0);
        cy += img_h + 4.0;
        lbl(c, font, "ImageFit::Fill", ix, cy);
        cy += 18.0;
    }

    // ── 5. ImageCache deduplication diagram ───────────────────────────────────
    {
        lbl(c, font, "ImageCache deduplication", ix, cy);
        cy += 14.0;

        let files = ["img1.png", "img2.png", "img3.png"];
        let file_x = ix + 4.0;
        let cache_x = ix + 160.0;
        let cache_y = cy + 4.0;
        let cache_w = 160.0_f32;
        let cache_h = 52.0_f32;

        // Cache box
        c.fill_rect(Rect { origin: Point { x: cache_x, y: cache_y }, size: Size { width: cache_w, height: cache_h } }, SURFACE);
        c.stroke_rect(Rect { origin: Point { x: cache_x, y: cache_y }, size: Size { width: cache_w, height: cache_h } }, ACCENT2, 1.5);
        c.draw_text("HashMap:", Point { x: cache_x + 8.0, y: cache_y + 6.0 }, ACCENT2, font, 9.0);
        c.draw_text("3 entries", Point { x: cache_x + 8.0, y: cache_y + 20.0 }, TEXT, font, 10.0);
        c.draw_text("dedup \u{2713}", Point { x: cache_x + 8.0, y: cache_y + 35.0 }, MUTED, font, 9.0);

        // File icons with arrows
        for (i, name) in files.iter().enumerate() {
            let fy = cy + i as f32 * 18.0 + 8.0;
            c.fill_rect(Rect { origin: Point { x: file_x, y: fy }, size: Size { width: 12.0, height: 14.0 } }, SURFACE);
            c.stroke_rect(Rect { origin: Point { x: file_x, y: fy }, size: Size { width: 12.0, height: 14.0 } }, BORDER, 1.0);
            c.draw_text(name, Point { x: file_x + 16.0, y: fy + 2.0 }, TEXT, font, 8.0);
            // Arrow line to cache box
            let ax = file_x + 90.0;
            let ay = fy + 7.0;
            c.fill_rect(
                Rect { origin: Point { x: ax, y: ay - 1.0 }, size: Size { width: cache_x - ax - 4.0, height: 1.5 } },
                MUTED,
            );
            // Arrowhead
            c.fill_rect(Rect { origin: Point { x: cache_x - 7.0, y: ay - 4.0 }, size: Size { width: 2.0, height: 8.0 } }, MUTED);
        }
    }
}

// ── Panel 2 — Overlay System ──────────────────────────────────────────────────

fn panel_overlay(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32) {
    panel_bg(c, font, x, y, w, h, "Overlays");

    let ix = x + 14.0;
    let mut cy = y + 32.0;
    let section_w = w - 28.0;

    // ── 1. Modal ─────────────────────────────────────────────────────────────
    {
        lbl(c, font, "Modal \u{2014} backdrop + centered box", ix, cy);
        cy += 13.0;

        let area_h = 90.0_f32;
        // Dim overlay
        c.fill_rect(
            Rect { origin: Point { x: ix, y: cy }, size: Size { width: section_w, height: area_h } },
            Color::rgba(0, 0, 0, 120),
        );
        // Centered modal box
        let box_w = 240.0_f32;
        let box_h = 58.0_f32;
        let box_x = ix + (section_w - box_w) / 2.0;
        let box_y = cy + (area_h - box_h) / 2.0;
        c.fill_rect(Rect { origin: Point { x: box_x, y: box_y }, size: Size { width: box_w, height: box_h } }, SURFACE);
        c.stroke_rect(Rect { origin: Point { x: box_x, y: box_y }, size: Size { width: box_w, height: box_h } }, BORDER, 1.5);
        c.draw_text("Modal", Point { x: box_x + 14.0, y: box_y + 8.0 }, ACCENT, font, 12.0);
        c.draw_text("Content area", Point { x: box_x + 14.0, y: box_y + 28.0 }, MUTED, font, 10.0);

        cy += area_h + 14.0;
    }

    // ── 2. Dialog ────────────────────────────────────────────────────────────
    {
        lbl(c, font, "Dialog \u{2014} title + message + buttons", ix, cy);
        cy += 13.0;

        let dlg_w = 280.0_f32;
        let dlg_h = 108.0_f32;
        let dlg_x = ix + (section_w - dlg_w) / 2.0;
        let dlg_y = cy;

        c.fill_rect(Rect { origin: Point { x: dlg_x, y: dlg_y }, size: Size { width: dlg_w, height: dlg_h } }, SURFACE);
        c.stroke_rect(Rect { origin: Point { x: dlg_x, y: dlg_y }, size: Size { width: dlg_w, height: dlg_h } }, BORDER, 1.5);

        // Title
        c.draw_text("Confirm Delete", Point { x: dlg_x + 14.0, y: dlg_y + 10.0 }, TEXT, font, 13.0);
        // Divider
        c.fill_rect(
            Rect { origin: Point { x: dlg_x + 8.0, y: dlg_y + 30.0 }, size: Size { width: dlg_w - 16.0, height: 1.0 } },
            BORDER,
        );
        // Message
        c.draw_text(
            "This action cannot be undone.",
            Point { x: dlg_x + 14.0, y: dlg_y + 40.0 },
            MUTED, font, 10.0,
        );

        // Buttons
        let btn_y = dlg_y + dlg_h - 32.0;
        let btn_h = 24.0_f32;
        // Cancel (outlined)
        let cancel_w = 72.0_f32;
        let cancel_x = dlg_x + dlg_w - 12.0 - cancel_w - 82.0;
        c.stroke_rect(Rect { origin: Point { x: cancel_x, y: btn_y }, size: Size { width: cancel_w, height: btn_h } }, BORDER, 1.0);
        c.draw_text("Cancel", Point { x: cancel_x + 10.0, y: btn_y + 6.0 }, TEXT, font, 10.0);
        // Delete (filled ERROR)
        let del_w = 72.0_f32;
        let del_x = dlg_x + dlg_w - 12.0 - del_w;
        c.fill_rect(Rect { origin: Point { x: del_x, y: btn_y }, size: Size { width: del_w, height: btn_h } }, ERROR);
        c.draw_text("Delete", Point { x: del_x + 12.0, y: btn_y + 6.0 }, Color::WHITE, font, 10.0);

        cy += dlg_h + 14.0;
    }

    // ── 3. Toast stack ────────────────────────────────────────────────────────
    {
        lbl(c, font, "ToastQueue \u{2014} auto-dismiss stack", ix, cy);
        cy += 13.0;

        let toast_w = section_w;
        let toast_h = 30.0_f32;
        let toast_gap = 5.0_f32;
        let accent_bar = 4.0_f32;

        let toasts: &[(&str, Color, u8)] = &[
            ("File saved successfully", SUCCESS, 240),
            ("Network error occurred",  ERROR,   160),
            ("Upload complete",         ACCENT,   60),
        ];

        for (i, (msg, bar_color, alpha)) in toasts.iter().enumerate() {
            let ty = cy + i as f32 * (toast_h + toast_gap);
            c.fill_rect(
                Rect { origin: Point { x: ix, y: ty }, size: Size { width: toast_w, height: toast_h } },
                Color::rgba(SURFACE.r, SURFACE.g, SURFACE.b, *alpha),
            );
            c.stroke_rect(
                Rect { origin: Point { x: ix, y: ty }, size: Size { width: toast_w, height: toast_h } },
                Color::rgba(BORDER.r, BORDER.g, BORDER.b, *alpha), 1.0,
            );
            // Left accent bar
            c.fill_rect(
                Rect { origin: Point { x: ix, y: ty }, size: Size { width: accent_bar, height: toast_h } },
                Color::rgba(bar_color.r, bar_color.g, bar_color.b, *alpha),
            );
            c.draw_text(
                msg,
                Point { x: ix + 12.0, y: ty + 9.0 },
                Color::rgba(TEXT.r, TEXT.g, TEXT.b, *alpha), font, 10.0,
            );
        }

        cy += 3.0 * (toast_h + toast_gap) + 16.0;
    }

    // ── API reference ─────────────────────────────────────────────────────────
    lbl(c, font, "tezzera-widgets::overlay API", ix, cy);
    cy += 14.0;
    let api_lines = [
        "Modal::new().visible(true)",
        "  .render_backdrop(&canvas)",
        "  .render_box(&canvas, &theme)",
        "",
        "Dialog::new(title, msg)",
        "  .button(\"Cancel\").button(\"OK\")",
        "  .render(&canvas, &font, &theme)",
        "",
        "ToastQueue::new()",
        "  .push(Toast::new(\"msg\"))",
        "  .tick(dt)  // removes expired",
        "  .render(&canvas, &font, &theme)",
    ];
    for line in &api_lines {
        if !line.is_empty() {
            c.draw_text(line, Point { x: ix + 8.0, y: cy }, MUTED, font, 9.0);
        }
        cy += 12.0;
    }
}

// ── Panel 3 — Screen Transitions ─────────────────────────────────────────────

fn panel_transitions(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32) {
    panel_bg(c, font, x, y, w, h, "Nav Transitions (tezzera-nav-anim)");

    // Build a nav-anim instance to show it compiles and demonstrate the API.
    // (We only use it for metadata display, not for actual rendering here.)
    #[derive(Debug, Clone, PartialEq)]
    enum DemoScreen { Home, Detail }
    impl tezzera_nav::Route for DemoScreen {}

    let mut nav = tezzera_nav_anim::NavigatorAnimated::new(DemoScreen::Home, 340.0, 880.0);
    nav.push_animated(DemoScreen::Detail, TransitionStyle::Slide(SlideDirection::Right));
    // Advance one frame to show physics are live.
    let (_ex, _ey, _ox, _oy, _progress, _done) = nav.update(1.0 / 60.0);

    let ix = x + 10.0;
    let mut cy = y + 32.0;
    let section_w = w - 20.0;

    // ── 3 transition diagrams (3 mini-screens each: before / mid / after) ──
    let diagram_labels = ["Slide(Right)", "Fade", "Scale"];
    let mini_w = 80.0_f32;
    let mini_h = 54.0_f32;
    let arrow_w = 14.0_f32;
    let total_diag_w = 3.0 * mini_w + 2.0 * arrow_w;
    let diag_start_x = ix + (section_w - total_diag_w) / 2.0;

    for (ti, label) in diagram_labels.iter().enumerate() {
        let row_y = cy + ti as f32 * 92.0;
        lbl(c, font, label, ix, row_y);
        let dy = row_y + 12.0;

        for step in 0usize..3 {
            let sx = diag_start_x + step as f32 * (mini_w + arrow_w);
            let sy = dy;

            // Choose background and text per (transition, step)
            let (bg, is_current, step_label) = match (ti, step) {
                // Slide Right: OLD slides out, overlap, NEW settles
                (0, 0) => (Color::rgb(40, 35, 65), false, "OLD"),
                (0, 1) => (Color::rgb(30, 28, 50), false, ">>"),
                (0, 2) => (ACCENT, true,  "NEW"),
                // Fade: full → half → in
                (1, 0) => (Color::rgb(55, 50, 85), false, "100%"),
                (1, 1) => (Color::rgba(ACCENT.r, ACCENT.g, ACCENT.b, 120), false, "50%"),
                (1, 2) => (ACCENT, true,  "IN"),
                // Scale: 100% → 115% → 100%
                (2, 0) => (Color::rgb(60, 55, 90), false, "100%"),
                (2, 1) => (Color::rgb(80, 75, 115), false, "115%"),
                (2, 2) => (ACCENT, true,  "100%"),
                _ => (SURFACE, false, ""),
            };

            c.fill_rect(Rect { origin: Point { x: sx, y: sy }, size: Size { width: mini_w, height: mini_h } }, bg);
            c.stroke_rect(
                Rect { origin: Point { x: sx, y: sy }, size: Size { width: mini_w, height: mini_h } },
                if is_current { ACCENT } else { BORDER },
                if is_current { 2.0 } else { 1.0 },
            );
            c.draw_text(
                step_label,
                Point { x: sx + 22.0, y: sy + 21.0 },
                if is_current { Color::WHITE } else { MUTED }, font, 10.0,
            );

            // Per-transition extra visuals
            match (ti, step) {
                (0, 0) => {
                    // Old screen half-slid-right: mask right half
                    c.fill_rect(
                        Rect { origin: Point { x: sx + mini_w / 2.0, y: sy }, size: Size { width: mini_w / 2.0, height: mini_h } },
                        PANEL,
                    );
                }
                (0, 1) => {
                    // Two screens overlapping
                    c.fill_rect(
                        Rect { origin: Point { x: sx + 10.0, y: sy + 4.0 }, size: Size { width: mini_w - 20.0, height: mini_h - 8.0 } },
                        Color::rgba(ACCENT.r, ACCENT.g, ACCENT.b, 80),
                    );
                }
                (1, 1) => {
                    // Fade mid: horizontal stripes suggest partial opacity
                    for s in 0..5u32 {
                        c.fill_rect(
                            Rect { origin: Point { x: sx + 4.0, y: sy + 4.0 + s as f32 * 9.0 }, size: Size { width: mini_w - 8.0, height: 4.0 } },
                            Color::rgba(210, 205, 230, 55),
                        );
                    }
                }
                (2, 1) => {
                    // Scale 115%: outer glow ring to suggest enlarged frame
                    c.stroke_rect(
                        Rect { origin: Point { x: sx - 5.0, y: sy - 4.0 }, size: Size { width: mini_w + 10.0, height: mini_h + 8.0 } },
                        Color::rgba(ACCENT2.r, ACCENT2.g, ACCENT2.b, 130), 1.5,
                    );
                }
                (2, 2) => {
                    // Scale settled: inner highlight bar
                    c.fill_rect(
                        Rect { origin: Point { x: sx + 10.0, y: sy + 16.0 }, size: Size { width: mini_w - 20.0, height: 5.0 } },
                        Color::rgba(255, 255, 255, 80),
                    );
                }
                _ => {}
            }

            // Arrow between steps
            if step < 2 {
                let ax = sx + mini_w + 1.0;
                let ay = sy + mini_h / 2.0;
                c.fill_rect(
                    Rect { origin: Point { x: ax, y: ay - 1.0 }, size: Size { width: arrow_w - 4.0, height: 2.0 } },
                    MUTED,
                );
                c.fill_rect(
                    Rect { origin: Point { x: ax + arrow_w - 7.0, y: ay - 4.0 }, size: Size { width: 2.0, height: 8.0 } },
                    MUTED,
                );
            }
        }
    }

    cy += 3.0 * 92.0 + 10.0;

    // ── Code snippet ──────────────────────────────────────────────────────────
    lbl(c, font, "API snippet:", ix, cy);
    cy += 12.0;
    let code_lines = [
        "nav.push_animated(",
        "  Screen::Detail,",
        "  Slide(Right)",
        ");",
        "let (ex,ey,ox,oy,..) =",
        "  nav.update(dt);",
    ];
    let code_h = code_lines.len() as f32 * 13.0 + 10.0;
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: section_w, height: code_h } }, SURFACE);
    c.stroke_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: section_w, height: code_h } }, BORDER, 1.0);
    for (i, line) in code_lines.iter().enumerate() {
        c.draw_text(line, Point { x: ix + 8.0, y: cy + 6.0 + i as f32 * 13.0 }, ACCENT3, font, 9.0);
    }
    cy += code_h + 12.0;

    // ── Spring physics curve ──────────────────────────────────────────────────
    lbl(c, font, "Spring curve (k=280, d=26):", ix, cy);
    cy += 12.0;

    let curve_h = 50.0_f32;
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: section_w, height: curve_h } }, SURFACE);
    c.stroke_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: section_w, height: curve_h } }, BORDER, 1.0);

    // Euler spring integration (stiffness=280, damping=26, target=1.0)
    let mut pos = 0.0_f32;
    let mut vel = 0.0_f32;
    let target = 1.0_f32;
    let stiffness = 280.0_f32;
    let damping = 26.0_f32;
    let dt = 1.0_f32 / 60.0;
    let steps = 30usize;

    for step in 0..steps {
        let force = stiffness * (target - pos) - damping * vel;
        vel += force * dt;
        pos += vel * dt;
        let px = ix + step as f32 / (steps - 1) as f32 * section_w;
        let py = cy + curve_h - pos.clamp(0.0, 1.25) * curve_h * 0.78 - 4.0;
        c.fill_circle(Point { x: px, y: py }, 3.0, ACCENT);
    }
}

// ── Panel 4 — Templates & A11y ────────────────────────────────────────────────

fn panel_templates_a11y(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32) {
    panel_bg(c, font, x, y, w, h, "Templates & Accessibility");

    let ix = x + 10.0;
    let mut cy = y + 32.0;
    let section_w = w - 20.0;

    // ── Section A — tzr new --template ───────────────────────────────────────
    lbl(c, font, "tzr new --template:", ix, cy);
    cy += 14.0;

    let mini_w = 140.0_f32;
    let mini_h = 78.0_f32;
    let col_gap = (section_w - 2.0 * mini_w) / 3.0;
    let row_gap = 18.0_f32;

    // 2×2 grid
    let templates: &[(&str, usize, usize)] = &[
        ("counter",   0, 0),
        ("nav-app",   1, 0),
        ("form-app",  0, 1),
        ("dashboard", 1, 1),
    ];

    for (name, col, row) in templates {
        let tx = ix + col_gap + *col as f32 * (mini_w + col_gap);
        let ty = cy + *row as f32 * (mini_h + row_gap + 14.0);

        c.fill_rect(
            Rect { origin: Point { x: tx, y: ty }, size: Size { width: mini_w, height: mini_h } },
            Color::rgb(20, 22, 35),
        );
        c.stroke_rect(
            Rect { origin: Point { x: tx, y: ty }, size: Size { width: mini_w, height: mini_h } },
            BORDER, 1.0,
        );

        match *name {
            "counter" => {
                c.draw_text("42", Point { x: tx + 50.0, y: ty + 14.0 }, ACCENT, font, 22.0);
                c.fill_rect(
                    Rect { origin: Point { x: tx + 28.0, y: ty + 52.0 }, size: Size { width: 84.0, height: 18.0 } },
                    ACCENT,
                );
                c.draw_text("Click me", Point { x: tx + 40.0, y: ty + 57.0 }, Color::WHITE, font, 9.0);
            }
            "nav-app" => {
                c.draw_text("Home Screen", Point { x: tx + 20.0, y: ty + 12.0 }, TEXT, font, 9.0);
                c.fill_rect(
                    Rect { origin: Point { x: tx + 8.0, y: ty + 30.0 }, size: Size { width: 124.0, height: 14.0 } },
                    ACCENT,
                );
                c.draw_text("\u{2192} Detail", Point { x: tx + 16.0, y: ty + 34.0 }, Color::WHITE, font, 8.0);
                c.fill_rect(
                    Rect { origin: Point { x: tx + 8.0, y: ty + 50.0 }, size: Size { width: 124.0, height: 14.0 } },
                    ACCENT2,
                );
                c.draw_text("\u{2192} Settings", Point { x: tx + 14.0, y: ty + 54.0 }, Color::WHITE, font, 8.0);
            }
            "form-app" => {
                c.stroke_rect(
                    Rect { origin: Point { x: tx + 8.0, y: ty + 6.0 }, size: Size { width: 124.0, height: 18.0 } },
                    BORDER, 1.0,
                );
                c.draw_text("Email", Point { x: tx + 12.0, y: ty + 10.0 }, MUTED, font, 8.0);
                c.stroke_rect(
                    Rect { origin: Point { x: tx + 8.0, y: ty + 30.0 }, size: Size { width: 124.0, height: 18.0 } },
                    BORDER, 1.0,
                );
                c.draw_text("Password", Point { x: tx + 12.0, y: ty + 34.0 }, MUTED, font, 8.0);
                c.fill_rect(
                    Rect { origin: Point { x: tx + 20.0, y: ty + 54.0 }, size: Size { width: 100.0, height: 16.0 } },
                    ACCENT,
                );
                c.draw_text("Sign In", Point { x: tx + 40.0, y: ty + 59.0 }, Color::WHITE, font, 8.0);
            }
            "dashboard" => {
                let card_colors = [ACCENT, ACCENT2, ACCENT3, ERROR];
                let card_labels = ["USD", "USR", "REQ", "ERR"];
                for (i, (cc, cl)) in card_colors.iter().zip(card_labels.iter()).enumerate() {
                    let cx = tx + 6.0 + i as f32 * 32.0;
                    c.fill_rect(
                        Rect { origin: Point { x: cx, y: ty + 6.0 }, size: Size { width: 26.0, height: 62.0 } },
                        Color::rgb(28, 30, 46),
                    );
                    c.fill_rect(
                        Rect { origin: Point { x: cx, y: ty + 6.0 }, size: Size { width: 26.0, height: 6.0 } },
                        *cc,
                    );
                    c.draw_text(cl, Point { x: cx + 2.0, y: ty + 42.0 }, MUTED, font, 7.0);
                }
            }
            _ => {}
        }

        // Template name below mini preview
        c.draw_text(name, Point { x: tx + 4.0, y: ty + mini_h + 3.0 }, MUTED, font, 8.0);
    }

    cy += 2.0 * (mini_h + row_gap + 14.0) + 14.0;

    // ── Section B — Accessibility tree ────────────────────────────────────────
    lbl(c, font, "Accessibility tree:", ix, cy);
    cy += 14.0;

    let tree_lines: &[(&str, Color)] = &[
        ("[Main] id:1",                                           MUTED),
        ("  \u{251C}\u{2500} [Navigation] id:2",                 MUTED),
        ("  \u{2502}  \u{2514}\u{2500} [Button \"Home\"] id:3 \u{25CF}focused", ACCENT),
        ("  \u{251C}\u{2500} [Heading \"Profile\"] id:4",        MUTED),
        ("  \u{2514}\u{2500} [TextInput \"Email\"] id:5",        MUTED),
        ("       (focusable=true)",                               MUTED),
    ];

    for (text, color) in tree_lines {
        c.draw_text(text, Point { x: ix + 4.0, y: cy }, *color, font, 9.0);
        cy += 13.0;
    }

    cy += 8.0;

    // ── JSON A11y snippet ─────────────────────────────────────────────────────
    lbl(c, font, "A11y node JSON:", ix, cy);
    cy += 12.0;

    let json_lines = [
        "[{\"id\":3,\"role\":\"button\",",
        "  \"aria-label\":\"Home\",",
        "  \"focusable\":true}]",
    ];
    let json_h = json_lines.len() as f32 * 13.0 + 10.0;
    c.fill_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: section_w, height: json_h } }, SURFACE);
    c.stroke_rect(Rect { origin: Point { x: ix, y: cy }, size: Size { width: section_w, height: json_h } }, BORDER, 1.0);
    for (i, line) in json_lines.iter().enumerate() {
        c.draw_text(line, Point { x: ix + 8.0, y: cy + 6.0 + i as f32 * 13.0 }, ACCENT2, font, 9.0);
    }
}

// ── main ─────────────────────────────────────────────────────────────────────

fn main() {
    let font = FontCache::system_mono().expect("system font not found");
    let mut c = SkiaCanvas::new(W, H);

    // Global background
    c.clear(BG);

    // Thin accent header stripe
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: 0.0 }, size: Size { width: W as f32, height: 8.0 } },
        ACCENT,
    );

    // Panels
    panel_image_rendering(&mut c, &font, panel_x(0), PANEL_Y, PANEL_W, PANEL_H);
    panel_overlay        (&mut c, &font, panel_x(1), PANEL_Y, PANEL_W, PANEL_H);
    panel_transitions    (&mut c, &font, panel_x(2), PANEL_Y, PANEL_W, PANEL_H);
    panel_templates_a11y (&mut c, &font, panel_x(3), PANEL_Y, PANEL_W, PANEL_H);

    // Status bar
    let sb_y = H as f32 - 14.0;
    c.fill_rect(
        Rect { origin: Point { x: 0.0, y: sb_y - 4.0 }, size: Size { width: W as f32, height: 18.0 } },
        Color::rgb(14, 15, 22),
    );
    c.draw_text(
        "TEZZERA  \u{2022}  Phase 5  \u{2022}  Images \u{2713}  Overlays \u{2713}  Transitions \u{2713}  Templates \u{2713}  A11y \u{2713}",
        Point { x: W as f32 / 2.0 - 255.0, y: sb_y - 1.0 },
        MUTED, &font, 10.0,
    );

    // Encode and write
    let png = c.encode_png().expect("png encode failed");
    std::fs::write("phase5_demo.png", &png).expect("write phase5_demo.png");
    println!("Saved phase5_demo.png ({}x{})", W, H);
}
