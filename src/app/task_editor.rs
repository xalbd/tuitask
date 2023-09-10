use crate::{
    action::Action,
    app::{App, AppReturn, TextBox},
    database::IOEvent,
    key::Key,
    task::TaskDate,
};

pub async fn do_action(app: &mut App, key: Key) -> AppReturn {
    match key {
        Key::Esc => {
            app.disable_pop_up();
        }
        Key::Number(c) | Key::Char(c) => {
            let proposed_edit = TextBox {
                text: format!("{}{}", app.name_edit.text, c),
                index: app.name_edit.index + 1,
            };
            if verify_name(&proposed_edit.text) {
                app.name_edit = proposed_edit;
            }
        }
        Key::Enter => {
            if !app.name_edit.text.is_empty() {
                if let TaskDate::Task(mut t) =
                    app.task_list[app.task_list_state.selected().unwrap()].clone()
                {
                    t.name = app.name_edit.text.clone();
                    app.task_list
                        .remove(app.task_list_state.selected().unwrap());
                    app.task_list.insert(
                        app.task_list_state.selected().unwrap(),
                        TaskDate::Task(t.clone()),
                    );
                    app.dispatch(IOEvent::UpdateTask(t)).await;
                    app.disable_pop_up();
                }
            }
        }
        _ => (),
    }
    AppReturn::Continue
}

pub fn initialize(app: &mut App) -> AppReturn {
    app.name_edit = TextBox {
        text: "".to_string(),
        index: 0,
    };
    app.allowed_actions = vec![Action::Quit];
    AppReturn::Continue
}

fn verify_name(name: &str) -> bool {
    name.len() < 15
}
