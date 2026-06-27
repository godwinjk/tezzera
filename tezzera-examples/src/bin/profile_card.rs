//! Profile Card — social profile UI with avatar, stats, and bio.
//!
//! Demonstrates: Column layout, circles, rounded-rect cards, state atoms.
//! Run:   cargo run -p tezzera-examples --bin profile_card
//! Output: profile_card.png (400×620)

use tezzera_core::types::{Point, Rect, Size};
use tezzera_layout::{Column, Constraints, Row};
use tezzera_render::{Color, FontCache, SkiaCanvas};
use tezzera_state::use_atom;

const W: u32 = 400;
const H: u32 = 620;

// Palette
const BG:           Color = Color::rgb(18,  18,  28);
const ACCENT:       Color = Color::rgb(100, 160, 255);
const ACCENT_DIM:   Color = Color::rgb( 60, 100, 180);
const CARD_BG:      Color = Color::rgb(28,  30,  46);
const CARD_BORDER:  Color = Color::rgb(50,  55,  80);
const TEXT_PRIMARY: Color = Color::rgb(240, 240, 255);
const TEXT_MUTED:   Color = Color::rgb(140, 145, 175);
const GREEN:        Color = Color::rgb( 72, 199, 116);
const ORANGE:       Color = Color::rgb(255, 160,  60);

fn stat_card(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32, value: &str, label: &str, color: Color) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, CARD_BG);
    c.stroke_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, CARD_BORDER, 1.0);
    // value — center using monospace approximation (px * 0.55 per glyph)
    let vw = value.len() as f32 * (16.0 * 0.55);
    c.draw_text(value, Point { x: x + (w - vw) * 0.5, y: y + 10.0 }, color, font, 16.0);
    // label
    let lw = label.len() as f32 * (10.0 * 0.55);
    c.draw_text(label, Point { x: x + (w - lw) * 0.5, y: y + 32.0 }, TEXT_MUTED, font, 10.0);
}

fn skill_bar(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, label: &str, pct: f32, color: Color) {
    c.draw_text(label, Point { x, y }, TEXT_MUTED, font, 10.0);
    let track_y = y + 14.0;
    // track
    c.fill_rect(Rect { origin: Point { x, y: track_y }, size: Size { width: w, height: 6.0 } }, Color::rgb(50,52,72));
    // fill
    c.fill_rect(Rect { origin: Point { x, y: track_y }, size: Size { width: w * pct, height: 6.0 } }, color);
    // end cap circle
    c.fill_circle(Point { x: x + w * pct, y: track_y + 3.0 }, 5.0, color);
}

fn main() {
    // Reactive state — follow count toggles followed/unfollowed
    let follow_count = use_atom(4_821_u32);
    let is_followed  = use_atom(false);

    // Simulate a follow action
    is_followed.set(true);
    follow_count.update(|n| n + 1);

    let follows = follow_count.get();
    let followed = is_followed.get();

    let font = FontCache::system_mono().expect("no system font");

    let mut c = SkiaCanvas::new(W, H);
    c.clear(BG);

    // ── Cover banner ────────────────────────────────────────────────────────
    // Gradient-like effect via two stacked rects
    c.fill_rect(Rect { origin: Point { x: 0.0, y: 0.0 }, size: Size { width: W as f32, height: 160.0 } }, Color::rgb(30, 40, 80));
    c.fill_rect(Rect { origin: Point { x: 0.0, y: 80.0 }, size: Size { width: W as f32, height: 80.0 } }, Color::rgb(20, 28, 55));
    // Decorative stripe
    for i in 0..8u32 {
        c.fill_rect(
            Rect { origin: Point { x: i as f32 * 54.0 - 20.0, y: 0.0 }, size: Size { width: 8.0, height: 160.0 } },
            Color::rgba(100, 140, 255, 18),
        );
    }

    // ── Avatar ───────────────────────────────────────────────────────────────
    let av_cx = W as f32 / 2.0;
    let av_cy = 148.0_f32;
    let av_r  = 46.0_f32;
    // ring
    c.fill_circle(Point { x: av_cx, y: av_cy }, av_r + 4.0, ACCENT);
    // avatar fill
    c.fill_circle(Point { x: av_cx, y: av_cy }, av_r, Color::rgb(60, 70, 120));
    // face placeholder
    c.fill_circle(Point { x: av_cx, y: av_cy - 8.0 }, 14.0, Color::rgb(200, 170, 130));
    c.fill_circle(Point { x: av_cx, y: av_cy + 26.0 }, 18.0, Color::rgb(200, 170, 130));

    // Online indicator
    c.fill_circle(Point { x: av_cx + av_r * 0.68, y: av_cy + av_r * 0.68 }, 8.0, BG);
    c.fill_circle(Point { x: av_cx + av_r * 0.68, y: av_cy + av_r * 0.68 }, 5.5, GREEN);

    // ── Name and handle ──────────────────────────────────────────────────────
    let name  = "Alexandra Kim";
    let handle = "@alexkim  •  Senior Engineer";
    // Center using monospace glyph-width approximation
    let name_w   = name.len()   as f32 * (17.0 * 0.55);
    let handle_w = handle.len() as f32 * (11.0 * 0.55);
    c.draw_text(name,   Point { x: (W as f32 - name_w)   / 2.0, y: 205.0 }, TEXT_PRIMARY, &font, 17.0);
    c.draw_text(handle, Point { x: (W as f32 - handle_w) / 2.0, y: 228.0 }, TEXT_MUTED,   &font, 11.0);

    // ── Stats row (followers / posts / projects) ─────────────────────────────
    let stat_sizes = vec![
        Size { width: 110.0, height: 60.0 },
        Size { width: 110.0, height: 60.0 },
        Size { width: 110.0, height: 60.0 },
    ];
    let stat_con = Constraints::loose(W as f32 - 40.0, 60.0);
    let stat_layout = Row::new().spacing(10.0).layout(stat_con, &stat_sizes);

    let follow_str = format!("{follows}");
    let stat_data = [
        (follow_str.as_str(), "Followers", ACCENT),
        ("312",               "Following", ORANGE),
        ("48",                "Projects",  GREEN),
    ];
    for (i, (val, lbl, color)) in stat_data.iter().enumerate() {
        let p = stat_layout.child_positions[i];
        stat_card(&mut c, &font, 20.0 + p.x, 254.0 + p.y, 110.0, 60.0, val, lbl, *color);
    }

    // ── Follow / Message buttons ─────────────────────────────────────────────
    let btn_y = 330.0_f32;
    let btn_color = if followed { ACCENT_DIM } else { ACCENT };
    let follow_label = if followed { "Following  \u{2713}" } else { "Follow" };
    c.fill_rect(Rect { origin: Point { x: 20.0, y: btn_y }, size: Size { width: 170.0, height: 36.0 } }, btn_color);
    let fl_w = follow_label.len() as f32 * (12.0 * 0.55);
    c.draw_text(follow_label, Point { x: 20.0 + (170.0 - fl_w) / 2.0, y: btn_y + 12.0 }, Color::WHITE, &font, 12.0);

    c.stroke_rect(Rect { origin: Point { x: 204.0, y: btn_y }, size: Size { width: 170.0, height: 36.0 } }, ACCENT, 1.5);
    let msg = "Message";
    let msg_w = msg.len() as f32 * (12.0 * 0.55);
    c.draw_text(msg, Point { x: 204.0 + (170.0 - msg_w) / 2.0, y: btn_y + 12.0 }, ACCENT, &font, 12.0);

    // ── Bio section ──────────────────────────────────────────────────────────
    let bio_y = 384.0_f32;
    c.fill_rect(Rect { origin: Point { x: 20.0, y: bio_y }, size: Size { width: W as f32 - 40.0, height: 1.0 } }, CARD_BORDER);
    c.draw_text("About", Point { x: 20.0, y: bio_y + 10.0 }, ACCENT, &font, 13.0);

    let bio_lines = [
        "Building next-gen UI frameworks in Rust.",
        "Open source contributor  •  Speaker  •  Mentor",
        "San Francisco, CA  •  he/him",
    ];
    for (i, line) in bio_lines.iter().enumerate() {
        c.draw_text(line, Point { x: 20.0, y: bio_y + 30.0 + i as f32 * 18.0 }, TEXT_MUTED, &font, 11.0);
    }

    // ── Skills section ────────────────────────────────────────────────────────
    let sk_y = bio_y + 100.0;
    c.fill_rect(Rect { origin: Point { x: 20.0, y: sk_y - 4.0 }, size: Size { width: W as f32 - 40.0, height: 1.0 } }, CARD_BORDER);
    c.draw_text("Skills", Point { x: 20.0, y: sk_y + 8.0 }, ACCENT, &font, 13.0);

    let col_layout = Column::new()
        .spacing(22.0)
        .layout(
            Constraints::loose(W as f32 - 40.0, 200.0),
            &vec![Size { width: W as f32 - 40.0, height: 20.0 }; 4],
        );

    let skills = [
        ("Rust",         0.92, ACCENT),
        ("UI/UX Design", 0.80, Color::rgb(200, 100, 255)),
        ("WebAssembly",  0.74, GREEN),
        ("Graphics API", 0.65, ORANGE),
    ];
    for (i, (name, pct, color)) in skills.iter().enumerate() {
        let p = col_layout.child_positions[i];
        skill_bar(&mut c, &font, 20.0, sk_y + 30.0 + p.y, W as f32 - 56.0, name, *pct, *color);
    }

    // ── Footer tag ────────────────────────────────────────────────────────────
    let footer = "TEZZERA UI  •  Profile Card Example";
    let footer_w = footer.len() as f32 * (10.0 * 0.55);
    c.draw_text(footer, Point { x: (W as f32 - footer_w) / 2.0, y: H as f32 - 18.0 }, Color::rgba(120,120,160,150), &font, 10.0);

    let png = c.encode_png().expect("encode png");
    std::fs::write("profile_card.png", png).expect("write png");
    println!("Saved  profile_card.png  ({W}x{H})  followed={followed}  followers={follows}");
}
