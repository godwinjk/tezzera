//! Counter Window — the first real TEZZERA windowed app.
//!
//! Demonstrates: TezzeraApp event loop, real font rendering, reactive Atom state,
//! Column layout, click-hit-testing on buttons.
//!
//! Run: cargo run -p tezzera-examples --bin counter_window

use tezzera_core::types::{Point, Rect, Size};
use tezzera_platform::{InputEvent, MouseButton, TezzeraApp};
use tezzera_render::{Color, FontCache, SkiaCanvas};
use tezzera_state::use_atom;

const W: u32 = 480;
const H: u32 = 360;

// Palette
const BG:         Color = Color::rgb(18,  20,  34);
const CARD:       Color = Color::rgb(28,  30,  50);
const BORDER:     Color = Color::rgb(55,  60,  90);
const ACCENT:     Color = Color::rgb(100, 160, 255);
const ACCENT_HOV: Color = Color::rgb(130, 185, 255);
const BTN_DEC:    Color = Color::rgb(60,  60,  90);
const BTN_DEC_H:  Color = Color::rgb(80,  80, 115);
const BTN_RST:    Color = Color::rgb(200,  70,  70);
const BTN_RST_H:  Color = Color::rgb(230, 100, 100);
const TEXT_HI:    Color = Color::rgb(240, 242, 255);
const TEXT_LO:    Color = Color::rgb(130, 135, 170);

fn hits(mx: f32, my: f32, x: f32, y: f32, w: f32, h: f32) -> bool {
    mx >= x && mx <= x + w && my >= y && my <= y + h
}

fn draw_btn(c: &mut SkiaCanvas, font: &FontCache, label: &str,
            x: f32, y: f32, w: f32, h: f32, bg: Color) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, bg);
    c.stroke_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, BORDER, 1.0);
    // Center text inside button (approximate: 8px per char at 16px)
    let char_w = 8.6_f32;
    let tx = x + (w - label.len() as f32 * char_w) / 2.0;
    let ty = y + (h - 16.0) / 2.0;
    c.draw_text(label, Point { x: tx, y: ty }, TEXT_HI, font, 16.0);
}

fn main() {
    // Load font once — used every frame
    let font = FontCache::system_mono()
        .expect("No system font found — install a TTF font at a standard path");

    // Reactive state — persists across frames via closure capture
    let count = use_atom(0_i32);

    // Track hover state across frames
    let mut mx = 0.0_f32;
    let mut my = 0.0_f32;

    TezzeraApp::new()
        .title("Counter — TEZZERA")
        .size(W, H)
        .run(move |canvas: &mut SkiaCanvas, events: &[InputEvent]| {
            // Process input
            for ev in events {
                match ev {
                    InputEvent::MouseMove { x, y } => { mx = *x; my = *y; }
                    InputEvent::MouseDown { x, y, button: MouseButton::Left } => {
                        // Button layout (matched to draw positions below)
                        let bw = 110.0_f32;
                        let bh = 44.0_f32;
                        let row_x = (W as f32 - bw * 3.0 - 16.0) / 2.0;
                        let row_y = H as f32 / 2.0 + 20.0;

                        if hits(*x, *y, row_x,                row_y, bw, bh) {
                            count.update(|n| n - 1);
                        }
                        if hits(*x, *y, row_x + bw + 8.0,     row_y, bw, bh) {
                            count.update(|n| n + 1);
                        }
                        if hits(*x, *y, row_x + (bw + 8.0)*2.0, row_y, bw, bh) {
                            count.set(0);
                        }
                    }
                    _ => {}
                }
            }

            let c_val = count.get();

            // ── Background ────────────────────────────────────────────────────
            canvas.clear(BG);

            // ── Center card ───────────────────────────────────────────────────
            let card_w = 360.0_f32;
            let card_h = 220.0_f32;
            let card_x = (W as f32 - card_w) / 2.0;
            let card_y = (H as f32 - card_h) / 2.0;
            canvas.fill_rect(Rect { origin: Point { x: card_x, y: card_y }, size: Size { width: card_w, height: card_h } }, CARD);
            canvas.stroke_rect(Rect { origin: Point { x: card_x, y: card_y }, size: Size { width: card_w, height: card_h } }, BORDER, 1.0);

            // ── Title ─────────────────────────────────────────────────────────
            let title = "TEZZERA Counter";
            canvas.draw_text(title, Point { x: card_x + (card_w - title.len() as f32 * 8.0) / 2.0, y: card_y + 18.0 }, TEXT_LO, &font, 14.0);

            // ── Count value (big) ─────────────────────────────────────────────
            let label = format!("{c_val:+}");
            let count_color = if c_val > 0 { ACCENT } else if c_val < 0 { Color::rgb(255, 100, 100) } else { TEXT_HI };
            // Large text: draw at 40px
            let lw = label.len() as f32 * 22.0;
            canvas.draw_text(&label, Point { x: card_x + (card_w - lw) / 2.0, y: card_y + 60.0 }, count_color, &font, 40.0);

            // ── Accent underline ──────────────────────────────────────────────
            canvas.fill_rect(Rect {
                origin: Point { x: card_x + card_w * 0.3, y: card_y + 110.0 },
                size: Size { width: card_w * 0.4, height: 2.0 },
            }, Color::rgba(ACCENT.r, ACCENT.g, ACCENT.b, 80));

            // ── Buttons row ───────────────────────────────────────────────────
            let bw = 110.0_f32;
            let bh = 44.0_f32;
            let row_x = (W as f32 - bw * 3.0 - 16.0) / 2.0;
            let row_y = H as f32 / 2.0 + 20.0;

            let dec_bg  = if hits(mx, my, row_x, row_y, bw, bh) { BTN_DEC_H } else { BTN_DEC };
            let inc_bg  = if hits(mx, my, row_x + bw + 8.0, row_y, bw, bh) { ACCENT_HOV } else { ACCENT };
            let rst_bg  = if hits(mx, my, row_x + (bw + 8.0) * 2.0, row_y, bw, bh) { BTN_RST_H } else { BTN_RST };

            draw_btn(canvas, &font, "−  Dec",  row_x,                 row_y, bw, bh, dec_bg);
            draw_btn(canvas, &font, "+  Inc",  row_x + bw + 8.0,      row_y, bw, bh, inc_bg);
            draw_btn(canvas, &font, "Reset",   row_x + (bw + 8.0)*2.0, row_y, bw, bh, rst_bg);

            // ── Footer hint ───────────────────────────────────────────────────
            let hint = "Click the buttons or press +  -  r";
            canvas.draw_text(hint, Point {
                x: (W as f32 - hint.len() as f32 * 6.5) / 2.0,
                y: H as f32 - 20.0,
            }, TEXT_LO, &font, 12.0);

            // Keyboard shortcuts
            for ev in events {
                match ev {
                    InputEvent::Text { character: '+' } | InputEvent::KeyDown { key: tezzera_platform::Key::ArrowUp } => {
                        count.update(|n| n + 1);
                    }
                    InputEvent::Text { character: '-' } | InputEvent::KeyDown { key: tezzera_platform::Key::ArrowDown } => {
                        count.update(|n| n - 1);
                    }
                    InputEvent::Text { character: 'r' } | InputEvent::Text { character: 'R' } => {
                        count.set(0);
                    }
                    _ => {}
                }
            }
        });
}
