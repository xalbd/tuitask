use crate::{
    app::App,
    task::{Task, TaskDate},
};
use chrono::{Days, NaiveDate};
use std::sync::Arc;

pub enum IOEvent {
    GrabUpcoming,
    UpdateTask(Task),
}

pub struct IOHandler {
    app: Arc<tokio::sync::Mutex<App>>,
}

impl IOHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>) -> Self {
        Self { app }
    }

    pub async fn handle_io(&mut self, io_event: IOEvent) -> Result<(), Box<dyn std::error::Error>> {
        match io_event {
            IOEvent::GrabUpcoming => self.grab_upcoming().await?,
            IOEvent::UpdateTask(t) => self.update_task(t).await?,
        };

        Ok(())
    }

    // Loads all incomplete tasks and fills in dates; loads into app for use in display in Upcoming mode
    async fn grab_upcoming(&mut self) -> Result<(), sqlx::Error> {
        let mut app = self.app.lock().await;

        let rows = sqlx::query!("SELECT * FROM task WHERE completed = FALSE ORDER BY due_date")
            .fetch_all(&app.db_pool)
            .await?;

        app.task_list = vec![];
        let new_selection = app.task_list_state.selected().unwrap_or(0);
        app.task_list_state.select(Some(new_selection));

        let mut prev_date: Option<NaiveDate> = None;
        for r in rows {
            let d = match prev_date {
                Some(mut d) => {
                    while d < r.due_date {
                        d = d + Days::new(1);
                        app.task_list.push(TaskDate::Date(d));
                    }
                    d
                }
                None => {
                    let d: NaiveDate = r.due_date;
                    app.task_list.push(TaskDate::Date(d));
                    d
                }
            };
            prev_date = Some(d);

            app.task_list.push(TaskDate::Task(Task {
                id: r.id,
                name: r.name,
                due_date: r.due_date,
                completed: r.completed,
            }));
        }

        Ok(())
    }

    // Updates task in database
    async fn update_task(&mut self, t: Task) -> Result<(), sqlx::Error> {
        let app = self.app.lock().await;

        sqlx::query!(
            "UPDATE task SET name = $1, due_date = $2, completed = $3 WHERE id = $4",
            t.name,
            t.due_date,
            t.completed,
            t.id
        )
        .execute(&app.db_pool)
        .await?;

        Ok(())
    }
}
