use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Transform};
use tezzera_core::types::{Point, Rect, Size};

/// TEZZERA's 2D drawing canvas backed by tiny-skia.
///
/// Replaces the placeholder `Canvas` in `tezzera-core` for the Phase 1 desktop
/// target. All drawing operations are performed on a CPU pixel buffer; no native
/// graphics library is required.
pub struct SkiaCanvas {
    pixmap: Pixmap,
    /// Device pixel ratio (e.g. 2.0 on Retina). All draw coordinates are in
    /// logical pixels; `play_picture` multiplies them by this before writing
    /// physical pixels, so the full HiDPI buffer is used without blurry upscaling.
    scale: f32,
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
    /// Create a canvas at physical pixel size with a device pixel ratio of 1.0.
    pub fn new(width: u32, height: u32) -> Self {
        Self::new_hidpi(width, height, 1.0)
    }

    /// Create a canvas for a HiDPI display.
    ///
    /// `phys_width` / `phys_height` are the framebuffer dimensions in physical
    /// pixels. `scale` is the device pixel ratio (e.g. 2.0 on Retina).
    /// All draw coordinates passed via [`play_picture`] are in logical pixels
    /// and are multiplied by `scale` before writing to the pixmap.
    pub fn new_hidpi(phys_width: u32, phys_height: u32, scale: f32) -> Self {
        Self {
            pixmap: Pixmap::new(phys_width, phys_height).expect("failed to create pixmap"),
            scale: scale.max(1.0),
        }
    }

    /// Physical pixel width of the underlying framebuffer.
    pub fn width(&self) -> u32 {
        self.pixmap.width()
    }

    /// Physical pixel height of the underlying framebuffer.
    pub fn height(&self) -> u32 {
        self.pixmap.height()
    }

    /// Logical width (physical / scale). Use this for layout calculations.
    pub fn logical_width(&self) -> u32 {
        (self.pixmap.width() as f32 / self.scale).round() as u32
    }

    /// Logical height (physical / scale). Use this for layout calculations.
    pub fn logical_height(&self) -> u32 {
        (self.pixmap.height() as f32 / self.scale).round() as u32
    }

    /// Device pixel ratio for this canvas.
    pub fn scale(&self) -> f32 {
        self.scale
    }

    /// Fill the entire canvas with a solid color.
    pub fn clear(&mut self, color: Color) {
        self.pixmap.fill(
            tiny_skia::Color::from_rgba8(color.r, color.g, color.b, color.a),
        );
    }

    /// Fill the entire canvas with fully-transparent pixels (D078).
    ///
    /// Used to reset the overlay canvas before each frame's overlay paint pass
    /// so that closed or repositioned overlays do not persist.
    pub fn clear_transparent(&mut self) {
        self.pixmap.fill(tiny_skia::Color::TRANSPARENT);
    }

    /// Fill a rectangle with a solid color.
    pub fn fill_rect(&mut self, rect: Rect, color: Color) {
        // Skip transparent, degenerate, or invisible rects.
        if color.a == 0 { return; }
        if rect.size.width < 0.5 || rect.size.height < 0.5 { return; }
        let mut paint = Paint::default();
        paint.set_color_rgba8(color.r, color.g, color.b, color.a);
        paint.anti_alias = false;  // avoid hairline-AA panic on thin rects
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
        if color.a == 0 || radius < 0.5 { return; }
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
    /// `origin` is the top-left of the glyph bounding box (top-of-cap-height
    /// convention). Each character is rasterized and alpha-blended onto the canvas.
    pub fn draw_text(&mut self, text: &str, origin: Point, color: Color, font: &crate::font::FontCache, px: f32) {
        let canvas_w = self.pixmap.width();
        let canvas_h = self.pixmap.height();
        let mut cursor_x = origin.x;

        // origin.y is the top of the line box.
        // baseline_y = origin.y + ascender so every glyph shares the same baseline
        // regardless of individual bitmap height.
        let ascender = font.ascender(px);

        for ch in text.chars() {
            let (metrics, bitmap) = font.rasterize(ch, px);

            if metrics.width == 0 || metrics.height == 0 {
                cursor_x += metrics.advance_width;
                continue;
            }

            let mut paint = tiny_skia::Paint {
                anti_alias: false,
                blend_mode: tiny_skia::BlendMode::SourceOver,
                ..tiny_skia::Paint::default()
            };

            for row in 0..metrics.height {
                // baseline_y - (ymin + height) + row places every glyph on a
                // consistent baseline.  metrics.ymin is negative for descenders.
                let py = origin.y as i32 + ascender
                    - metrics.ymin
                    - metrics.height as i32
                    + row as i32;
                if py < 0 || py as u32 >= canvas_h { continue; }

                for col in 0..metrics.width {
                    let coverage = bitmap[row * metrics.width + col];
                    if coverage == 0 { continue; }

                    let px_xi = cursor_x as i32 + col as i32 + metrics.xmin;
                    if px_xi < 0 || px_xi as u32 >= canvas_w { continue; }

                    let alpha = (coverage as u32 * color.a as u32 / 255) as u8;
                    paint.set_color_rgba8(color.r, color.g, color.b, alpha);

                    if let Some(r) = tiny_skia::Rect::from_xywh(px_xi as f32, py as f32, 1.0, 1.0) {
                        self.pixmap.fill_rect(r, &paint, tiny_skia::Transform::identity(), None);
                    }
                }
            }

            cursor_x += metrics.advance_width;
        }
    }

    /// Fill a rounded rectangle using three overlapping rects and four corner circles.
    pub fn fill_rrect(&mut self, rect: Rect, radius: f32, color: Color) {
        if color.a == 0 { return; }
        let r = radius.min(rect.size.width / 2.0).min(rect.size.height / 2.0);
        if r < 0.5 {
            self.fill_rect(rect, color);
            return;
        }
        let x = rect.origin.x;
        let y = rect.origin.y;
        let w = rect.size.width;
        let h = rect.size.height;
        // Center horizontal band + center vertical band cover the body.
        self.fill_rect(Rect { origin: Point { x: x + r, y }, size: Size { width: w - r * 2.0, height: h } }, color);
        self.fill_rect(Rect { origin: Point { x, y: y + r }, size: Size { width: w, height: h - r * 2.0 } }, color);
        // Four corner arcs.
        self.fill_circle(Point { x: x + r,     y: y + r     }, r, color);
        self.fill_circle(Point { x: x + w - r, y: y + r     }, r, color);
        self.fill_circle(Point { x: x + r,     y: y + h - r }, r, color);
        self.fill_circle(Point { x: x + w - r, y: y + h - r }, r, color);
    }

    /// Replay a [`Picture`] (display list) onto this canvas.
    ///
    /// All draw-command coordinates are in **logical pixels**. They are
    /// multiplied by `self.scale` before writing to the physical pixmap, so
    /// the full HiDPI framebuffer resolution is used and there is no
    /// nearest-neighbour upscaling blur.
    pub fn play_picture(&mut self, picture: &crate::picture::Picture, font: &crate::font::FontCache) {
        use crate::draw_command::DrawCommand;
        let s = self.scale;
        let sr = |r: Rect| Rect {
            origin: Point { x: r.origin.x * s, y: r.origin.y * s },
            size:   Size  { width: r.size.width * s, height: r.size.height * s },
        };
        let sp = |p: Point| Point { x: p.x * s, y: p.y * s };

        for cmd in &picture.commands {
            match cmd {
                DrawCommand::FillRect { rect, color } => self.fill_rect(sr(*rect), *color),
                DrawCommand::StrokeRect { rect, color, width } => self.stroke_rect(sr(*rect), *color, *width * s),
                DrawCommand::FillRRect { rect, radius, color } => self.fill_rrect(sr(*rect), *radius * s, *color),
                DrawCommand::FillCircle { center, radius, color } => self.fill_circle(sp(*center), *radius * s, *color),
                DrawCommand::DrawText { text, origin, color, px } => {
                    self.draw_text(text, sp(*origin), *color, font, *px * s);
                }
                DrawCommand::DrawShadow { rect, color, blur } => {
                    let steps = (*blur as u32).min(8).max(1);
                    for i in 0..steps {
                        let alpha = (color.a as f32 * (1.0 - i as f32 / steps as f32) / steps as f32) as u8;
                        let spread = i as f32 * *blur / steps as f32 * s;
                        let scaled = sr(*rect);
                        let shifted = Rect {
                            origin: Point { x: scaled.origin.x + spread, y: scaled.origin.y + spread },
                            size: scaled.size,
                        };
                        self.fill_rect(shifted, Color::rgba(color.r, color.g, color.b, alpha));
                    }
                }
                DrawCommand::BlitRgba { pixels, src_width, src_height, dest_rect } => {
                    self.blit_rgba(pixels, *src_width, *src_height, sr(*dest_rect));
                }
            }
        }
    }

    /// Blit pre-decoded RGBA pixel data into `dest_rect`.
    ///
    /// `pixels` must be `src_width × src_height × 4` bytes (RGBA). The source
    /// is scaled to fill `dest_rect` using nearest-neighbour sampling. Pixels
    /// outside the canvas bounds are clipped silently.
    pub fn blit_rgba(&mut self, pixels: &[u8], src_w: u32, src_h: u32, dest: Rect) {
        let cw = self.pixmap.width() as i32;
        let ch = self.pixmap.height() as i32;
        let dst = self.pixmap.data_mut();

        let dx = dest.origin.x as i32;
        let dy = dest.origin.y as i32;
        let dw = dest.size.width as i32;
        let dh = dest.size.height as i32;

        for row in 0..dh {
            let src_row = (row * src_h as i32 / dh.max(1)) as u32;
            let py = dy + row;
            if py < 0 || py >= ch { continue; }
            for col in 0..dw {
                let src_col = (col * src_w as i32 / dw.max(1)) as u32;
                let px = dx + col;
                if px < 0 || px >= cw { continue; }
                let si = (src_row * src_w + src_col) as usize * 4;
                let di = (py * cw + px) as usize * 4;
                if si + 3 >= pixels.len() || di + 3 >= dst.len() { continue; }
                let alpha = pixels[si + 3] as u32;
                let inv = 255 - alpha;
                dst[di]     = ((pixels[si]     as u32 * alpha + dst[di]     as u32 * inv) / 255) as u8;
                dst[di + 1] = ((pixels[si + 1] as u32 * alpha + dst[di + 1] as u32 * inv) / 255) as u8;
                dst[di + 2] = ((pixels[si + 2] as u32 * alpha + dst[di + 2] as u32 * inv) / 255) as u8;
                dst[di + 3] = 255;
            }
        }
    }

    /// Returns the raw RGBA pixel data as a byte slice.
    pub fn pixels(&self) -> &[u8] {
        self.pixmap.data()
    }

    /// Returns the raw RGBA pixel data as a mutable byte slice.
    ///
    /// Callers can write directly into the pixel buffer to blit pre-decoded
    /// image data or apply custom compositing.
    pub fn pixels_mut(&mut self) -> &mut [u8] {
        self.pixmap.data_mut()
    }

    /// Encode the canvas contents as a PNG byte vector, returning `None` on error.
    pub fn encode_png(&self) -> Option<Vec<u8>> {
        self.pixmap.encode_png().ok()
    }
}
