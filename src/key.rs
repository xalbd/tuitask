use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(PartialEq)]
pub enum Key {
    Char(char),
    Ctrl(char),
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
            } => Key::Char(c),
            _ => Key::Unused,
        }
    }
}
