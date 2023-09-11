use crate::{
    action::Action,
    app::{App, AppReturn, SelectedField, TextBox},
    database::IOEvent,
    key::Key,
    task::TaskDate,
};

pub async fn do_action(app: &mut App, key: Key) -> AppReturn {
    let current_field: &mut TextBox = match app.task_edit_field {
        SelectedField::Name => &mut app.name_edit,
        SelectedField::Year => &mut app.year_edit,
        SelectedField::Month => &mut app.month_edit,
        SelectedField::Date => &mut app.date_edit,
    };

    match key {
        Key::Esc => {
            app.disable_pop_up();
        }
        Key::Number(c) | Key::Char(c) => {
            let proposed_edit = TextBox {
                text: format!("{}{}", current_field.text, c),
                index: current_field.index + 1,
            };
            if verify_name(&proposed_edit.text) {
                *current_field = proposed_edit;
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
        Key::Tab => {
            let next = match app.task_edit_field {
                SelectedField::Name => SelectedField::Year,
                SelectedField::Year => SelectedField::Month,
                SelectedField::Month => SelectedField::Date,
                SelectedField::Date => SelectedField::Name,
            };

            app.task_edit_field = next;
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
