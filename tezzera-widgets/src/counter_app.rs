//! Phase 1 integration demo: a counter that increments on each "click".
//!
//! This is not a real windowed app yet (that requires `tezzera-cli` + winit).
//! It renders frames to a pixel buffer using [`RenderPipeline`], which proves
//! the full stack works: state → layout → render → pixels.

use tezzera_core::types::{Point, Rect, Size};
use tezzera_layout::{Column, Constraints};
use tezzera_render::{Color, RenderPipeline, SkiaCanvas};
use tezzera_state::use_atom;

/// Renders one frame of the counter app with the given `count` value.
///
/// Paints a white background, a text label showing the count, and a blue
/// "Increment" button — all positioned by the [`Column`] layout engine.
///
/// Returns the raw RGBA pixel buffer (`width × height × 4` bytes).
pub fn render_counter_frame(count: i32, width: u32, height: u32) -> Vec<u8> {
    let mut pipeline = RenderPipeline::new(width, height);
    pipeline.mark_dirty();

    let label = format!("Count: {}", count);
    let button_label = "Increment";

    let result = pipeline.render_frame(|canvas: &mut SkiaCanvas| {
        // Background
        canvas.clear(Color::WHITE);

        // Layout: column with text + button
        let col = Column::new().spacing(16.0);
        let text_size = Size { width: label.len() as f32 * 8.0, height: 16.0 };
        let btn_size = Size { width: button_label.len() as f32 * 8.0 + 16.0, height: 32.0 };
        let child_sizes = vec![text_size, btn_size];

        let constraints = Constraints::loose(width as f32, height as f32);
        let layout = col.layout(constraints, &child_sizes);

        // Paint text
        let text_pos = layout.child_positions[0];
        let margin = Point { x: 20.0 + text_pos.x, y: 20.0 + text_pos.y };
        canvas.draw_text_placeholder(&label, margin, Color::BLACK);

        // Paint button
        let btn_pos = layout.child_positions[1];
        let btn_origin = Point { x: 20.0 + btn_pos.x, y: 20.0 + btn_pos.y };
        canvas.fill_rect(
            Rect { origin: btn_origin, size: btn_size },
            Color::rgb(70, 130, 200),
        );
        canvas.draw_text_placeholder(
            button_label,
            Point { x: btn_origin.x + 8.0, y: btn_origin.y + 8.0 },
            Color::WHITE,
        );
    });

    result.to_vec()
}

/// Simulates the counter app lifecycle: mount → increment × `increments` →
/// return the final count value.
///
/// Uses [`use_atom`] to create a fresh reactive atom, applies `increments`
/// updates, and returns the final value.
pub fn run_counter_simulation(increments: u32) -> i32 {
    let count = use_atom(0i32);
    for _ in 0..increments {
        count.update(|n| n + 1);
    }
    count.get()
}
