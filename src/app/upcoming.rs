use crate::{
    app::{App, AppReturn},
    key::Key,
};

pub async fn do_action(app: &mut App, key: Key) -> AppReturn {
    match key {
        Key::Char('j') | Key::Down => {
            let i = match app.task_list_state.selected() {
                Some(i) => i + 1,
                None => 0,
            };
            app.task_list_state.select(Some(i));
        }
        Key::Char('k') | Key::Up => {
            let i = match app.task_list_state.selected() {
                Some(i) => {
                    if i > 0 {
                        i - 1
                    } else {
                        i
                    }
                }
                None => 0,
            };
            app.task_list_state.select(Some(i));
        }
        Key::Char('a') => {
            app.task_list_state.select(Some(0));
        }
        Key::Char('q') | Key::Esc | Key::Ctrl('c') => return AppReturn::Quit,
        _ => (),
    }
    AppReturn::Continue
}

pub fn initialize(app: &mut App) {
    app.keybind_hints = "Scroll[j/k]  Reset[a]  Edit[i]  Quit[q/esc/ctrl-c]".to_string();
}
