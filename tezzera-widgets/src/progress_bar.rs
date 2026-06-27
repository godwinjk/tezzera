//! [`ProgressBar`] — a determinate or indeterminate progress indicator.

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::canvas::SkiaCanvas;
use tezzera_render::FontCache;
use tezzera_theme::ThemeData;
use crate::theme_color_to_render as tc;

/// A horizontal progress bar that can be determinate (`value` in `[0, 1]`) or
/// indeterminate (animating pulse).
pub struct ProgressBar {
    /// `Some(v)` → determinate (0.0–1.0). `None` → indeterminate.
    pub value: Option<f32>,
    /// Width of the bar in pixels.
    pub width: f32,
    /// Height of the bar in pixels.
    pub height: f32,
    /// Phase of the indeterminate animation pulse (`0.0–1.0`).
    pub pulse_phase: f32,
}

impl ProgressBar {
    /// Creates a new indeterminate [`ProgressBar`] with default dimensions.
    pub fn new() -> Self {
        Self {
            value: None,
            width: 200.0,
            height: 6.0,
            pulse_phase: 0.0,
        }
    }

    /// Sets a determinate progress value clamped to `[0.0, 1.0]`.
    pub fn value(mut self, v: f32) -> Self {
        self.value = Some(v.clamp(0.0, 1.0));
        self
    }

    /// Switches the bar to indeterminate mode.
    pub fn indeterminate(mut self) -> Self {
        self.value = None;
        self
    }

    /// Sets the bar width in pixels.
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// Sets the bar height in pixels.
    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    /// Sets the animation pulse phase (`0.0–1.0`) for indeterminate mode.
    pub fn pulse_phase(mut self, p: f32) -> Self {
        self.pulse_phase = p;
        self
    }

    /// Paints the progress bar onto `canvas` at `(x, y)`.
    pub fn render(
        &self,
        canvas: &mut SkiaCanvas,
        _font: &FontCache,
        x: f32,
        y: f32,
        theme: &ThemeData,
    ) {
        // Track background
        canvas.fill_rect(
            Rect {
                origin: Point { x, y },
                size: Size { width: self.width, height: self.height },
            },
            tc(theme.colors.outline),
        );

        match self.value {
            Some(v) => {
                canvas.fill_rect(
                    Rect {
                        origin: Point { x, y },
                        size: Size { width: self.width * v, height: self.height },
                    },
                    tc(theme.colors.primary),
                );
            }
            None => {
                // Indeterminate pulse: a block that sweeps across the track
                let block_w = self.width * 0.4;
                let offset = self.pulse_phase * (self.width + block_w) - block_w;
                let bx = (x + offset).max(x);
                let bw = ((x + offset + block_w).min(x + self.width) - bx).max(0.0);
                canvas.fill_rect(
                    Rect {
                        origin: Point { x: bx, y },
                        size: Size { width: bw, height: self.height },
                    },
                    tc(theme.colors.primary),
                );
            }
        }
    }
}

impl Default for ProgressBar {
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
    fn progress_bar_new_is_indeterminate() {
        let pb = ProgressBar::new();
        assert!(pb.value.is_none());
    }

    #[test]
    fn progress_bar_determinate_value() {
        let pb = ProgressBar::new().value(0.75);
        assert_eq!(pb.value, Some(0.75));
    }

    #[test]
    fn progress_bar_value_clamped_above_one() {
        let pb = ProgressBar::new().value(1.5);
        assert_eq!(pb.value, Some(1.0));
    }

    #[test]
    fn progress_bar_value_clamped_below_zero() {
        let pb = ProgressBar::new().value(-0.5);
        assert_eq!(pb.value, Some(0.0));
    }

    #[test]
    fn progress_bar_indeterminate_pulse_phase() {
        let pb = ProgressBar::new().value(0.5).indeterminate().pulse_phase(0.3);
        assert!(pb.value.is_none());
        assert!((pb.pulse_phase - 0.3).abs() < 1e-6);
    }

    #[test]
    fn progress_bar_width_setter() {
        let pb = ProgressBar::new().width(400.0);
        assert_eq!(pb.width, 400.0);
    }

    #[test]
    fn progress_bar_height_setter() {
        let pb = ProgressBar::new().height(10.0);
        assert_eq!(pb.height, 10.0);
    }

    #[test]
    fn progress_bar_default_dimensions() {
        let pb = ProgressBar::default();
        assert_eq!(pb.width, 200.0);
        assert_eq!(pb.height, 6.0);
    }
}
