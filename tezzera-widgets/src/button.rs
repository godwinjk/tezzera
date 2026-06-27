//! [`Button`] — a pressable widget with a label.

use tezzera_core::{element::Element, types::{Point, Rect, Size}};
use tezzera_render::canvas::{Color, SkiaCanvas};

/// A rectangular button with a text label.
///
/// Pressing the button calls the closure registered with [`Button::on_press`].
/// When [`Button::disabled`] is `true`, [`Button::fire_press`] is a no-op.
pub struct Button {
    label: String,
    on_press: Option<Box<dyn Fn() + Send + Sync>>,
    background: Color,
    foreground: Color,
    padding: f32,
    disabled: bool,
}

impl Button {
    /// Creates a new [`Button`] with the given label and default styling.
    ///
    /// Default background is a medium blue (`rgb(70, 130, 200)`), foreground
    /// is white, and padding is 8 px.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            on_press: None,
            background: Color::rgb(70, 130, 200),
            foreground: Color::WHITE,
            padding: 8.0,
            disabled: false,
        }
    }

    /// Registers a callback that is invoked when the button is pressed.
    pub fn on_press(mut self, f: impl Fn() + Send + Sync + 'static) -> Self {
        self.on_press = Some(Box::new(f));
        self
    }

    /// Sets the button's background color.
    pub fn background(mut self, c: Color) -> Self {
        self.background = c;
        self
    }

    /// Sets the text / foreground color.
    pub fn foreground(mut self, c: Color) -> Self {
        self.foreground = c;
        self
    }

    /// When `true`, [`fire_press`](Self::fire_press) becomes a no-op and
    /// the button renders with a grey background.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

    /// Returns the button's approximate natural size based on the label length
    /// and padding.
    pub fn natural_size(&self) -> Size {
        let text_w = self.label.len() as f32 * 8.0;
        Size { width: text_w + self.padding * 2.0, height: 16.0 + self.padding * 2.0 }
    }

    /// Paints the button onto `canvas` at `origin` with the given `size`.
    ///
    /// Renders a grey background when the button is disabled.
    pub fn paint(&self, canvas: &mut SkiaCanvas, origin: Point, size: Size) {
        let bg = if self.disabled { Color::rgb(180, 180, 180) } else { self.background };
        canvas.fill_rect(Rect { origin, size }, bg);
        let text_origin = Point {
            x: origin.x + self.padding,
            y: origin.y + self.padding,
        };
        canvas.draw_text_placeholder(&self.label, text_origin, self.foreground);
    }

    /// Fires the registered `on_press` callback if the button is not disabled.
    pub fn fire_press(&self) {
        if !self.disabled {
            if let Some(f) = &self.on_press {
                f();
            }
        }
    }
}

impl From<Button> for Element {
    fn from(b: Button) -> Element {
        Element::Native(tezzera_core::element::NativeElement {
            tag: "button",
            children: vec![Element::text(b.label)],
        })
    }
}
