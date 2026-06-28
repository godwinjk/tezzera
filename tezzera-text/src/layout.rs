use crate::direction::TextDirection;
use crate::span::TextSpan;
#[cfg(test)]
use crate::span::TextStyle;
use tezzera_core::types::Point;
use tezzera_render::{Color, FontCache, SkiaCanvas};
use tezzera_theme::Color as ThemeColor;

/// A single laid-out line of text (may contain multiple spans).
#[derive(Debug, Clone)]
pub struct TextLine {
    pub spans: Vec<TextSpan>,
    pub width: f32,
    pub height: f32,
}

impl TextLine {
    pub fn new() -> Self { Self { spans: Vec::new(), width: 0.0, height: 0.0 } }

    pub fn push_span(&mut self, span: TextSpan) {
        self.height = self.height.max(span.style.font_size * 1.3);
        self.width += span.estimated_width();
        self.spans.push(span);
    }

    /// Like `push_span` but uses the caller-supplied `width` instead of the
    /// heuristic, so `layout_with_measure` can track accurate widths.
    pub fn push_span_with_width(&mut self, span: TextSpan, width: f32) {
        self.height = self.height.max(span.style.font_size * 1.3);
        self.width += width;
        self.spans.push(span);
    }

    pub fn is_empty(&self) -> bool { self.spans.is_empty() }

    pub fn plain_text(&self) -> String {
        self.spans.iter().map(|s| s.text.as_str()).collect()
    }
}

impl Default for TextLine { fn default() -> Self { Self::new() } }

/// Lays out a `RichText` into wrapped lines given a maximum pixel width.
pub struct TextLayout {
    pub lines: Vec<TextLine>,
    pub max_width: f32,
    pub line_spacing: f32,
    pub direction: TextDirection,
}

impl TextLayout {
    /// Lay out spans into wrapped lines.
    pub fn layout(spans: &[TextSpan], max_width: f32) -> Self {
        let mut lines: Vec<TextLine> = Vec::new();
        let mut current = TextLine::new();

        for span in spans {
            // Split span text into words
            let words: Vec<&str> = span.text.split_whitespace().collect();
            let leading_space = span.text.starts_with(' ');
            let trailing_space = span.text.ends_with(' ');

            for (i, word) in words.iter().enumerate() {
                let prefix = if i == 0 && leading_space { " " } else if i > 0 { " " } else { "" };
                let token = format!("{}{}", prefix, word);
                let token_span = TextSpan::new(&token, span.style.clone());
                let token_w = token_span.estimated_width();

                if !current.is_empty() && current.width + token_w > max_width {
                    // Wrap: commit current line, start new
                    lines.push(std::mem::replace(&mut current, TextLine::new()));
                }
                current.push_span(token_span);
            }

            if trailing_space && !words.is_empty() {
                // Add trailing space as part of the span
                let sp = TextSpan::new(" ", span.style.clone());
                current.push_span(sp);
            }
        }

        if !current.is_empty() {
            lines.push(current);
        }

        Self { lines, max_width, line_spacing: 1.4, direction: TextDirection::Ltr }
    }

    /// Lay out spans for RTL display.
    ///
    /// Calls `layout` then reverses each line's span order for visual RTL reordering.
    pub fn layout_rtl(spans: &[TextSpan], max_width: f32) -> Self {
        let mut layout = Self::layout(spans, max_width);
        for line in &mut layout.lines {
            line.spans.reverse();
        }
        layout.direction = TextDirection::Rtl;
        layout
    }

    /// Total height of all lines.
    pub fn total_height(&self) -> f32 {
        self.lines.iter().map(|l| l.height * self.line_spacing).sum()
    }

    /// Number of lines.
    pub fn line_count(&self) -> usize { self.lines.len() }

    /// Render this layout onto a canvas at (x, y).
    pub fn render(&self, canvas: &mut SkiaCanvas, font: &FontCache, x: f32, y: f32) {
        let mut cy = y;
        for line in &self.lines {
            let mut cx = x;
            for span in &line.spans {
                let color = theme_color_to_render(span.style.color);
                canvas.draw_text(&span.text, Point { x: cx, y: cy }, color, font, span.style.font_size);
                cx += span.estimated_width();
            }
            cy += line.height * self.line_spacing;
        }
    }
}

fn theme_color_to_render(c: ThemeColor) -> Color {
    Color::rgba(
        (c.r * 255.0) as u8,
        (c.g * 255.0) as u8,
        (c.b * 255.0) as u8,
        (c.a * 255.0) as u8,
    )
}

/// Greedy word wrapper — returns wrapped lines for plain text.
///
/// `char_width` is the pixel width of a single character.
pub fn word_wrap(text: &str, max_width: f32, char_width: f32) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    let mut current_w = 0.0_f32;

    for word in text.split_whitespace() {
        let word_w = word.len() as f32 * char_width;
        let space_w = char_width;

        if !current.is_empty() && current_w + space_w + word_w > max_width {
            lines.push(current.trim_end().to_string());
            current = String::new();
            current_w = 0.0;
        }

        if !current.is_empty() {
            current.push(' ');
            current_w += space_w;
        }
        current.push_str(word);
        current_w += word_w;
    }

    if !current.is_empty() {
        lines.push(current.trim_end().to_string());
    }

    if lines.is_empty() && !text.is_empty() {
        lines.push(text.to_string());
    }

    lines
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use tezzera_theme::Color;

    fn style(size: f32) -> TextStyle {
        TextStyle::new(size, Color::WHITE)
    }

    // ---- word_wrap tests ----

    #[test]
    fn word_wrap_empty_string() {
        let lines = word_wrap("", 100.0, 8.0);
        assert!(lines.is_empty());
    }

    #[test]
    fn word_wrap_single_word_fits() {
        // "Hi" = 2 chars * 8.0 = 16.0 < 100.0
        let lines = word_wrap("Hi", 100.0, 8.0);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Hi");
    }

    #[test]
    fn word_wrap_single_word_too_long_kept() {
        // A very long word that exceeds max_width is kept on its own line.
        let lines = word_wrap("superlongword", 10.0, 8.0);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "superlongword");
    }

    #[test]
    fn word_wrap_two_words_fit_on_one_line() {
        // "Hi" + " " + "yo" = 2+1+2 = 5 chars * 8.0 = 40.0 < 100.0
        let lines = word_wrap("Hi yo", 100.0, 8.0);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Hi yo");
    }

    #[test]
    fn word_wrap_two_words_wrap() {
        // char_width = 8.0, max = 20.0
        // "Hello" = 5*8 = 40 > 20 → stays on first line (forced)
        // "world" pushed to next because 40 + 8 + 40 > 20
        let lines = word_wrap("Hello world", 20.0, 8.0);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[0], "Hello");
        assert_eq!(lines[1], "world");
    }

    #[test]
    fn word_wrap_multiple_lines() {
        // char_width=10, max=50: each word "one"=30, "two"=30, "three"=50 fits alone
        // "one two" = 30+10+30=70 > 50 → wrap
        let lines = word_wrap("one two three", 50.0, 10.0);
        assert!(lines.len() >= 2);
    }

    // ---- TextLine tests ----

    #[test]
    fn text_line_new_empty() {
        let line = TextLine::new();
        assert!(line.is_empty());
        assert_eq!(line.width, 0.0);
        assert_eq!(line.height, 0.0);
    }

    #[test]
    fn text_line_push_span_accumulates_width() {
        let mut line = TextLine::new();
        let span = TextSpan::new("Hi", style(14.0));
        let expected_w = span.estimated_width();
        line.push_span(span);
        assert!((line.width - expected_w).abs() < 1e-5);
    }

    #[test]
    fn text_line_height_max_of_spans() {
        let mut line = TextLine::new();
        line.push_span(TextSpan::new("small", style(10.0)));
        line.push_span(TextSpan::new("large", style(20.0)));
        // height is max(font_size * 1.3) = 20.0 * 1.3 = 26.0
        assert!((line.height - 26.0).abs() < 1e-5);
    }

    // ---- TextLayout tests ----

    #[test]
    fn text_layout_single_line() {
        let spans = vec![TextSpan::new("Hi", style(14.0))];
        let layout = TextLayout::layout(&spans, 1000.0);
        assert_eq!(layout.line_count(), 1);
        assert_eq!(layout.lines[0].plain_text().trim(), "Hi");
    }

    #[test]
    fn text_layout_wraps_long_paragraph() {
        // Use a small max_width to force wrapping.
        // "one two three four" with max_width = 50, font_size = 14
        // char_width ≈ 14 * 0.55 = 7.7
        // "one" = 3 * 7.7 = 23.1; "two" = 3 * 7.7 = 23.1; space = 7.7
        // "one two" = 23.1 + 7.7 + 23.1 = 53.9 > 50 → wrap
        let spans = vec![TextSpan::new("one two three four", style(14.0))];
        let layout = TextLayout::layout(&spans, 50.0);
        assert!(layout.line_count() > 1, "expected wrapping, got {} lines", layout.line_count());
    }

    #[test]
    fn text_layout_total_height() {
        let spans = vec![TextSpan::new("one two three", style(14.0))];
        let layout = TextLayout::layout(&spans, 50.0);
        assert!(layout.total_height() > 0.0);
        // Each line contributes height * line_spacing (1.4)
        let expected = layout.lines.iter().map(|l| l.height * 1.4).sum::<f32>();
        assert!((layout.total_height() - expected).abs() < 1e-5);
    }
}
