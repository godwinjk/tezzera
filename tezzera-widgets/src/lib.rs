//! `tezzera-widgets` — built-in widgets for the TEZZERA UI framework.
//!
//! This crate is the "glue" layer: it implements concrete widgets by combining
//! `tezzera-core` (traits and element tree), `tezzera-layout` (constraint-based
//! layout), `tezzera-render` (pixel painting), and `tezzera-state` (atoms).
//!
//! # Phase 1 contents
//!
//! - [`Text`] — plain text leaf widget
//! - [`Button`] — pressable button with an optional callback
//! - [`counter_app`] — integration demo that renders frames to a pixel buffer

pub mod button;
pub mod counter_app;
pub mod text;

pub use button::Button;
pub use counter_app::{render_counter_frame, run_counter_simulation};
pub use text::Text;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counter_simulation_increments_correctly() {
        let result = run_counter_simulation(5);
        assert_eq!(result, 5);
    }

    #[test]
    fn counter_simulation_starts_at_zero() {
        let result = run_counter_simulation(0);
        assert_eq!(result, 0);
    }

    #[test]
    fn render_counter_frame_returns_correct_pixel_count() {
        let pixels = render_counter_frame(0, 400, 300);
        // RGBA = 4 bytes per pixel
        assert_eq!(pixels.len(), 400 * 300 * 4);
    }

    #[test]
    fn render_counter_frame_is_not_all_zeros() {
        let pixels = render_counter_frame(3, 200, 200);
        assert!(pixels.iter().any(|&b| b != 0));
    }

    #[test]
    fn render_counter_frame_different_counts_produce_different_pixels() {
        let frame_0 = render_counter_frame(0, 200, 100);
        let frame_99 = render_counter_frame(99, 200, 100);
        // "Count: 0" vs "Count: 99" — pixels must differ
        assert_ne!(frame_0, frame_99);
    }

    #[test]
    fn text_natural_size_scales_with_content() {
        let short = Text::new("Hi").natural_size();
        let long = Text::new("Hello World").natural_size();
        assert!(long.width > short.width);
    }

    #[test]
    fn button_fires_on_press() {
        use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
        let fired = Arc::new(AtomicBool::new(false));
        let fired2 = fired.clone();
        let btn = Button::new("Click").on_press(move || fired2.store(true, Ordering::SeqCst));
        btn.fire_press();
        assert!(fired.load(Ordering::SeqCst));
    }

    #[test]
    fn disabled_button_does_not_fire() {
        use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
        let fired = Arc::new(AtomicBool::new(false));
        let fired2 = fired.clone();
        let btn = Button::new("Click")
            .on_press(move || fired2.store(true, Ordering::SeqCst))
            .disabled(true);
        btn.fire_press();
        assert!(!fired.load(Ordering::SeqCst));
    }
}
