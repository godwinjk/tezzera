//! Overlay widgets: [`Modal`], [`Dialog`], [`Toast`], and [`ToastQueue`].
//!
//! Overlay widgets paint on top of all other content. They are rendered last
//! in the frame and do not participate in normal layout.

use tezzera_core::types::{Point, Rect, Size};
use tezzera_render::{Color, FontCache, SkiaCanvas};
use tezzera_theme::ThemeData;
use crate::theme_color_to_render as tc;

// ---------------------------------------------------------------------------
// Modal
// ---------------------------------------------------------------------------

/// A full-canvas dim overlay with a centered content box.
///
/// Call [`render_backdrop`] followed by [`render_box`] to draw the modal, then
/// draw your own content within the returned `(x, y)` coordinates.
///
/// [`render_backdrop`]: Modal::render_backdrop
/// [`render_box`]: Modal::render_box
pub struct Modal {
    /// Whether the modal is currently visible.
    pub visible: bool,
    /// Width of the modal content box in pixels.
    pub width: f32,
    /// Height of the modal content box in pixels.
    pub height: f32,
    /// Whether a click on the dim backdrop should dismiss the modal.
    pub dismiss_on_backdrop: bool,
    /// Opacity of the dim backdrop (0 = transparent, 255 = opaque).
    pub dim_alpha: u8,
}

impl Modal {
    /// Create a new hidden modal with sensible defaults.
    pub fn new() -> Self {
        Self {
            visible: false,
            width: 400.0,
            height: 250.0,
            dismiss_on_backdrop: true,
            dim_alpha: 160,
        }
    }

    /// Set modal visibility.
    pub fn visible(mut self, v: bool) -> Self {
        self.visible = v;
        self
    }

    /// Set the modal content box dimensions.
    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.width = w;
        self.height = h;
        self
    }

    /// Set the backdrop dim alpha.
    pub fn dim_alpha(mut self, a: u8) -> Self {
        self.dim_alpha = a;
        self
    }

    /// Draw the semi-transparent dim overlay over the entire canvas.
    ///
    /// Must be called before drawing the content box so the box appears on top.
    pub fn render_backdrop(&self, canvas: &mut SkiaCanvas) {
        if !self.visible {
            return;
        }
        let w = canvas.width() as f32;
        let h = canvas.height() as f32;
        canvas.fill_rect(
            Rect {
                origin: Point { x: 0.0, y: 0.0 },
                size: Size { width: w, height: h },
            },
            Color::rgba(0, 0, 0, self.dim_alpha),
        );
    }

    /// Draw the centered modal content box and return `(content_x, content_y)`.
    ///
    /// Returns `(0.0, 0.0)` when the modal is not visible.
    pub fn render_box(&self, canvas: &mut SkiaCanvas, theme: &ThemeData) -> (f32, f32) {
        if !self.visible {
            return (0.0, 0.0);
        }
        let cx = (canvas.width() as f32 - self.width) / 2.0;
        let cy = (canvas.height() as f32 - self.height) / 2.0;
        canvas.fill_rect(
            Rect {
                origin: Point { x: cx, y: cy },
                size: Size {
                    width: self.width,
                    height: self.height,
                },
            },
            tc(theme.colors.surface),
        );
        canvas.stroke_rect(
            Rect {
                origin: Point { x: cx, y: cy },
                size: Size {
                    width: self.width,
                    height: self.height,
                },
            },
            tc(theme.colors.outline),
            1.5,
        );
        (cx, cy)
    }

    /// Returns `true` if `(mx, my)` is outside the modal content box
    /// and the modal is configured to dismiss on backdrop clicks.
    pub fn backdrop_clicked(&self, canvas: &SkiaCanvas, mx: f32, my: f32) -> bool {
        if !self.visible || !self.dismiss_on_backdrop {
            return false;
        }
        let cx = (canvas.width() as f32 - self.width) / 2.0;
        let cy = (canvas.height() as f32 - self.height) / 2.0;
        !(mx >= cx && mx <= cx + self.width && my >= cy && my <= cy + self.height)
    }
}

impl Default for Modal {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Dialog
// ---------------------------------------------------------------------------

/// A modal dialog with a title, body message, and one or more action buttons.
pub struct Dialog {
    pub modal: Modal,
    pub title: String,
    pub message: String,
    pub buttons: Vec<String>,
}

impl Dialog {
    /// Create a new visible dialog with a title and message.
    pub fn new(title: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            modal: Modal::new().visible(true).size(440.0, 200.0),
            title: title.into(),
            message: message.into(),
            buttons: vec!["OK".to_string()],
        }
    }

    /// Append an action button with the given label.
    pub fn button(mut self, label: impl Into<String>) -> Self {
        self.buttons.push(label.into());
        self
    }

    /// Set dialog visibility.
    pub fn visible(mut self, v: bool) -> Self {
        self.modal.visible = v;
        self
    }

    /// Render the full dialog (backdrop + box + title + message + buttons).
    pub fn render(&self, canvas: &mut SkiaCanvas, font: &FontCache, theme: &ThemeData) {
        self.modal.render_backdrop(canvas);
        let (bx, by) = self.modal.render_box(canvas, theme);
        canvas.draw_text(
            &self.title,
            Point {
                x: bx + 20.0,
                y: by + 20.0,
            },
            tc(theme.colors.on_surface),
            font,
            16.0,
        );
        canvas.draw_text(
            &self.message,
            Point {
                x: bx + 20.0,
                y: by + 52.0,
            },
            tc(theme.colors.on_surface),
            font,
            13.0,
        );
        // Render buttons right-aligned.
        let mut btn_x = bx + self.modal.width - 20.0;
        for label in self.buttons.iter().rev() {
            let bw = label.len() as f32 * 8.0 + 24.0;
            btn_x -= bw + 10.0;
            let btn_y = by + self.modal.height - 50.0;
            canvas.fill_rect(
                Rect {
                    origin: Point { x: btn_x, y: btn_y },
                    size: Size { width: bw, height: 32.0 },
                },
                tc(theme.colors.primary),
            );
            canvas.draw_text(
                label,
                Point {
                    x: btn_x + 12.0,
                    y: btn_y + 9.0,
                },
                tc(theme.colors.on_primary),
                font,
                12.0,
            );
        }
    }

    /// Returns `true` when the dialog is currently visible.
    pub fn is_visible(&self) -> bool {
        self.modal.visible
    }
}

// ---------------------------------------------------------------------------
// Toast
// ---------------------------------------------------------------------------

/// A single transient notification that expires after a fixed duration.
#[derive(Debug, Clone)]
pub struct Toast {
    /// The message to display.
    pub message: String,
    /// Total lifetime in seconds.
    pub lifetime: f32,
    /// Seconds elapsed since the toast was created.
    pub elapsed: f32,
}

impl Toast {
    /// Create a new 3-second toast with the given message.
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            lifetime: 3.0,
            elapsed: 0.0,
        }
    }

    /// Override the default lifetime.
    pub fn lifetime(mut self, secs: f32) -> Self {
        self.lifetime = secs;
        self
    }

    /// Returns `true` once `elapsed >= lifetime`.
    pub fn is_expired(&self) -> bool {
        self.elapsed >= self.lifetime
    }

    /// Advance elapsed time by `dt` seconds.
    pub fn tick(&mut self, dt: f32) {
        self.elapsed += dt;
    }

    /// Remaining display time in seconds (clamped to 0).
    pub fn remaining(&self) -> f32 {
        (self.lifetime - self.elapsed).max(0.0)
    }

    /// Fraction of lifetime consumed, clamped to [0, 1].
    pub fn progress(&self) -> f32 {
        (self.elapsed / self.lifetime).min(1.0)
    }
}

// ---------------------------------------------------------------------------
// ToastQueue
// ---------------------------------------------------------------------------

/// Manages a queue of active [`Toast`] notifications.
///
/// Toasts are rendered stacked from the bottom of the canvas. The oldest toast
/// is evicted when the queue reaches capacity.
pub struct ToastQueue {
    toasts: Vec<Toast>,
    max: usize,
}

impl ToastQueue {
    /// Create a new queue with capacity for 5 toasts.
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            max: 5,
        }
    }

    /// Add a toast. Evicts the oldest entry if the queue is at capacity.
    pub fn push(&mut self, toast: Toast) {
        if self.toasts.len() >= self.max {
            self.toasts.remove(0);
        }
        self.toasts.push(toast);
    }

    /// Advance all toasts by `dt` seconds and remove expired ones.
    ///
    /// Returns the number of toasts that expired and were removed.
    pub fn tick(&mut self, dt: f32) -> usize {
        for t in &mut self.toasts {
            t.tick(dt);
        }
        let before = self.toasts.len();
        self.toasts.retain(|t| !t.is_expired());
        before - self.toasts.len()
    }

    /// Render all active toasts centered at the bottom of the canvas.
    pub fn render(&self, canvas: &mut SkiaCanvas, font: &FontCache, theme: &ThemeData) {
        let canvas_w = canvas.width() as f32;
        let canvas_h = canvas.height() as f32;
        let toast_h = 44.0_f32;
        let toast_w = 320.0_f32;

        for (i, toast) in self.toasts.iter().enumerate() {
            let tx = (canvas_w - toast_w) / 2.0;
            let ty = canvas_h - 60.0 - (i as f32 * (toast_h + 8.0));

            // Fade out during the last 20% of lifetime.
            let alpha = if toast.progress() > 0.8 {
                ((1.0 - toast.progress()) * 5.0 * 255.0) as u8
            } else {
                220u8
            };

            canvas.fill_rect(
                Rect {
                    origin: Point { x: tx, y: ty },
                    size: Size {
                        width: toast_w,
                        height: toast_h,
                    },
                },
                Color::rgba(40, 42, 60, alpha),
            );
            canvas.stroke_rect(
                Rect {
                    origin: Point { x: tx, y: ty },
                    size: Size {
                        width: toast_w,
                        height: toast_h,
                    },
                },
                Color::rgba(103, 80, 164, alpha),
                1.0,
            );
            canvas.draw_text(
                &toast.message,
                Point {
                    x: tx + 16.0,
                    y: ty + 14.0,
                },
                tc(theme.colors.on_surface),
                font,
                13.0,
            );
        }
    }

    /// Number of active toasts in the queue.
    pub fn len(&self) -> usize {
        self.toasts.len()
    }

    /// Returns `true` when there are no active toasts.
    pub fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }

    /// Remove all toasts from the queue.
    pub fn clear(&mut self) {
        self.toasts.clear();
    }

    /// Read-only slice of all active toasts.
    pub fn active(&self) -> &[Toast] {
        &self.toasts
    }
}

impl Default for ToastQueue {
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

    // ── Modal ─────────────────────────────────────────────────────────────────

    #[test]
    fn modal_new_defaults() {
        let m = Modal::new();
        assert!(!m.visible);
        assert_eq!(m.width, 400.0);
        assert_eq!(m.height, 250.0);
        assert!(m.dismiss_on_backdrop);
        assert_eq!(m.dim_alpha, 160);
    }

    #[test]
    fn modal_visible_setter() {
        let m = Modal::new().visible(true);
        assert!(m.visible);
    }

    #[test]
    fn modal_size_setter() {
        let m = Modal::new().size(600.0, 400.0);
        assert_eq!(m.width, 600.0);
        assert_eq!(m.height, 400.0);
    }

    #[test]
    fn modal_not_visible_backdrop_click_false() {
        let canvas = SkiaCanvas::new(800, 600);
        let m = Modal::new(); // visible = false
        assert!(!m.backdrop_clicked(&canvas, 0.0, 0.0));
    }

    // ── Dialog ────────────────────────────────────────────────────────────────

    #[test]
    fn dialog_new_has_title_and_message() {
        let d = Dialog::new("Hello", "World");
        assert_eq!(d.title, "Hello");
        assert_eq!(d.message, "World");
    }

    #[test]
    fn dialog_button_added() {
        let d = Dialog::new("T", "M").button("Cancel").button("OK");
        // Default "OK" + "Cancel" + "OK" = 3
        assert_eq!(d.buttons.len(), 3);
        assert_eq!(d.buttons[1], "Cancel");
    }

    #[test]
    fn dialog_visible_false_hides() {
        let d = Dialog::new("T", "M").visible(false);
        assert!(!d.is_visible());
    }

    // ── Toast ─────────────────────────────────────────────────────────────────

    #[test]
    fn toast_new_defaults() {
        let t = Toast::new("hello");
        assert_eq!(t.message, "hello");
        assert_eq!(t.lifetime, 3.0);
        assert_eq!(t.elapsed, 0.0);
    }

    #[test]
    fn toast_tick_increments_elapsed() {
        let mut t = Toast::new("hi");
        t.tick(1.0);
        assert!((t.elapsed - 1.0).abs() < 1e-6);
        t.tick(0.5);
        assert!((t.elapsed - 1.5).abs() < 1e-6);
    }

    #[test]
    fn toast_is_expired_after_lifetime() {
        let mut t = Toast::new("bye").lifetime(2.0);
        assert!(!t.is_expired());
        t.tick(2.0);
        assert!(t.is_expired());
    }

    #[test]
    fn toast_remaining_decreases() {
        let mut t = Toast::new("r").lifetime(5.0);
        t.tick(2.0);
        assert!((t.remaining() - 3.0).abs() < 1e-6);
    }

    #[test]
    fn toast_progress_clamps_to_one() {
        let mut t = Toast::new("p").lifetime(1.0);
        t.tick(10.0); // way past lifetime
        assert_eq!(t.progress(), 1.0);
    }

    #[test]
    fn toast_lifetime_setter() {
        let t = Toast::new("x").lifetime(7.5);
        assert_eq!(t.lifetime, 7.5);
    }

    // ── ToastQueue ────────────────────────────────────────────────────────────

    #[test]
    fn toast_queue_new_empty() {
        let q = ToastQueue::new();
        assert!(q.is_empty());
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn toast_queue_push() {
        let mut q = ToastQueue::new();
        q.push(Toast::new("a"));
        q.push(Toast::new("b"));
        assert_eq!(q.len(), 2);
    }

    #[test]
    fn toast_queue_tick_removes_expired() {
        let mut q = ToastQueue::new();
        q.push(Toast::new("short").lifetime(0.5));
        q.push(Toast::new("long").lifetime(5.0));
        let removed = q.tick(1.0);
        assert_eq!(removed, 1);
        assert_eq!(q.len(), 1);
        assert_eq!(q.active()[0].message, "long");
    }

    #[test]
    fn toast_queue_max_capacity_evicts_oldest() {
        let mut q = ToastQueue::new();
        for i in 0..6 {
            q.push(Toast::new(format!("msg{i}")));
        }
        // max=5; 6th push evicts the first
        assert_eq!(q.len(), 5);
        assert_eq!(q.active()[0].message, "msg1");
    }

    #[test]
    fn toast_queue_len() {
        let mut q = ToastQueue::new();
        q.push(Toast::new("a"));
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn toast_queue_clear() {
        let mut q = ToastQueue::new();
        q.push(Toast::new("a"));
        q.push(Toast::new("b"));
        q.clear();
        assert!(q.is_empty());
    }

    #[test]
    fn toast_queue_active_returns_slice() {
        let mut q = ToastQueue::new();
        q.push(Toast::new("x"));
        let slice = q.active();
        assert_eq!(slice.len(), 1);
        assert_eq!(slice[0].message, "x");
    }
}
