//! Dashboard — analytics overview with stat cards, bar chart, and activity feed.
//!
//! Demonstrates: Row + Column + Stack layout, layered drawing, reactive state.
//! Run:   cargo run -p tezzera-examples --bin dashboard
//! Output: dashboard.png (900×600)

use tezzera_core::types::{Point, Rect, Size};
use tezzera_layout::{Column, Constraints, Row};
use tezzera_render::{Color, FontCache, SkiaCanvas};
use tezzera_state::use_atom;

const W: u32 = 900;
const H: u32 = 600;

const BG:          Color = Color::rgb(15,  16,  26);
const SIDEBAR_BG:  Color = Color::rgb(20,  22,  36);
const CARD_BG:     Color = Color::rgb(26,  28,  44);
const CARD_BORDER: Color = Color::rgb(44,  48,  72);
const HEADER_BG:   Color = Color::rgb(22,  24,  38);
const TEXT_HI:     Color = Color::rgb(240, 242, 255);
const TEXT_LO:     Color = Color::rgb(110, 115, 150);
const ACCENT:      Color = Color::rgb(100, 160, 255);
const GREEN:       Color = Color::rgb( 72, 199, 116);
const RED:         Color = Color::rgb(255,  90,  80);
const ORANGE:      Color = Color::rgb(255, 165,  55);
const PURPLE:      Color = Color::rgb(180, 100, 255);

fn card(c: &mut SkiaCanvas, x: f32, y: f32, w: f32, h: f32) {
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, CARD_BG);
    c.stroke_rect(Rect { origin: Point { x, y }, size: Size { width: w, height: h } }, CARD_BORDER, 1.0);
}

fn stat_tile(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32,
             label: &str, value: &str, delta: &str, positive: bool, accent: Color) {
    card(c, x, y, w, h);
    // left accent bar
    c.fill_rect(Rect { origin: Point { x, y }, size: Size { width: 3.0, height: h } }, accent);
    c.draw_text(label, Point { x: x + 14.0, y: y + 12.0 }, TEXT_LO, font, 10.0);
    c.draw_text(value, Point { x: x + 14.0, y: y + 28.0 }, TEXT_HI, font, 17.0);
    let delta_color = if positive { GREEN } else { RED };
    c.draw_text(delta, Point { x: x + 14.0, y: y + 51.0 }, delta_color, font, 10.0);
    // mini sparkline (decorative)
    let spark_heights: [f32; 8] = [20.0, 28.0, 18.0, 32.0, 24.0, 35.0, 28.0, 38.0];
    let sx = x + w - 68.0;
    let sy = y + h - 8.0;
    for (i, &sh) in spark_heights.iter().enumerate() {
        c.fill_rect(
            Rect { origin: Point { x: sx + i as f32 * 8.0, y: sy - sh }, size: Size { width: 5.0, height: sh } },
            Color::rgba(accent.r, accent.g, accent.b, 80),
        );
    }
}

fn bar_chart(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32) {
    card(c, x, y, w, h);
    c.draw_text("Revenue  (last 12 months)", Point { x: x + 16.0, y: y + 14.0 }, TEXT_HI, font, 13.0);
    c.draw_text("USD thousands", Point { x: x + 16.0, y: y + 32.0 }, TEXT_LO, font, 10.0);

    let months = ["J","F","M","A","M","J","J","A","S","O","N","D"];
    let values: [f32; 12] = [42.0, 55.0, 48.0, 63.0, 71.0, 68.0, 82.0, 75.0, 90.0, 88.0, 95.0, 110.0];
    let max_v = 120.0_f32;
    let chart_h = h - 72.0;
    let chart_y = y + 52.0;
    let bw = (w - 32.0) / 12.0;

    // Gridlines
    for g in 0..5u32 {
        let gy = chart_y + chart_h - (g as f32 / 4.0) * chart_h;
        c.fill_rect(Rect { origin: Point { x: x + 16.0, y: gy }, size: Size { width: w - 32.0, height: 1.0 } }, Color::rgba(80,84,110,80));
    }

    for (i, (&v, &month)) in values.iter().zip(months.iter()).enumerate() {
        let bh = (v / max_v) * chart_h;
        let bx = x + 16.0 + i as f32 * bw + 2.0;
        let by = chart_y + chart_h - bh;
        // Bar gradient (top brighter)
        c.fill_rect(Rect { origin: Point { x: bx, y: by }, size: Size { width: bw - 4.0, height: bh * 0.4 } }, ACCENT);
        c.fill_rect(Rect { origin: Point { x: bx, y: by + bh * 0.4 }, size: Size { width: bw - 4.0, height: bh * 0.6 } }, Color::rgb(55, 100, 190));
        // Month label — center under bar using monospace glyph-width approximation
        let lw = month.len() as f32 * (9.0 * 0.55);
        c.draw_text(month, Point { x: bx + (bw - 4.0 - lw) / 2.0, y: chart_y + chart_h + 4.0 }, TEXT_LO, font, 9.0);
    }
}

fn activity_feed(c: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32, w: f32, h: f32) {
    card(c, x, y, w, h);
    c.draw_text("Live Activity", Point { x: x + 14.0, y: y + 14.0 }, TEXT_HI, font, 13.0);

    let events = [
        ("New user signup",       "2m ago",  GREEN),
        ("Payment processed",     "5m ago",  ACCENT),
        ("Deploy completed",      "11m ago", PURPLE),
        ("Alert: high CPU",       "18m ago", RED),
        ("Backup finished",       "34m ago", ORANGE),
        ("New user signup",       "41m ago", GREEN),
        ("API rate limit hit",    "52m ago", RED),
        ("Report generated",      "1h ago",  ACCENT),
    ];

    let item_sizes = vec![Size { width: w - 28.0, height: 30.0 }; events.len()];
    let col_con = Constraints::loose(w - 28.0, h - 40.0);
    let layout = Column::new().spacing(2.0).layout(col_con, &item_sizes);

    for (i, ((evt, time, dot_color), &pos)) in events.iter()
        .zip(layout.child_positions.iter())
        .enumerate()
    {
        let ey = y + 36.0 + pos.y;
        // Alternating row bg
        if i % 2 == 0 {
            c.fill_rect(Rect { origin: Point { x: x + 1.0, y: ey }, size: Size { width: w - 2.0, height: 30.0 } }, Color::rgba(255,255,255,6));
        }
        c.fill_circle(Point { x: x + 21.0, y: ey + 15.0 }, 5.0, *dot_color);
        c.draw_text(evt,  Point { x: x + 34.0, y: ey + 9.0 },  TEXT_HI, font, 11.0);
        let time_x = x + w - time.len() as f32 * (9.0 * 0.55) - 10.0;
        c.draw_text(time, Point { x: time_x, y: ey + 11.0 }, TEXT_LO, font, 9.0);
    }
}

fn sidebar(c: &mut SkiaCanvas, font: &FontCache) {
    let sw = 180.0_f32;
    c.fill_rect(Rect { origin: Point { x: 0.0, y: 0.0 }, size: Size { width: sw, height: H as f32 } }, SIDEBAR_BG);
    c.fill_rect(Rect { origin: Point { x: sw - 1.0, y: 0.0 }, size: Size { width: 1.0, height: H as f32 } }, CARD_BORDER);

    // Logo area
    c.fill_circle(Point { x: 30.0, y: 36.0 }, 12.0, ACCENT);
    c.draw_text("TEZZERA", Point { x: 50.0, y: 28.0 }, TEXT_HI, font, 14.0);

    // Nav items
    let items = [
        ("Dashboard", true,  ACCENT),
        ("Analytics", false, TEXT_LO),
        ("Users",     false, TEXT_LO),
        ("Revenue",   false, TEXT_LO),
        ("Settings",  false, TEXT_LO),
    ];
    for (i, (label, active, color)) in items.iter().enumerate() {
        let iy = 80.0 + i as f32 * 44.0;
        if *active {
            c.fill_rect(Rect { origin: Point { x: 0.0, y: iy - 2.0 }, size: Size { width: sw, height: 28.0 } }, Color::rgba(100,160,255,20));
            c.fill_rect(Rect { origin: Point { x: 0.0, y: iy - 2.0 }, size: Size { width: 3.0, height: 28.0 } }, ACCENT);
        }
        c.draw_text(label, Point { x: 22.0, y: iy + 6.0 }, *color, font, 12.0);
    }

    // Bottom user chip
    let uy = H as f32 - 60.0;
    c.fill_rect(Rect { origin: Point { x: 10.0, y: uy }, size: Size { width: sw - 20.0, height: 44.0 } }, Color::rgba(255,255,255,8));
    c.fill_circle(Point { x: 28.0, y: uy + 22.0 }, 14.0, ACCENT);
    c.draw_text("Admin User",       Point { x: 48.0, y: uy + 10.0 }, TEXT_HI, font, 11.0);
    c.draw_text("admin@tezzera.io", Point { x: 48.0, y: uy + 25.0 }, TEXT_LO, font,  9.0);
}

fn main() {
    // Reactive state for key metrics
    let page_views  = use_atom(1_284_930_u64);
    let active_users = use_atom(4_821_u32);
    let revenue      = use_atom(98_420_u32);
    let error_rate   = use_atom(0.12_f32);

    // Simulate data arriving
    page_views.update(|n| n + 1_200);
    active_users.update(|n| n + 3);

    let pv = page_views.get();
    let au = active_users.get();
    let rv = revenue.get();
    let er = error_rate.get();

    let font = FontCache::system_mono().expect("no system font");

    let mut c = SkiaCanvas::new(W, H);
    c.clear(BG);

    // Sidebar
    sidebar(&mut c, &font);

    let cx = 190.0_f32; // content x
    let cw = W as f32 - cx - 10.0;

    // ── Top header bar ───────────────────────────────────────────────────────
    c.fill_rect(Rect { origin: Point { x: cx, y: 0.0 }, size: Size { width: cw, height: 48.0 } }, HEADER_BG);
    c.draw_text("Overview",       Point { x: cx + 12.0,           y: 16.0 }, TEXT_HI, &font, 14.0);
    c.draw_text("Last 30 days  v", Point { x: W as f32 - 130.0,   y: 18.0 }, TEXT_LO, &font, 11.0);
    c.fill_circle(Point { x: W as f32 - 24.0, y: 24.0 }, 7.0, GREEN); // status dot

    // ── Stat tiles ───────────────────────────────────────────────────────────
    let tile_sizes = vec![Size { width: 152.0, height: 76.0 }; 4];
    let tile_con = Constraints::loose(cw - 16.0, 76.0);
    let tile_layout = Row::new().spacing(8.0).layout(tile_con, &tile_sizes);

    let pv_str = format!("{:.2}M", pv as f64 / 1_000_000.0);
    let au_str = format!("{au}");
    let rv_str = format!("${rv}");
    let er_str = format!("{er:.2}%");

    let tiles = [
        ("Page Views",    pv_str.as_str(),  "+12.4%  this month", true,  ACCENT),
        ("Active Users",  au_str.as_str(),  "+3.1%   vs last wk", true,  GREEN),
        ("Revenue",       rv_str.as_str(),  "+8.7%   vs last wk", true,  ORANGE),
        ("Error Rate",    er_str.as_str(),  "-0.05%  improving",  false, RED),
    ];
    for (i, (lbl, val, delta, pos, color)) in tiles.iter().enumerate() {
        let p = tile_layout.child_positions[i];
        stat_tile(&mut c, &font, cx + 8.0 + p.x, 56.0 + p.y, 152.0, 76.0, lbl, val, delta, *pos, *color);
    }

    // ── Bar chart (left 60%) + activity feed (right 40%) ────────────────────
    let section_y = 144.0_f32;
    let section_h = H as f32 - section_y - 10.0;
    let chart_w   = cw * 0.60 - 6.0;
    let feed_w    = cw - chart_w - 14.0;

    bar_chart(&mut c, &font, cx + 8.0, section_y, chart_w, section_h);
    activity_feed(&mut c, &font, cx + 8.0 + chart_w + 6.0, section_y, feed_w, section_h);

    let png = c.encode_png().expect("encode png");
    std::fs::write("dashboard.png", png).expect("write png");
    println!("Saved  dashboard.png  ({W}x{H})  views={pv}  users={au}");
}
