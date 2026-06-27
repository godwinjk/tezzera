use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use tezzera_core::types::{Point, Rect, Size};

/// TEZZERA's 2D drawing canvas backed by tiny-skia.
///
/// Replaces the placeholder `Canvas` in `tezzera-core` for the Phase 1 desktop
/// target. All drawing operations are performed on a CPU pixel buffer; no native
/// graphics library is required.
pub struct SkiaCanvas {
    pixmap: Pixmap,
}

/// An RGBA color value.
#[derive(Debug, Clone, Copy)]
pub struct Color {
    /// Red channel (0–255).
    pub r: u8,
    /// Green channel (0–255).
    pub g: u8,
    /// Blue channel (0–255).
    pub b: u8,
    /// Alpha channel (0–255).
    pub a: u8,
}

impl Color {
    /// Create an opaque color from red, green, and blue components.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    /// Create a color with explicit alpha.
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Opaque white.
    pub const WHITE: Color = Color::rgb(255, 255, 255);
    /// Opaque black.
    pub const BLACK: Color = Color::rgb(0, 0, 0);
    /// Opaque red.
    pub const RED: Color = Color::rgb(255, 0, 0);
    /// Opaque green.
    pub const GREEN: Color = Color::rgb(0, 255, 0);
    /// Opaque blue.
    pub const BLUE: Color = Color::rgb(0, 0, 255);
    /// Fully transparent.
    pub const TRANSPARENT: Color = Color::rgba(0, 0, 0, 0);
}

impl SkiaCanvas {
    /// Create a new canvas with the given pixel dimensions.
    ///
    /// # Panics
    /// Panics if either dimension is zero.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            pixmap: Pixmap::new(width, height).expect("failed to create pixmap"),
        }
    }

    /// Canvas width in pixels.
    pub fn width(&self) -> u32 {
        self.pixmap.width()
    }

    /// Canvas height in pixels.
    pub fn height(&self) -> u32 {
        self.pixmap.height()
    }

    /// Fill the entire canvas with a solid color.
    pub fn clear(&mut self, color: Color) {
        self.pixmap.fill(
            tiny_skia::Color::from_rgba8(color.r, color.g, color.b, color.a),
        );
    }

    /// Fill a rectangle with a solid color.
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        let mut paint = Paint::default();
        paint.set_color_rgba8(color.r, color.g, color.b, color.a);
        paint.anti_alias = true;
        let r = tiny_skia::Rect::from_xywh(
            rect.origin.x,
            rect.origin.y,
            rect.size.width,
            rect.size.height,
        );
        if let Some(r) = r {
            self.pixmap
                .fill_rect(r, &paint, Transform::identity(), None);
        }
    }

    /// Draw a rectangle outline with the given stroke width.
    pub fn stroke_rect(&mut self, rect: Rect, color: Color, stroke_width: f32) {
        let mut paint = Paint::default();
        paint.set_color_rgba8(color.r, color.g, color.b, color.a);
        paint.anti_alias = true;
        let Some(skia_rect) = tiny_skia::Rect::from_xywh(
            rect.origin.x,
            rect.origin.y,
            rect.size.width,
            rect.size.height,
        ) else {
            return;
        };
        let path = PathBuilder::from_rect(skia_rect);
        let stroke = tiny_skia::Stroke {
            width: stroke_width,
            ..Default::default()
        };
        self.pixmap
            .stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }

    /// Draw a filled circle centered at `center` with the given `radius`.
    pub fn fill_circle(&mut self, center: Point, radius: f32, color: Color) {
        let mut paint = Paint::default();
        paint.set_color_rgba8(color.r, color.g, color.b, color.a);
        paint.anti_alias = true;
        let mut pb = PathBuilder::new();
        pb.push_circle(center.x, center.y, radius);
        if let Some(path) = pb.finish() {
            self.pixmap.fill_path(
                &path,
                &paint,
                FillRule::Winding,
                Transform::identity(),
                None,
            );
        }
    }

    /// Draw a text placeholder at `origin`.
    ///
    /// Real text rendering requires font integration, which is planned for Phase 2.
    /// For Phase 1, this draws a colored rectangle whose width is proportional to
    /// the text length.
    pub fn draw_text_placeholder(&mut self, text: &str, origin: Point, color: Color) {
        let width = text.len() as f32 * 8.0;
        let height = 16.0;
        self.fill_rect(
            Rect {
                origin,
                size: Size { width, height },
            },
            color,
        );
    }

    /// Draw real text glyphs at `origin` using `font` at `px` size.
    ///
    /// Each character is rasterized individually and blended onto the canvas.
    /// This replaces `draw_text_placeholder` for Phase 2.
    pub fn draw_text(&mut self, text: &str, origin: Point, color: Color, font: &crate::font::FontCache, px: f32) {
        let mut cursor_x = origin.x;
        for ch in text.chars() {
            let (metrics, bitmap) = font.rasterize(ch, px);
            if metrics.width == 0 || metrics.height == 0 {
                cursor_x += px * 0.3;
                continue;
            }
            for row in 0..metrics.height {
                for col in 0..metrics.width {
                    let coverage = bitmap[row * metrics.width + col];
                    if coverage == 0 { continue; }
                    let px_x = (cursor_x as i32 + col as i32 + metrics.xmin) as u32;
                    let px_y = (origin.y as i32 + row as i32 - metrics.ymin) as u32;
                    if px_x >= self.pixmap.width() || px_y >= self.pixmap.height() { continue; }
                    let alpha = (coverage as u32 * color.a as u32 / 255) as u8;
                    let mut paint = tiny_skia::Paint::default();
                    paint.set_color_rgba8(color.r, color.g, color.b, alpha);
                    paint.anti_alias = false;
                    if let Some(r) = tiny_skia::Rect::from_xywh(px_x as f32, px_y as f32, 1.0, 1.0) {
                        self.pixmap.fill_rect(r, &paint, tiny_skia::Transform::identity(), None);
                    }
                }
            }
            cursor_x += metrics.advance_width;
        }
    }

    /// Returns the raw RGBA pixel data as a byte slice.
    pub fn pixels(&self) -> &[u8] {
        self.pixmap.data()
    }

    /// Encode the canvas contents as a PNG byte vector, returning `None` on error.
    pub fn encode_png(&self) -> Option<Vec<u8>> {
        self.pixmap.encode_png().ok()
    }
}
