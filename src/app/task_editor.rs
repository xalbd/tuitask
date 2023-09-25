use crate::{
    app::{App, AppReturn, SelectedField, TextBox},
    database::IOEvent,
    key::Key,
    task::{Task, TaskDate},
};
use chrono::{Datelike, NaiveDate};

pub async fn do_action(app: &mut App, key: Key) -> AppReturn {
    let current_field: &mut TextBox = match app.task_edit_field {
        SelectedField::Name => &mut app.name_edit,
        SelectedField::Year => &mut app.year_edit,
        SelectedField::Month => &mut app.month_edit,
        SelectedField::Date => &mut app.date_edit,
    };

    match key {
        Key::Esc | Key::Ctrl('c') => {
            app.disable_pop_up();
        }
        Key::Number(c) | Key::Char(c) => {
            let mut proposed_text = current_field.text.clone();
            proposed_text.insert(current_field.index, c);

            if proposed_text.len() <= current_field.max_length
                && match app.task_edit_field {
                    SelectedField::Name => true,
                    _ => proposed_text.parse::<isize>().is_ok(),
                }
            {
                *current_field = TextBox {
                    text: proposed_text,
                    index: current_field.index + 1,
                    max_length: current_field.max_length,
                };
            }
        }
        Key::Left => {
            if current_field.index > 0 {
                current_field.index -= 1;
            }
        }
        Key::Right => {
            if current_field.index < current_field.text.len() {
                current_field.index += 1;
            }
        }
        Key::Backspace => {
            if current_field.index > 0 {
                current_field.text.remove(current_field.index - 1);
                current_field.index -= 1;
            }
        }
        Key::Enter => {
            // TODO: needs to automatically select the new/edited task in the list
            if !app.name_edit.text.is_empty()
                && app.year_edit.text.parse::<i32>().is_ok()
                && app.month_edit.text.parse::<u32>().is_ok()
                && app.date_edit.text.parse::<u32>().is_ok()
                && NaiveDate::from_ymd_opt(
                    app.year_edit.text.parse::<i32>().unwrap(),
                    app.month_edit.text.parse::<u32>().unwrap(),
                    app.date_edit.text.parse::<u32>().unwrap(),
                )
                .is_some()
            {
                if app.editing_task {
                    let editing_task = &mut app.task_list.tasks[app.task_list.selected_index];
                    *editing_task = Task {
                        due_date: NaiveDate::from_ymd_opt(
                            app.year_edit.text.parse::<i32>().unwrap(),
                            app.month_edit.text.parse::<u32>().unwrap(),
                            app.date_edit.text.parse::<u32>().unwrap(),
                        )
                        .unwrap(),
                        name: app.name_edit.text.clone(),
                        ..editing_task.clone()
                    };

                    let io_event = IOEvent::UpdateTask(editing_task.clone());
                    app.dispatch(io_event).await;
                    app.disable_pop_up();
                } else {
                    let new_task = Task {
                        due_date: NaiveDate::from_ymd_opt(
                            app.year_edit.text.parse::<i32>().unwrap(),
                            app.month_edit.text.parse::<u32>().unwrap(),
                            app.date_edit.text.parse::<u32>().unwrap(),
                        )
                        .unwrap(),
                        name: app.name_edit.text.clone(),
                        completed: false,
                        category: app.categories[0].clone(), // TODO: need to be able to edit this
                        id: -1,
                    };
                    app.dispatch(IOEvent::CreateTask(new_task)).await;
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
    let (name, year, month, date): (String, String, String, String);
    match &app.task_list.current_taskdate {
        TaskDate::Task(t) => {
            (name, year, month, date) = (
                if app.editing_task {
                    t.name.clone()
                } else {
                    "".to_string()
                },
                t.due_date.year().to_string(),
                t.due_date.month().to_string(),
                t.due_date.day().to_string(),
            );
        }
        TaskDate::Date(d) => {
            (name, year, month, date) = (
                "".to_string(),
                d.year().to_string(),
                d.month().to_string(),
                d.day().to_string(),
            );
        }
    };

    app.name_edit = TextBox {
        index: name.len(),
        text: name,
        ..app.name_edit
    };
    app.year_edit = TextBox {
        index: year.len(),
        text: year,
        ..app.year_edit
    };
    app.month_edit = TextBox {
        index: month.len(),
        text: month,
        ..app.month_edit
    };
    app.date_edit = TextBox {
        index: date.len(),
        text: date,
        ..app.date_edit
    };
    app.keybind_hints = "Exit[esc/ctrl-c]".to_string();
    app.task_edit_field = SelectedField::Name;

    AppReturn::Continue
}
