//! Nav Demo — live multi-screen navigation for TEZZERA.
//!
//! Three screens driven by a `Navigator<Screen>` stack:
//!   Home     → push Profile / Settings
//!   Profile  → show user info, back button
//!   Settings → theme toggle, back button
//!
//! Run: cargo run -p tezzera-examples --bin nav_demo

use tezzera_core::types::{Point, Rect, Size};
use tezzera_nav::{Navigator, Route};
use tezzera_platform::{InputEvent, Key, MouseButton, TezzeraApp};
use tezzera_render::{Color, FontCache, SkiaCanvas};
use tezzera_state::use_atom;
use tezzera_theme::built_in::{dark_theme, light_theme};
use tezzera_theme::set_theme;

const W: u32 = 640;
const H: u32 = 480;

// ── Palette ──────────────────────────────────────────────────────────────────
const BG: Color     = Color::rgb(18,  18,  28);
const ACCENT: Color = Color::rgb(103, 80,  164);
const SURFACE: Color = Color::rgb(28,  30,  46);
const TEXT: Color   = Color::rgb(230, 225, 229);
const MUTED: Color  = Color::rgb(140, 145, 175);
const BORDER: Color = Color::rgb(55,  60,  90);
const HOME_BG: Color = Color::rgb(26, 18, 42);  // purple-tinted home background
const _ERROR: Color  = Color::rgb(200, 70,  70);
const _GREEN: Color  = Color::rgb(72,  199, 116);

// ── Screen enum ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Screen {
    Home,
    Profile,
    Settings,
}

impl Route for Screen {}

// ── Helpers ───────────────────────────────────────────────────────────────────

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
    border: Color,
) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, bg);
    c.stroke_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, border, 1.5);
    let char_w = 8.0_f32;
    let tx = x + (w - label.len() as f32 * char_w) / 2.0;
    let ty = y + (h - 14.0) / 2.0;
    c.draw_text(label, Point { x: tx, y: ty }, TEXT, font, 14.0);
}

fn btn_color(is_hov: bool, base: Color, hover_shift: i16) -> Color {
    if is_hov {
        Color::rgb(
            (base.r as i16 + hover_shift).clamp(0, 255) as u8,
            (base.g as i16 + hover_shift).clamp(0, 255) as u8,
            (base.b as i16 + hover_shift).clamp(0, 255) as u8,
        )
    } else {
        base
    }
}

// ── Screen renderers ──────────────────────────────────────────────────────────

fn render_home(
    c: &mut SkiaCanvas,
    font: &FontCache,
    mx: f32,
    my: f32,
) -> ([f32; 4], [f32; 4]) {
    c.clear(HOME_BG);

    // Top accent strip
    c.fill_rect(Rect { origin: Point { x: 0.0, y: 0.0 }, size: Size { width: W as f32, height: 4.0 } }, ACCENT);

    // Title
    let title = "TEZZERA \u{2014} Nav Demo";
    let tx = (W as f32 - title.len() as f32 * 13.0) / 2.0;
    c.draw_text(title, Point { x: tx, y: 80.0 }, TEXT, font, 22.0);

    // Subtitle
    let sub = "Use the buttons below to navigate between screens";
    let sx = (W as f32 - sub.len() as f32 * 7.5) / 2.0;
    c.draw_text(sub, Point { x: sx, y: 118.0 }, MUTED, font, 13.0);

    // Divider
    c.fill_rect(
        Rect { origin: Point { x: W as f32 * 0.2, y: 148.0 }, size: Size { width: W as f32 * 0.6, height: 2.0 } },
        Color::rgba(ACCENT.r, ACCENT.g, ACCENT.b, 80),
    );

    // Buttons
    let bw = 200.0_f32;
    let bh = 48.0_f32;
    let bx = (W as f32 - bw) / 2.0;
    let b1y = 200.0_f32;
    let b2y = 268.0_f32;

    let b1_hov = hits(mx, my, bx, b1y, bw, bh);
    let b2_hov = hits(mx, my, bx, b2y, bw, bh);

    let b1_bg = btn_color(b1_hov, ACCENT, 30);
    let b2_bg = btn_color(b2_hov, SURFACE, 15);

    draw_btn(c, font, "\u{2192} Profile", bx, b1y, bw, bh, b1_bg, ACCENT);
    draw_btn(c, font, "\u{2192} Settings", bx, b2y, bw, bh, b2_bg, BORDER);

    // Footer hint
    c.draw_text(
        "Click a button to navigate",
        Point { x: (W as f32 - 26.0 * 7.5) / 2.0, y: H as f32 - 28.0 },
        MUTED,
        font,
        12.0,
    );

    ([bx, b1y, bw, bh], [bx, b2y, bw, bh])
}

fn render_profile(
    c: &mut SkiaCanvas,
    font: &FontCache,
    mx: f32,
    my: f32,
    depth: usize,
) -> [f32; 4] {
    c.clear(BG);

    // Top accent strip
    c.fill_rect(Rect { origin: Point { x: 0.0, y: 0.0 }, size: Size { width: W as f32, height: 4.0 } }, ACCENT);

    // Title
    c.draw_text("Profile", Point { x: 40.0, y: 30.0 }, TEXT, font, 20.0);

    // Stack depth badge
    let depth_label = format!("Stack depth: {}", depth);
    c.draw_text(
        &depth_label,
        Point { x: W as f32 - depth_label.len() as f32 * 7.5 - 20.0, y: 34.0 },
        MUTED,
        font,
        12.0,
    );

    // Card
    let card_x = 60.0_f32;
    let card_y = 80.0_f32;
    let card_w = W as f32 - 120.0;
    let card_h = 240.0_f32;
    c.fill_rect(Rect { origin: Point { x: card_x, y: card_y }, size: Size { width: card_w, height: card_h } }, SURFACE);
    c.stroke_rect(Rect { origin: Point { x: card_x, y: card_y }, size: Size { width: card_w, height: card_h } }, BORDER, 1.0);

    // Avatar circle
    c.fill_circle(Point { x: card_x + 60.0, y: card_y + 70.0 }, 36.0, ACCENT);
    c.draw_text("GJ", Point { x: card_x + 48.0, y: card_y + 58.0 }, TEXT, font, 18.0);

    // Info
    c.draw_text("Name: Godwin Joseph", Point { x: card_x + 120.0, y: card_y + 50.0 }, TEXT, font, 15.0);
    c.draw_text("Role: UI Engineer", Point { x: card_x + 120.0, y: card_y + 76.0 }, TEXT, font, 15.0);
    c.draw_text("Framework: TEZZERA", Point { x: card_x + 120.0, y: card_y + 102.0 }, MUTED, font, 13.0);

    // Divider inside card
    c.fill_rect(
        Rect { origin: Point { x: card_x + 16.0, y: card_y + 134.0 }, size: Size { width: card_w - 32.0, height: 1.0 } },
        BORDER,
    );
    c.draw_text("Phase 4 — Multi-screen Navigation", Point { x: card_x + 16.0, y: card_y + 150.0 }, MUTED, font, 11.0);
    c.draw_text("Navigator<Screen> with push / pop / depth tracking", Point { x: card_x + 16.0, y: card_y + 168.0 }, MUTED, font, 11.0);

    // Back button
    let bw = 160.0_f32;
    let bh = 44.0_f32;
    let bx = (W as f32 - bw) / 2.0;
    let by = card_y + card_h + 30.0;
    let hov = hits(mx, my, bx, by, bw, bh);
    let bg = btn_color(hov, Color::rgb(45, 35, 70), 20);
    draw_btn(c, font, "\u{2190} Back", bx, by, bw, bh, bg, ACCENT);

    // Footer
    c.draw_text(
        "Press Backspace to go back",
        Point { x: (W as f32 - 26.0 * 7.5) / 2.0, y: H as f32 - 28.0 },
        MUTED, font, 12.0,
    );

    [bx, by, bw, bh]
}

fn render_settings(
    c: &mut SkiaCanvas,
    font: &FontCache,
    mx: f32,
    my: f32,
    is_dark: bool,
) -> ([f32; 4], [f32; 4]) {
    let screen_bg = if is_dark { BG } else { Color::rgb(240, 235, 248) };
    let label_color = if is_dark { TEXT } else { Color::rgb(30, 25, 50) };
    let muted_color = if is_dark { MUTED } else { Color::rgb(100, 90, 130) };

    c.clear(screen_bg);

    // Top accent strip
    c.fill_rect(Rect { origin: Point { x: 0.0, y: 0.0 }, size: Size { width: W as f32, height: 4.0 } }, ACCENT);

    // Title
    c.draw_text("Settings", Point { x: 40.0, y: 30.0 }, label_color, font, 20.0);

    // Section card
    let card_x = 60.0_f32;
    let card_y = 80.0_f32;
    let card_w = W as f32 - 120.0;
    let card_h = 200.0_f32;
    let card_bg = if is_dark { SURFACE } else { Color::rgb(250, 248, 255) };
    let card_border = if is_dark { BORDER } else { Color::rgb(200, 195, 220) };
    c.fill_rect(Rect { origin: Point { x: card_x, y: card_y }, size: Size { width: card_w, height: card_h } }, card_bg);
    c.stroke_rect(Rect { origin: Point { x: card_x, y: card_y }, size: Size { width: card_w, height: card_h } }, card_border, 1.0);

    // Theme section label
    c.draw_text("Appearance", Point { x: card_x + 16.0, y: card_y + 18.0 }, muted_color, font, 11.0);

    // Toggle button
    let theme_label = if is_dark { "Theme: Dark" } else { "Theme: Light" };
    let tbw = 200.0_f32;
    let tbh = 44.0_f32;
    let tbx = card_x + (card_w - tbw) / 2.0;
    let tby = card_y + 52.0;
    let t_hov = hits(mx, my, tbx, tby, tbw, tbh);
    let t_bg = if t_hov {
        Color::rgb(120, 100, 180)
    } else {
        ACCENT
    };
    draw_btn(c, font, theme_label, tbx, tby, tbw, tbh, t_bg, ACCENT);

    let status = if is_dark {
        "Dark mode is active"
    } else {
        "Light mode is active"
    };
    c.draw_text(status, Point { x: card_x + (card_w - status.len() as f32 * 7.5) / 2.0, y: tby + tbh + 12.0 }, muted_color, font, 12.0);

    // Divider
    c.fill_rect(
        Rect { origin: Point { x: card_x + 16.0, y: card_y + 134.0 }, size: Size { width: card_w - 32.0, height: 1.0 } },
        card_border,
    );
    c.draw_text("tezzera_theme::set_theme() in action", Point { x: card_x + 16.0, y: card_y + 150.0 }, muted_color, font, 11.0);

    // Back button
    let bw = 160.0_f32;
    let bh = 44.0_f32;
    let bx = (W as f32 - bw) / 2.0;
    let by = card_y + card_h + 30.0;
    let b_hov = hits(mx, my, bx, by, bw, bh);
    let b_bg = btn_color(b_hov, Color::rgb(45, 35, 70), 20);
    draw_btn(c, font, "\u{2190} Back", bx, by, bw, bh, b_bg, ACCENT);

    // Footer
    c.draw_text(
        "Press Backspace to go back",
        Point { x: (W as f32 - 26.0 * 7.5) / 2.0, y: H as f32 - 28.0 },
        muted_color, font, 12.0,
    );

    ([tbx, tby, tbw, tbh], [bx, by, bw, bh])
}

// ── main ─────────────────────────────────────────────────────────────────────

fn main() {
    let font = FontCache::system_mono().expect("no system font");
    let nav = Navigator::new(Screen::Home);
    let theme_is_dark = use_atom(false);

    let mut mx = 0.0_f32;
    let mut my = 0.0_f32;

    TezzeraApp::new()
        .title("TEZZERA \u{2014} Nav Demo")
        .size(W, H)
        .run(move |canvas: &mut SkiaCanvas, events: &[InputEvent]| {
            let is_dark = theme_is_dark.get();
            let current = nav.current().unwrap_or(Screen::Home);

            // ── Process events ────────────────────────────────────────────────
            for ev in events {
                match ev {
                    InputEvent::MouseMove { x, y } => {
                        mx = *x;
                        my = *y;
                    }

                    InputEvent::MouseDown { x, y, button: MouseButton::Left } => {
                        match &current {
                            Screen::Home => {
                                // Buttons are computed below — we replicate the same
                                // geometry here so click testing works before rendering.
                                let bw = 200.0_f32;
                                let bh = 48.0_f32;
                                let bx = (W as f32 - bw) / 2.0;
                                if hits(*x, *y, bx, 200.0, bw, bh) {
                                    nav.push(Screen::Profile);
                                } else if hits(*x, *y, bx, 268.0, bw, bh) {
                                    nav.push(Screen::Settings);
                                }
                            }
                            Screen::Profile => {
                                let bw = 160.0_f32;
                                let bh = 44.0_f32;
                                let bx = (W as f32 - bw) / 2.0;
                                let by = 80.0 + 240.0 + 30.0; // card_y + card_h + gap
                                if hits(*x, *y, bx, by, bw, bh) {
                                    nav.pop();
                                }
                            }
                            Screen::Settings => {
                                // Theme toggle
                                let card_x = 60.0_f32;
                                let card_y = 80.0_f32;
                                let card_w = W as f32 - 120.0;
                                let tbw = 200.0_f32;
                                let tbh = 44.0_f32;
                                let tbx = card_x + (card_w - tbw) / 2.0;
                                let tby = card_y + 52.0;
                                if hits(*x, *y, tbx, tby, tbw, tbh) {
                                    let new_dark = !is_dark;
                                    theme_is_dark.set(new_dark);
                                    if new_dark {
                                        set_theme(dark_theme());
                                    } else {
                                        set_theme(light_theme());
                                    }
                                }
                                // Back button
                                let bw = 160.0_f32;
                                let bh = 44.0_f32;
                                let bx = (W as f32 - bw) / 2.0;
                                let by = card_y + 200.0 + 30.0;
                                if hits(*x, *y, bx, by, bw, bh) {
                                    nav.pop();
                                }
                            }
                        }
                    }

                    InputEvent::KeyDown { key: Key::Backspace } => {
                        nav.pop();
                    }

                    _ => {}
                }
            }

            // Re-read current after event handling
            let current = nav.current().unwrap_or(Screen::Home);
            let depth = nav.depth();

            // ── Render current screen ─────────────────────────────────────────
            match current {
                Screen::Home => {
                    render_home(canvas, &font, mx, my);
                }
                Screen::Profile => {
                    render_profile(canvas, &font, mx, my, depth);
                }
                Screen::Settings => {
                    let dark = theme_is_dark.get();
                    render_settings(canvas, &font, mx, my, dark);
                }
            }
        });
}
