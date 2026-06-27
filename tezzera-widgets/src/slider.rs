//! [`Slider`] — a horizontal range slider with optional value label.

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::canvas::SkiaCanvas;
use tezzera_render::FontCache;
use tezzera_theme::ThemeData;
use crate::theme_color_to_render as tc;

/// A horizontal slider for selecting a value within a numeric range.
pub struct Slider {
    /// The current value (clamped to `[min, max]` during render).
    pub value: f32,
    /// Minimum value.
    pub min: f32,
    /// Maximum value.
    pub max: f32,
    /// Width of the slider track in pixels.
    pub width: f32,
    /// Whether to show the numeric value below the thumb.
    pub show_label: bool,
}

impl Slider {
    /// Creates a new [`Slider`] with range `0.0–1.0` and value `0.0`.
    pub fn new() -> Self {
        Self {
            value: 0.0,
            min: 0.0,
            max: 1.0,
            width: 200.0,
            show_label: false,
        }
    }

    /// Sets the current value.
    pub fn value(mut self, v: f32) -> Self {
        self.value = v;
        self
    }

    /// Sets the minimum bound.
    pub fn min(mut self, v: f32) -> Self {
        self.min = v;
        self
    }

    /// Sets the maximum bound.
    pub fn max(mut self, v: f32) -> Self {
        self.max = v;
        self
    }

    /// Sets the slider track width in pixels.
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// When `true`, renders a numeric label below the thumb.
    pub fn show_label(mut self, s: bool) -> Self {
        self.show_label = s;
        self
    }

    /// Returns the value normalized to `[0.0, 1.0]`.
    ///
    /// Returns `0.0` when `min == max` to avoid division by zero.
    pub fn normalized(&self) -> f32 {
        if (self.max - self.min).abs() < f32::EPSILON {
            0.0
        } else {
            (self.value - self.min) / (self.max - self.min)
        }
    }

    /// Paints the slider onto `canvas` at `(x, y)`.
    pub fn render(
        &self,
        canvas: &mut SkiaCanvas,
        font: &FontCache,
        x: f32,
        y: f32,
        theme: &ThemeData,
    ) {
        let track_h = 4.0_f32;
        let thumb_r = 10.0_f32;
        let t = self.normalized().clamp(0.0, 1.0);
        let track_y = y + thumb_r - track_h / 2.0;

        // Track background
        canvas.fill_rect(
            Rect {
                origin: Point { x, y: track_y },
                size: Size { width: self.width, height: track_h },
            },
            tc(theme.colors.outline),
        );

        // Track fill (progress)
        canvas.fill_rect(
            Rect {
                origin: Point { x, y: track_y },
                size: Size { width: self.width * t, height: track_h },
            },
            tc(theme.colors.primary),
        );

        // Thumb
        let thumb_x = x + self.width * t;
        canvas.fill_circle(
            Point { x: thumb_x, y: y + thumb_r },
            thumb_r,
            tc(theme.colors.primary),
        );

        // Optional numeric label
        if self.show_label {
            let label = format!("{:.0}", self.min + (self.max - self.min) * t);
            canvas.draw_text(
                &label,
                Point { x: thumb_x - 8.0, y: y + thumb_r * 2.0 + 2.0 },
                tc(theme.colors.on_surface),
                font,
                11.0,
            );
        }
    }

    /// Fixed height of the slider widget in pixels (thumb diameter).
    pub fn height() -> f32 {
        24.0
    }
}

impl Default for Slider {
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
    fn slider_normalized_value_midpoint() {
        let s = Slider::new().min(0.0).max(100.0).value(50.0);
        assert!((s.normalized() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn slider_normalized_at_min_is_zero() {
        let s = Slider::new().min(10.0).max(20.0).value(10.0);
        assert!((s.normalized() - 0.0).abs() < 1e-6);
    }

    #[test]
    fn slider_normalized_at_max_is_one() {
        let s = Slider::new().min(10.0).max(20.0).value(20.0);
        assert!((s.normalized() - 1.0).abs() < 1e-6);
    }

    #[test]
    fn slider_normalized_zero_range_returns_zero() {
        let s = Slider::new().min(5.0).max(5.0).value(5.0);
        assert_eq!(s.normalized(), 0.0);
    }

    #[test]
    fn slider_default_range_zero_to_one() {
        let s = Slider::new();
        assert_eq!(s.min, 0.0);
        assert_eq!(s.max, 1.0);
    }

    #[test]
    fn slider_width_setter() {
        let s = Slider::new().width(300.0);
        assert_eq!(s.width, 300.0);
    }

    #[test]
    fn slider_height_constant() {
        assert_eq!(Slider::height(), 24.0);
    }

    #[test]
    fn slider_show_label_setter() {
        let s = Slider::new().show_label(true);
        assert!(s.show_label);
    }
}
