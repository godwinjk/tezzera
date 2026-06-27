//! Animated Counter — spring-physics demonstration for TEZZERA.
//!
//! Shows a counter that spring-animates to its target value each time it changes.
//! Four spring presets selectable at runtime: Gentle / Bouncy / Stiff / Slow
//!
//! Run: cargo run -p tezzera-examples --bin animated_counter

use tezzera_animate::Spring;
use tezzera_core::types::{Point, Rect, Size};
use tezzera_platform::{InputEvent, Key, MouseButton, TezzeraApp};
use tezzera_render::{Color, FontCache, SkiaCanvas};
use tezzera_state::use_atom;

const W: u32 = 520;
const H: u32 = 420;

// Palette
const BG: Color      = Color::rgb(18, 18, 28);
const CARD: Color    = Color::rgb(28, 30, 46);
const BORDER: Color  = Color::rgb(55, 60, 90);
const ACCENT: Color  = Color::rgb(103, 80, 164);
const ACCENT_L: Color = Color::rgb(140, 120, 200);
const GREEN: Color   = Color::rgb(72, 199, 116);
const RED_C: Color   = Color::rgb(200, 70, 70);
const TEXT_HI: Color = Color::rgb(230, 225, 229);
const TEXT_LO: Color = Color::rgb(140, 135, 160);

#[derive(Clone, Copy, PartialEq, Debug)]
enum SpringPreset {
    Gentle,
    Bouncy,
    Stiff,
    Slow,
}

impl SpringPreset {
    fn make_spring(self, initial: f32, target: f32) -> Spring {
        match self {
            SpringPreset::Gentle => Spring::new(initial, target).stiffness(120.0).damping(14.0),
            SpringPreset::Bouncy => Spring::new(initial, target).stiffness(300.0).damping(10.0),
            SpringPreset::Stiff  => Spring::new(initial, target).stiffness(400.0).damping(28.0),
            SpringPreset::Slow   => Spring::new(initial, target).stiffness(80.0).damping(20.0),
        }
    }

    fn label(self) -> &'static str {
        match self {
            SpringPreset::Gentle => "Gentle",
            SpringPreset::Bouncy => "Bouncy",
            SpringPreset::Stiff  => "Stiff",
            SpringPreset::Slow   => "Slow",
        }
    }
}

fn hits(mx: f32, my: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
    mx >= x && mx <= x + w && my >= y && my <= y + h
}

fn draw_btn(
    c: &mut SkiaCanvas,
    font: &FontCache,
    label: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    bg: Color,
) {
    c.fill_rect(
        Rect { origin: Point { x, y }, size: Size { width: w, height: h } },
        bg,
    );
    c.stroke_rect(
        Rect { origin: Point { x, y }, size: Size { width: w, height: h } },
        BORDER,
        1.0,
    );
    let char_w = 8.6_f32;
    let tx = x + (w - label.len() as f32 * char_w) / 2.0;
    let ty = y + (h - 16.0) / 2.0;
    c.draw_text(label, Point { x: tx, y: ty }, TEXT_HI, font, 16.0);
}

fn main() {
    let font = FontCache::system_mono().expect("no system font");
    let count = use_atom(0_i32);
    let mut display_value = 0.0_f32;
    let mut current_preset = SpringPreset::Bouncy;
    let mut spring = current_preset.make_spring(0.0, 0.0);
    let mut mx = 0.0_f32;
    let mut my = 0.0_f32;

    // Preset list — fixed order used for layout and hit-testing
    let presets = [
        SpringPreset::Gentle,
        SpringPreset::Bouncy,
        SpringPreset::Stiff,
        SpringPreset::Slow,
    ];

    TezzeraApp::new()
        .title("Animated Counter — TEZZERA")
        .size(W, H)
        .run(move |canvas: &mut SkiaCanvas, events: &[InputEvent]| {
            // ── Layout geometry (computed once per frame) ─────────────────────
            let card_w = 460.0_f32;
            let card_h = 330.0_f32;
            let card_x = (W as f32 - card_w) / 2.0;
            let card_y = (H as f32 - card_h) / 2.0;

            // Action buttons row
            let bw = 110.0_f32;
            let bh = 44.0_f32;
            let row_x = (W as f32 - bw * 3.0 - 16.0) / 2.0;
            let row_y = card_y + card_h - 126.0;

            // Preset selector row
            let pw = 90.0_f32;
            let ph = 32.0_f32;
            let pgap = 8.0_f32;
            let presets_total_w = pw * presets.len() as f32 + pgap * (presets.len() as f32 - 1.0);
            let preset_x0 = (W as f32 - presets_total_w) / 2.0;
            let preset_y = card_y + card_h - 52.0;

            // ── Process input events ──────────────────────────────────────────
            for ev in events {
                match ev {
                    InputEvent::MouseMove { x, y } => {
                        mx = *x;
                        my = *y;
                    }
                    InputEvent::MouseDown { x, y, button: MouseButton::Left } => {
                        // Action buttons
                        if hits(*x, *y, row_x, row_y, bw, bh) {
                            count.update(|n| n - 1);
                        }
                        if hits(*x, *y, row_x + bw + 8.0, row_y, bw, bh) {
                            count.update(|n| n + 1);
                        }
                        if hits(*x, *y, row_x + (bw + 8.0) * 2.0, row_y, bw, bh) {
                            count.set(0);
                        }
                        // Preset buttons
                        for (i, &preset) in presets.iter().enumerate() {
                            let px = preset_x0 + i as f32 * (pw + pgap);
                            if hits(*x, *y, px, preset_y, pw, ph) && preset != current_preset {
                                current_preset = preset;
                                spring = preset.make_spring(display_value, count.get() as f32);
                            }
                        }
                    }
                    InputEvent::Text { character: '+' }
                    | InputEvent::KeyDown { key: Key::ArrowUp } => {
                        count.update(|n| n + 1);
                    }
                    InputEvent::Text { character: '-' }
                    | InputEvent::KeyDown { key: Key::ArrowDown } => {
                        count.update(|n| n - 1);
                    }
                    InputEvent::Text { character: 'r' }
                    | InputEvent::Text { character: 'R' } => {
                        count.set(0);
                    }
                    _ => {}
                }
            }

            // ── Advance spring ────────────────────────────────────────────────
            spring.set_target(count.get() as f32);
            display_value = spring.update(1.0 / 60.0);

            // ── Draw ──────────────────────────────────────────────────────────
            canvas.clear(BG);

            // Card background
            canvas.fill_rect(
                Rect { origin: Point { x: card_x, y: card_y }, size: Size { width: card_w, height: card_h } },
                CARD,
            );
            canvas.stroke_rect(
                Rect { origin: Point { x: card_x, y: card_y }, size: Size { width: card_w, height: card_h } },
                BORDER,
                1.0,
            );

            // Title
            let title = "TEZZERA Animated Counter";
            let title_x = card_x + (card_w - title.len() as f32 * 7.6) / 2.0;
            canvas.draw_text(title, Point { x: title_x, y: card_y + 22.0 }, TEXT_LO, &font, 14.0);

            // Animated counter value (big)
            let display_int = display_value.round() as i32;
            let num_label = format!("{display_int:+}");
            let num_color = if display_int > 0 {
                GREEN
            } else if display_int < 0 {
                RED_C
            } else {
                TEXT_HI
            };
            let num_w = num_label.len() as f32 * 22.0;
            canvas.draw_text(
                &num_label,
                Point { x: card_x + (card_w - num_w) / 2.0, y: card_y + 68.0 },
                num_color,
                &font,
                40.0,
            );

            // Spring preset subtitle
            let subtitle = format!("Spring: {}", current_preset.label());
            let sub_x = card_x + (card_w - subtitle.len() as f32 * 7.2) / 2.0;
            canvas.draw_text(
                &subtitle,
                Point { x: sub_x, y: card_y + 124.0 },
                ACCENT_L,
                &font,
                13.0,
            );

            // Accent divider
            canvas.fill_rect(
                Rect {
                    origin: Point { x: card_x + card_w * 0.3, y: card_y + 142.0 },
                    size: Size { width: card_w * 0.4, height: 2.0 },
                },
                Color::rgba(ACCENT.r, ACCENT.g, ACCENT.b, 80),
            );

            // Action buttons
            let dec_bg = if hits(mx, my, row_x, row_y, bw, bh) {
                Color::rgb(100, 80, 80)
            } else {
                Color::rgb(68, 58, 58)
            };
            let inc_bg = if hits(mx, my, row_x + bw + 8.0, row_y, bw, bh) {
                Color::rgb(120, 98, 180)
            } else {
                ACCENT
            };
            let rst_bg = if hits(mx, my, row_x + (bw + 8.0) * 2.0, row_y, bw, bh) {
                Color::rgb(230, 100, 100)
            } else {
                RED_C
            };

            draw_btn(canvas, &font, "−  Dec", row_x,                      row_y, bw, bh, dec_bg);
            draw_btn(canvas, &font, "+  Inc", row_x + bw + 8.0,           row_y, bw, bh, inc_bg);
            draw_btn(canvas, &font, "Reset",  row_x + (bw + 8.0) * 2.0,  row_y, bw, bh, rst_bg);

            // Preset selector buttons
            for (i, &preset) in presets.iter().enumerate() {
                let px = preset_x0 + i as f32 * (pw + pgap);
                let is_active = preset == current_preset;
                let is_hov = hits(mx, my, px, preset_y, pw, ph);
                let pbg = if is_active {
                    ACCENT
                } else if is_hov {
                    Color::rgb(45, 48, 72)
                } else {
                    Color::rgb(35, 38, 58)
                };
                let pb = if is_active { ACCENT_L } else { BORDER };
                let border_w = if is_active { 2.0 } else { 1.0 };
                canvas.fill_rect(
                    Rect { origin: Point { x: px, y: preset_y }, size: Size { width: pw, height: ph } },
                    pbg,
                );
                canvas.stroke_rect(
                    Rect { origin: Point { x: px, y: preset_y }, size: Size { width: pw, height: ph } },
                    pb,
                    border_w,
                );
                let plabel = preset.label();
                let ptx = px + (pw - plabel.len() as f32 * 7.5) / 2.0;
                let pty = preset_y + (ph - 12.0) / 2.0;
                let ptcolor = if is_active { Color::WHITE } else { TEXT_LO };
                canvas.draw_text(plabel, Point { x: ptx, y: pty }, ptcolor, &font, 12.0);
            }

            // Footer hint
            let hint = "Click buttons  or  press + - r";
            canvas.draw_text(
                hint,
                Point {
                    x: (W as f32 - hint.len() as f32 * 6.5) / 2.0,
                    y: H as f32 - 14.0,
                },
                TEXT_LO,
                &font,
                11.0,
            );
        });
}
