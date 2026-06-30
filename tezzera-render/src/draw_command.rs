use std::sync::Arc;

use tezzera_core::types::{Point, Rect};

use crate::canvas::Color;

/// A single drawing instruction recorded during the paint pass.
///
/// Widgets push these into a [`PictureRecorder`] instead of writing pixels
/// directly. The compositor later replays them onto whatever backend is active
/// (currently [`SkiaCanvas`], eventually wgpu).
#[derive(Debug, Clone)]
pub enum DrawCommand {
    FillRect   { rect: Rect, color: Color },
    StrokeRect { rect: Rect, color: Color, width: f32 },
    /// Filled rounded rectangle approximated as rects + corner circles.
    FillRRect  { rect: Rect, radius: f32, color: Color },
    FillCircle { center: Point, radius: f32, color: Color },
    DrawText   { text: String, origin: Point, color: Color, px: f32 },
    /// Multi-step offset shadow. Compositor expands into several FillRects.
    DrawShadow { rect: Rect, color: Color, blur: f32 },
    /// Raw pre-decoded RGBA pixel blit. `pixels` must be `width × height × 4` bytes.
    BlitRgba   { pixels: Arc<Vec<u8>>, src_width: u32, src_height: u32, dest_rect: Rect },
}
