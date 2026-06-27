#[derive(Debug, Clone)]
pub enum InputEvent {
    MouseMove   { x: f32, y: f32 },
    MouseDown   { x: f32, y: f32, button: MouseButton },
    MouseUp     { x: f32, y: f32, button: MouseButton },
    KeyDown     { key: Key },
    KeyUp       { key: Key },
    Text        { character: char },
    WindowResized { width: u32, height: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseButton { Left, Right, Middle }

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Key {
    Enter, Escape, Space, Backspace, Tab,
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    Char(char),
}
