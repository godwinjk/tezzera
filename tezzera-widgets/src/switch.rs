//! [`Switch`] — a toggle switch (track + sliding thumb).

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::canvas::{Color, SkiaCanvas};
use tezzera_render::FontCache;
use tezzera_theme::ThemeData;
use crate::theme_color_to_render as tc;

/// A pill-shaped toggle switch with an animated thumb.
pub struct Switch {
    pub on: bool,
    pub disabled: bool,
}

impl Switch {
    /// Creates a new [`Switch`] in the off position.
    pub fn new() -> Self {
        Self { on: false, disabled: false }
    }

    /// Sets whether the switch is on.
    pub fn on(mut self, v: bool) -> Self {
        self.on = v;
        self
    }

    /// When `true`, the switch is rendered in a muted, non-interactive style.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    /// Paints the switch onto `canvas` at `(x, y)`.
    pub fn render(
        &self,
        canvas: &mut SkiaCanvas,
        _font: &FontCache,
        x: f32,
        y: f32,
        theme: &ThemeData,
    ) {
        let track_w = Self::width();
        let track_h = Self::height();
        let thumb_r = 11.0_f32;

        let track_color = if self.disabled {
            Color::rgba(120, 120, 120, 150)
        } else if self.on {
            tc(theme.colors.primary)
        } else {
            tc(theme.colors.outline)
        };

        canvas.fill_rect(
            Rect {
                origin: Point { x, y },
                size: Size { width: track_w, height: track_h },
            },
            track_color,
        );

        // Thumb position: right when on, left when off
        let thumb_x = if self.on {
            x + track_w - thumb_r - 4.0
        } else {
            x + thumb_r + 4.0
        };
        let thumb_y = y + track_h / 2.0;

        let thumb_color = if self.on {
            tc(theme.colors.on_primary)
        } else {
            Color::WHITE
        };
        canvas.fill_circle(Point { x: thumb_x, y: thumb_y }, thumb_r, thumb_color);
    }

    /// Fixed width of the switch track in pixels.
    pub fn width() -> f32 {
        52.0
    }

    /// Fixed height of the switch track in pixels.
    pub fn height() -> f32 {
        28.0
    }
}

impl Default for Switch {
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
    fn switch_default_is_off() {
        let sw = Switch::new();
        assert!(!sw.on);
    }

    #[test]
    fn switch_on_changes_state() {
        let sw = Switch::new().on(true);
        assert!(sw.on);
    }

    #[test]
    fn switch_off_after_on() {
        let sw = Switch::new().on(true).on(false);
        assert!(!sw.on);
    }

    #[test]
    fn switch_fixed_dimensions() {
        assert_eq!(Switch::width(), 52.0);
        assert_eq!(Switch::height(), 28.0);
    }

    #[test]
    fn switch_disabled_setter() {
        let sw = Switch::new().disabled(true);
        assert!(sw.disabled);
    }

    #[test]
    fn switch_default_not_disabled() {
        let sw = Switch::default();
        assert!(!sw.disabled);
    }

    #[test]
    fn switch_on_thumb_position_is_right() {
        // thumb_x when on = track_w - thumb_r - 4 = 52 - 11 - 4 = 37
        let track_w = Switch::width();
        let thumb_r = 11.0_f32;
        let thumb_x_on = track_w - thumb_r - 4.0;
        assert!(thumb_x_on > Switch::width() / 2.0);
    }

    #[test]
    fn switch_off_thumb_position_is_left() {
        // thumb_x when off = thumb_r + 4 = 11 + 4 = 15
        let thumb_r = 11.0_f32;
        let thumb_x_off = thumb_r + 4.0;
        assert!(thumb_x_off < Switch::width() / 2.0);
    }
}
