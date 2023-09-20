use crate::{
    app::{App, AppReturn},
    database::IOEvent,
    key::Key,
    task::TaskDate,
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
        Key::Enter => {
            if let TaskDate::Task(_) = app.task_list.current_taskdate {
                let editing_task = &mut app.task_list.tasks[app.task_list.selected_index];
                editing_task.completed = !editing_task.completed;

                let io_event = IOEvent::UpdateTask(editing_task.clone());
                app.dispatch(io_event).await;
            }
        }
        Key::Char('q') | Key::Esc | Key::Ctrl('c') => return AppReturn::Quit,
        _ => (),
    }
    AppReturn::Continue
}

pub fn initialize(app: &mut App) {
    app.keybind_hints =
        "Scroll[j/k]  [R]eset  [E]dit  [A]dd  Complete[Enter] [Q]uit[esc/ctrl-c]".to_string();
}
