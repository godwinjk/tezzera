/// A position within a multi-line text layout.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TextCursor {
    pub line: usize,
    pub col: usize,
}

impl TextCursor {
    pub fn new(line: usize, col: usize) -> Self { Self { line, col } }

    /// Move cursor right by one character, wrapping to next line.
    pub fn advance(&mut self, line_lengths: &[usize]) {
        if self.line >= line_lengths.len() { return; }
        if self.col < line_lengths[self.line] {
            self.col += 1;
        } else if self.line + 1 < line_lengths.len() {
            self.line += 1;
            self.col = 0;
        }
    }

    /// Move cursor left by one character, wrapping to previous line.
    pub fn backspace(&mut self, line_lengths: &[usize]) {
        if self.col > 0 {
            self.col -= 1;
        } else if self.line > 0 {
            self.line -= 1;
            self.col = line_lengths.get(self.line).copied().unwrap_or(0);
        }
    }

    /// Move to start of line.
    pub fn home(&mut self) { self.col = 0; }

    /// Move to end of current line.
    pub fn end(&mut self, line_lengths: &[usize]) {
        self.col = line_lengths.get(self.line).copied().unwrap_or(0);
    }

    /// Move up one line (same column or clamped).
    pub fn up(&mut self, line_lengths: &[usize]) {
        if self.line > 0 {
            self.line -= 1;
            self.col = self.col.min(line_lengths.get(self.line).copied().unwrap_or(0));
        }
    }

    /// Move down one line.
    pub fn down(&mut self, line_lengths: &[usize]) {
        if self.line + 1 < line_lengths.len() {
            self.line += 1;
            self.col = self.col.min(line_lengths.get(self.line).copied().unwrap_or(0));
        }
    }

    /// Pixel x-offset of the cursor given a char_width.
    pub fn pixel_x(&self, char_width: f32) -> f32 {
        self.col as f32 * char_width
    }

    /// Pixel y-offset given a line_height.
    pub fn pixel_y(&self, line_height: f32) -> f32 {
        self.line as f32 * line_height
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_new() {
        let c = TextCursor::new(2, 5);
        assert_eq!(c.line, 2);
        assert_eq!(c.col, 5);
    }

    #[test]
    fn cursor_advance_within_line() {
        let mut c = TextCursor::new(0, 0);
        let lengths = [5, 3];
        c.advance(&lengths);
        assert_eq!(c.line, 0);
        assert_eq!(c.col, 1);
    }

    #[test]
    fn cursor_advance_wraps_to_next_line() {
        // col == line_length[0] → wrap
        let mut c = TextCursor::new(0, 5);
        let lengths = [5, 3];
        c.advance(&lengths);
        assert_eq!(c.line, 1);
        assert_eq!(c.col, 0);
    }

    #[test]
    fn cursor_backspace_within_line() {
        let mut c = TextCursor::new(0, 3);
        let lengths = [5, 3];
        c.backspace(&lengths);
        assert_eq!(c.line, 0);
        assert_eq!(c.col, 2);
    }

    #[test]
    fn cursor_backspace_wraps_to_prev_line() {
        let mut c = TextCursor::new(1, 0);
        let lengths = [5, 3];
        c.backspace(&lengths);
        assert_eq!(c.line, 0);
        assert_eq!(c.col, 5);
    }

    #[test]
    fn cursor_home() {
        let mut c = TextCursor::new(1, 4);
        c.home();
        assert_eq!(c.col, 0);
        assert_eq!(c.line, 1);
    }

    #[test]
    fn cursor_end() {
        let mut c = TextCursor::new(0, 1);
        let lengths = [7, 3];
        c.end(&lengths);
        assert_eq!(c.col, 7);
        assert_eq!(c.line, 0);
    }

    #[test]
    fn cursor_up() {
        let mut c = TextCursor::new(1, 3);
        let lengths = [5, 3];
        c.up(&lengths);
        assert_eq!(c.line, 0);
        assert_eq!(c.col, 3); // col <= line 0 length (5), so unchanged
    }

    #[test]
    fn cursor_down() {
        let mut c = TextCursor::new(0, 2);
        let lengths = [5, 3];
        c.down(&lengths);
        assert_eq!(c.line, 1);
        assert_eq!(c.col, 2); // col <= line 1 length (3), so unchanged
    }

    #[test]
    fn cursor_pixel_position() {
        let c = TextCursor::new(2, 4);
        assert!((c.pixel_x(8.0) - 32.0).abs() < 1e-5);
        assert!((c.pixel_y(20.0) - 40.0).abs() < 1e-5);
    }
}
