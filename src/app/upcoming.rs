use crate::{
    app::{action::Action, App, AppReturn},
    database::IOEvent,
    key::Key,
    task::TaskDate,
};
use chrono::Days;

pub async fn do_action(app: &mut App, key: Key) -> AppReturn {
    if let Ok(action) = Action::try_from(key) {
        match action {
            Action::Next => {
                let i = match app.task_list_state.selected() {
                    Some(i) => i + 1,
                    None => 0,
                };
                app.task_list_state.select(Some(i));
            }
            Action::Previous => {
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
            Action::Reset => {
                app.task_list_state.select(Some(0));
            }
            Action::IncreaseDueDate => {
                let current_task = app
                    .task_list
                    .get_mut(app.task_list_state.selected().unwrap());

                match current_task {
                    Some(TaskDate::Task(t)) => {
                        t.due_date = t.due_date + Days::new(1); // TODO: this doesn't update the LOCATION of the task in the vector
                        let t_new = t.clone();
                        t.name.push_str("e");
                        app.dispatch(IOEvent::UpdateTask(t_new)).await;
                        app.dispatch(IOEvent::GrabUpcoming).await;
                    }
                    _ => (),
                }
            }
            _ => (),
        };
    }

    AppReturn::Continue
}

pub fn update_hints(app: &mut App) {
    app.keybind_hints = vec![
        Action::Next.to_string(),
        Action::Previous.to_string(),
        Action::Reset.to_string(),
        Action::IncreaseDueDate.to_string(),
    ];
}
