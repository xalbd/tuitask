use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(PartialEq)]
pub enum Key {
    Number(char),
    Char(char),
    Ctrl(char),
    Enter,
    Esc,
    Tab,
    ShiftTab,
    Backspace,
    Left,
    Right,
    Up,
    Down,
    Unused,
}

// Converts crossterm backend key events to custom enum
impl From<KeyEvent> for Key {
    fn from(value: KeyEvent) -> Self {
        match value {
            KeyEvent {
                code: KeyCode::Char(c),
                modifiers: KeyModifiers::CONTROL,
                ..
            } => Key::Ctrl(c),
            KeyEvent {
                code: KeyCode::Char(c),
                ..
            } => match c.to_digit(10) {
                Some(_) => Key::Number(c),
                None => Key::Char(c),
            },
            KeyEvent {
                code: KeyCode::Enter,
                ..
            } => Key::Enter,
            KeyEvent {
                code: KeyCode::BackTab,
                ..
            } => Key::ShiftTab,
            KeyEvent {
                code: KeyCode::Tab, ..
            } => Key::Tab,
            KeyEvent {
                code: KeyCode::Esc, ..
            } => Key::Esc,
            KeyEvent {
                code: KeyCode::Backspace,
                ..
            } => Key::Backspace,
            KeyEvent {
                code: KeyCode::Up, ..
            } => Key::Up,
            KeyEvent {
                code: KeyCode::Down,
                ..
            } => Key::Down,
            KeyEvent {
                code: KeyCode::Left,
                ..
            } => Key::Left,
            KeyEvent {
                code: KeyCode::Right,
                ..
            } => Key::Right,
            _ => Key::Unused,
        }
    }
}
