//! [`TextInput`] — a single-line text input widget with cursor rendering.

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::canvas::{Color, SkiaCanvas};
use tezzera_render::FontCache;

/// A single-line text input field.
///
/// Renders a background, a border (accent-colored when focused), the current
/// value (or placeholder), and a blinking-cursor indicator when focused.
pub struct TextInput {
    pub value: String,
    pub placeholder: String,
    pub width: f32,
    pub height: f32,
    /// When `true`, the value is displayed as bullet characters (`•`).
    pub obscure: bool,
    /// When `true`, the field draws an accent border and a cursor glyph.
    pub focused: bool,
}

impl TextInput {
    /// Creates a new [`TextInput`] with default placeholder text.
    pub fn new() -> Self {
        Self {
            value: String::new(),
            placeholder: "Type here\u{2026}".into(),
            width: 200.0,
            height: 36.0,
            obscure: false,
            focused: false,
        }
    }

    // ── Builder methods ───────────────────────────────────────────────────────

    /// Sets the current text value.
    pub fn value(mut self, v: impl Into<String>) -> Self {
        self.value = v.into();
        self
    }

    /// Sets the placeholder string shown when the value is empty.
    pub fn placeholder(mut self, p: impl Into<String>) -> Self {
        self.placeholder = p.into();
        self
    }

    /// Sets the field width in pixels.
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// When `true`, the value characters are replaced with `•` glyphs.
    pub fn obscure(mut self, o: bool) -> Self {
        self.obscure = o;
        self
    }

    /// When `true`, renders an accent border and a cursor indicator.
    pub fn focused(mut self, f: bool) -> Self {
        self.focused = f;
        self
    }

    // ── Layout ───────────────────────────────────────────────────────────────

    /// Returns the preferred (width, height) of this field.
    pub fn preferred_size(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    // ── Display helpers (pure, no canvas) ────────────────────────────────────

    /// Returns the string that should be displayed in the field (after applying
    /// `obscure`) and the color it should be drawn in.
    ///
    /// This is a pure helper used by both `render` and tests.
    pub fn display_text(&self) -> (String, Color) {
        if self.value.is_empty() {
            (self.placeholder.clone(), Color::rgba(140, 145, 175, 160))
        } else if self.obscure {
            ("\u{2022}".repeat(self.value.chars().count()), Color::rgb(240, 242, 255))
        } else {
            (self.value.clone(), Color::rgb(240, 242, 255))
        }
    }

    // ── Rendering ────────────────────────────────────────────────────────────

    /// Draws the input field onto `canvas` at `(x, y)` using `font` for glyphs.
    pub fn render(&self, canvas: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32) {
        let rect = Rect {
            origin: Point { x, y },
            size: Size { width: self.width, height: self.height },
        };

        // Background
        canvas.fill_rect(rect, Color::rgb(28, 30, 46));

        // Border — accent when focused
        let border = if self.focused {
            Color::rgb(100, 160, 255)
        } else {
            Color::rgb(55, 60, 90)
        };
        canvas.stroke_rect(rect, border, 1.5);

        // Text
        let font_size = 14.0_f32;
        let (text, text_color) = self.display_text();
        let ty = y + (self.height - font_size) / 2.0;
        canvas.draw_text(&text, Point { x: x + 10.0, y: ty }, text_color, font, font_size);

        // Cursor
        if self.focused {
            let cursor_x = x + 10.0 + text.chars().count() as f32 * font_size * 0.55;
            canvas.fill_rect(
                Rect {
                    origin: Point { x: cursor_x, y: y + 8.0 },
                    size: Size { width: 1.5, height: self.height - 16.0 },
                },
                Color::rgb(100, 160, 255),
            );
        }
    }
}

impl Default for TextInput {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_input_default_value_is_empty() {
        let ti = TextInput::new();
        assert!(ti.value.is_empty());
    }

    #[test]
    fn text_input_obscure_shows_bullets() {
        let ti = TextInput::new().value("abc").obscure(true);
        let (display, _) = ti.display_text();
        assert_eq!(display, "\u{2022}\u{2022}\u{2022}");
        assert!(!display.contains('a'));
    }

    #[test]
    fn text_input_no_obscure_shows_value() {
        let ti = TextInput::new().value("hello").obscure(false);
        let (display, _) = ti.display_text();
        assert_eq!(display, "hello");
    }

    #[test]
    fn text_input_empty_shows_placeholder() {
        let ti = TextInput::new().placeholder("Enter name");
        let (display, _) = ti.display_text();
        assert_eq!(display, "Enter name");
    }
}
