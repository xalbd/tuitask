use crate::{
    app::{App, AppReturn},
    key::Key,
};

pub async fn do_action(app: &mut App, key: Key) -> AppReturn {
    match key {
        Key::Char('j') | Key::Down => {
            app.task_list_state
                .select(Some(match app.task_list_state.selected() {
                    Some(i) => i + 1,
                    None => 0,
                }));
        }
        Key::Char('k') | Key::Up => {
            app.task_list_state.select(Some(
                app.task_list_state
                    .selected()
                    .unwrap_or(0)
                    .saturating_sub(1),
            ));
        }
        Key::Char('r') => {
            app.task_list_state.select(Some(0));
        }
        Key::Char('q') | Key::Esc | Key::Ctrl('c') => return AppReturn::Quit,
        _ => (),
    }
    AppReturn::Continue
}

pub fn initialize(app: &mut App) {
    app.keybind_hints = "Scroll[j/k]  [R]eset  [E]dit  [A]dd  [Q]uit[esc/ctrl-c]".to_string();
}
