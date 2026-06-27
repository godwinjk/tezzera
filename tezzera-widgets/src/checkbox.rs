//! [`Checkbox`] — a three-state checkbox with optional label.

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::canvas::{Color, SkiaCanvas};
use tezzera_render::FontCache;
use tezzera_theme::ThemeData;
use crate::theme_color_to_render as tc;

/// The checked / unchecked / indeterminate state of a [`Checkbox`].
#[derive(Debug, Clone, PartialEq)]
pub enum CheckState {
    Unchecked,
    Checked,
    Indeterminate,
}

/// A square checkbox that can be unchecked, checked, or indeterminate,
/// with an optional text label rendered to the right.
pub struct Checkbox {
    pub state: CheckState,
    pub size: f32,
    pub label: Option<String>,
    pub disabled: bool,
}

impl Checkbox {
    /// Creates a new [`Checkbox`] in the [`CheckState::Unchecked`] state.
    pub fn new() -> Self {
        Self {
            state: CheckState::Unchecked,
            size: 20.0,
            label: None,
            disabled: false,
        }
    }

    /// Sets the check state.
    pub fn state(mut self, s: CheckState) -> Self {
        self.state = s;
        self
    }

    /// Sets the size (width and height) of the checkbox box in pixels.
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }

    /// Attaches a text label shown to the right of the box.
    pub fn label(mut self, l: impl Into<String>) -> Self {
        self.label = Some(l.into());
        self
    }

    /// When `true`, the checkbox is rendered in a muted, non-interactive style.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    /// Paints the checkbox onto `canvas` at `(x, y)`.
    pub fn render(
        &self,
        canvas: &mut SkiaCanvas,
        font: &FontCache,
        x: f32,
        y: f32,
        theme: &ThemeData,
    ) {
        let border = if self.disabled {
            Color::rgba(120, 120, 120, 100)
        } else {
            tc(theme.colors.outline)
        };
        let s = self.size;

        // Outer border
        canvas.stroke_rect(
            Rect {
                origin: Point { x, y },
                size: Size { width: s, height: s },
            },
            border,
            2.0,
        );

        // Inner fill (checked or indeterminate only)
        if self.state != CheckState::Unchecked {
            let fill = tc(theme.colors.primary);
            let inner = s * 0.15;
            canvas.fill_rect(
                Rect {
                    origin: Point { x: x + inner, y: y + inner },
                    size: Size {
                        width: s - inner * 2.0,
                        height: s - inner * 2.0,
                    },
                },
                fill,
            );
        }

        // Checkmark glyph
        if self.state == CheckState::Checked {
            canvas.draw_text(
                "\u{2713}",
                Point { x: x + 3.0, y: y + 3.0 },
                Color::WHITE,
                font,
                s * 0.7,
            );
        }

        // Indeterminate dash
        if self.state == CheckState::Indeterminate {
            let mid = s / 2.0;
            canvas.fill_rect(
                Rect {
                    origin: Point { x: x + s * 0.2, y: y + mid - 1.5 },
                    size: Size { width: s * 0.6, height: 3.0 },
                },
                Color::WHITE,
            );
        }

        // Label
        if let Some(lbl) = &self.label {
            canvas.draw_text(
                lbl,
                Point { x: x + s + 8.0, y: y + 4.0 },
                tc(theme.colors.on_surface),
                font,
                s * 0.65,
            );
        }
    }

    /// Width of the checkbox box plus any label in pixels.
    pub fn width(&self) -> f32 {
        let lbl_w = self
            .label
            .as_ref()
            .map(|l| l.len() as f32 * self.size * 0.65 * 0.55 + 8.0)
            .unwrap_or(0.0);
        self.size + lbl_w
    }

    /// Height of the checkbox in pixels.
    pub fn height(&self) -> f32 {
        self.size
    }
}

impl Default for Checkbox {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checkbox_new_is_unchecked() {
        let cb = Checkbox::new();
        assert_eq!(cb.state, CheckState::Unchecked);
    }

    #[test]
    fn checkbox_checked_state() {
        let cb = Checkbox::new().state(CheckState::Checked);
        assert_eq!(cb.state, CheckState::Checked);
    }

    #[test]
    fn checkbox_indeterminate_state() {
        let cb = Checkbox::new().state(CheckState::Indeterminate);
        assert_eq!(cb.state, CheckState::Indeterminate);
    }

    #[test]
    fn checkbox_with_label_widens() {
        let no_label = Checkbox::new().size(20.0);
        let with_label = Checkbox::new().size(20.0).label("Accept terms");
        assert!(with_label.width() > no_label.width());
    }

    #[test]
    fn checkbox_height_equals_size() {
        let cb = Checkbox::new().size(24.0);
        assert_eq!(cb.height(), 24.0);
    }

    #[test]
    fn checkbox_default_is_unchecked() {
        let cb = Checkbox::default();
        assert_eq!(cb.state, CheckState::Unchecked);
        assert!(!cb.disabled);
    }

    #[test]
    fn checkbox_disabled_setter() {
        let cb = Checkbox::new().disabled(true);
        assert!(cb.disabled);
    }

    #[test]
    fn checkbox_size_setter() {
        let cb = Checkbox::new().size(32.0);
        assert_eq!(cb.size, 32.0);
        assert_eq!(cb.height(), 32.0);
    }
}
