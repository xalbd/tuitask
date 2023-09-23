use std::cmp::min;

use crate::{
    app::{App, AppReturn},
    key::Key,
};

pub async fn do_action(app: &mut App, key: Key) -> AppReturn {
    match key {
        Key::Char('j') | Key::Down => {
            app.category_list_state.select(Some(min(
                app.categories.len().saturating_sub(1),
                app.category_list_state.selected().unwrap() + 1,
            )));
        }
        Key::Char('k') | Key::Up => {
            app.category_list_state.select(Some(
                app.category_list_state
                    .selected()
                    .unwrap()
                    .saturating_sub(1),
            ));
        }
        Key::Char('q') | Key::Esc | Key::Ctrl('c') => return AppReturn::Quit,
        _ => (),
    }
    AppReturn::Continue
}

pub fn initialize(app: &mut App) {
    app.keybind_hints = "Scroll[j/k]  [R]eset  [E]dit  [A]dd  [Q]uit[esc/ctrl-c]".to_string();
}
