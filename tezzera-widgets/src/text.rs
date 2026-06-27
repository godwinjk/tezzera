//! [`Text`] — a plain-text leaf widget.

use tezzera_core::{element::Element, types::Size};
use tezzera_core::types::Point;
use tezzera_render::canvas::{Color, SkiaCanvas};

/// A widget that displays a string of text.
///
/// In Phase 1 the text is rendered as a colored rectangle whose width is
/// proportional to the number of characters (8 px per character, 16 px tall).
/// Full font-based rendering is planned for Phase 2.
pub struct Text {
    content: String,
    color: Color,
}

impl Text {
    /// Creates a new [`Text`] widget with the given content and black color.
    pub fn new(content: impl Into<String>) -> Self {
        Self { content: content.into(), color: Color::BLACK }
    }

    /// Sets the color used to render the text placeholder.
    pub fn color(mut self, c: Color) -> Self {
        self.color = c;
        self
    }

    /// Returns the approximate natural size: 8 px per character wide, 16 px tall.
    pub fn natural_size(&self) -> Size {
        Size { width: self.content.len() as f32 * 8.0, height: 16.0 }
    }

    /// Paints this text onto `canvas` at the given `origin`.
    pub fn paint(&self, canvas: &mut SkiaCanvas, origin: Point) {
        canvas.draw_text_placeholder(&self.content, origin, self.color);
    }
}

impl From<Text> for Element {
    fn from(t: Text) -> Element {
        Element::text(t.content)
    }
}
