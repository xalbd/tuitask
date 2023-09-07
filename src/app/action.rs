use crate::key::Key;
use std::fmt::Display;

// Describes all possible actions in the app
pub enum Action {
    Quit,
    Next,
    Previous,
    Reset,
    IncreaseDueDate,
}

// Allow keybind hints to be displayed
impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Action::Quit => "Quit[q]  ",
            Action::Next => "Next[j]  ",
            Action::Previous => "Previous[k]  ",
            Action::Reset => "Reset[r]  ",
            Action::IncreaseDueDate => "Increase Due Date[d]",
        };
        write!(f, "{}", str)
    }
}

// Global implementation of keybinds
// TODO: allow rebinding/multiple binds for single key: move each screen into implementing a trait?
impl TryFrom<Key> for Action {
    type Error = &'static str;
    fn try_from(value: Key) -> Result<Self, Self::Error> {
        match value {
            Key::Char('q') | Key::Ctrl('c') => Ok(Action::Quit),
            Key::Char('j') => Ok(Action::Next),
            Key::Char('k') => Ok(Action::Previous),
            Key::Char('r') => Ok(Action::Reset),
            Key::Char('d') => Ok(Action::IncreaseDueDate),
            _ => Err("Could not convert key to action"),
        }
    }
}
