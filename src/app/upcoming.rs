use crate::{
    app::{action::Action, App, AppReturn},
    database::IOEvent,
    key::Key,
    task::TaskDate,
};
use chrono::{Days, NaiveDate};

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
            Action::DecreaseDueDate => {
                if let TaskDate::Task(mut t) =
                    app.task_list[app.task_list_state.selected().unwrap()].clone()
                {
                    app.task_list
                        .remove(app.task_list_state.selected().unwrap());
                    t.due_date = t.due_date - Days::new(1);
                    ensure_date_present(app, t.due_date);
                    if let Some(pos) = app.task_list.iter().position(|x| {
                        if let TaskDate::Date(y) = x {
                            *y == t.due_date
                        } else {
                            false
                        }
                    }) {
                        app.task_list.insert(pos + 1, TaskDate::Task(t.clone()));
                        app.task_list_state.select(Some(pos + 1));
                    }
                    app.dispatch(IOEvent::UpdateTask(t)).await;
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
        Action::DecreaseDueDate.to_string(),
    ];
}

pub fn ensure_date_present(app: &mut App, d: NaiveDate) {
    if app.task_list.is_empty() {
        app.task_list.push(TaskDate::Date(d));
    } else {
        if let TaskDate::Date(first_date) = app.task_list[0] {
            if d < first_date {
                let mut missing_dates: Vec<TaskDate> = vec![];
                let mut current = d;
                while current < first_date {
                    missing_dates.push(TaskDate::Date(current.clone()));
                    current = current + Days::new(1);
                }
                app.task_list.splice(..0, missing_dates);
                return;
            }
        }
        if let TaskDate::Date(last_date) = app.task_list[app.task_list.len().saturating_sub(1)] {
            if d > last_date {
                let mut current = last_date + Days::new(1);
                while current <= d {
                    app.task_list.push(TaskDate::Date(current.clone()));
                    current = current + Days::new(1);
                }
            }
        }
    }
}
