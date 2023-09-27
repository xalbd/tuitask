use std::cmp::min;

use crate::{
    app::{App, AppReturn, SelectedField, TextBox},
    database::IOEvent,
    key::Key,
    task::{Task, TaskDate},
};
use chrono::{Datelike, NaiveDate};
use ratatui::widgets::ListState;

pub async fn do_action(app: &mut App, key: Key) -> AppReturn {
    match app.task_edit_field {
        SelectedField::Name => handle_textbox(&mut app.name_edit, &key, |_x| true),
        SelectedField::Year => {
            handle_textbox(&mut app.year_edit, &key, |x| x.parse::<isize>().is_ok())
        }
        SelectedField::Month => {
            handle_textbox(&mut app.month_edit, &key, |x| x.parse::<isize>().is_ok())
        }
        SelectedField::Date => {
            handle_textbox(&mut app.date_edit, &key, |x| x.parse::<isize>().is_ok())
        }
        SelectedField::Category => {
            handle_selector(app.categories.len(), &mut app.category_edit_state, &key)
        }
    };

    match key {
        Key::Esc | Key::Ctrl('c') => {
            app.disable_pop_up();
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
                        category: app.categories[app.category_edit_state.selected().unwrap()]
                            .clone(),
                        ..editing_task.clone()
                    };

                    let io_event = IOEvent::UpdateTask(editing_task.clone());
                    app.dispatch(io_event).await;
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
                        category: app.categories[app.category_edit_state.selected().unwrap()]
                            .clone(),
                        id: -1,
                    };
                    app.dispatch(IOEvent::CreateTask(new_task)).await;
                }

                app.disable_pop_up();
            }
        }
        Key::Tab => {
            app.task_edit_field = match app.task_edit_field {
                SelectedField::Name => SelectedField::Year,
                SelectedField::Year => SelectedField::Month,
                SelectedField::Month => SelectedField::Date,
                SelectedField::Date => SelectedField::Category,
                SelectedField::Category => SelectedField::Name,
            };
        }
        Key::ShiftTab => {
            app.task_edit_field = match app.task_edit_field {
                SelectedField::Name => SelectedField::Category,
                SelectedField::Year => SelectedField::Name,
                SelectedField::Month => SelectedField::Year,
                SelectedField::Date => SelectedField::Month,
                SelectedField::Category => SelectedField::Date,
            };
        }
        _ => (),
    };

    fn handle_textbox(current_field: &mut TextBox, key: &Key, verify: fn(&str) -> bool) {
        match key {
            Key::Number(c) | Key::Char(c) => {
                let mut proposed_text = current_field.text.clone();
                proposed_text.insert(current_field.index, *c);

                if proposed_text.len() <= current_field.max_length && verify(&proposed_text) {
                    *current_field = TextBox {
                        text: proposed_text,
                        index: current_field.index + 1,
                        ..*current_field
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
            _ => {}
        };
    }

    fn handle_selector(num_categories: usize, category_edit_state: &mut ListState, key: &Key) {
        match key {
            Key::Char('k') | Key::Up => {
                category_edit_state.select(Some(
                    category_edit_state.selected().unwrap().saturating_sub(1),
                ));
            }
            Key::Char('j') | Key::Down => {
                category_edit_state.select(Some(min(
                    num_categories.saturating_sub(1),
                    category_edit_state.selected().unwrap() + 1,
                )));
            }
            _ => {}
        };
    }

    AppReturn::Continue
}

pub fn initialize(app: &mut App) -> AppReturn {
    let (name, year, month, date, category_index): (String, String, String, String, usize) =
        match &app.task_list.current_taskdate {
            TaskDate::Task(t) => (
                if app.editing_task {
                    t.name.clone()
                } else {
                    "".to_string()
                },
                t.due_date.year().to_string(),
                t.due_date.month().to_string(),
                t.due_date.day().to_string(),
                app.categories
                    .iter()
                    .position(|c| c.id == t.category.id)
                    .unwrap(),
            ),
            TaskDate::Date(d) => (
                "".to_string(),
                d.year().to_string(),
                d.month().to_string(),
                d.day().to_string(),
                0,
            ),
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
    app.category_edit_state.select(Some(category_index));
    app.task_edit_field = SelectedField::Name;
    app.keybind_hints = "Exit[esc/ctrl-c]".to_string();

    AppReturn::Continue
}
