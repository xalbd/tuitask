use crate::{app::App, task::Task};
use sqlx::Row;
use std::sync::Arc;

pub enum IOEvent {
    GrabUpcoming,
    UpdateTask(Task),
    CreateTask(Task),
}

pub struct IOHandler {
    app: Arc<tokio::sync::Mutex<App>>,
    pub db_pool: sqlx::PgPool,
}

impl IOHandler {
    pub fn new(app: Arc<tokio::sync::Mutex<App>>, db_pool: sqlx::PgPool) -> Self {
        Self { app, db_pool }
    }

    pub async fn handle_io(&mut self, io_event: IOEvent) -> Result<(), Box<dyn std::error::Error>> {
        match io_event {
            IOEvent::GrabUpcoming => self.grab_upcoming().await?,
            IOEvent::UpdateTask(t) => self.update_task(t).await?,
            IOEvent::CreateTask(t) => self.create_task(t).await?,
        };

        Ok(())
    }

    // Loads all incomplete tasks and fills in dates; loads into app for use in display in Upcoming mode
    async fn grab_upcoming(&mut self) -> Result<(), sqlx::Error> {
        let rows = sqlx::query("SELECT * FROM task WHERE completed = FALSE")
            .fetch_all(&self.db_pool)
            .await?;

        let task_list: Vec<Task> = rows
            .iter()
            .map(|r| Task {
                id: r.get("id"),
                name: r.get("name"),
                due_date: r.get("due_date"),
                completed: r.get("completed"),
            })
            .collect();

        let mut app = self.app.lock().await;
        app.task_list.tasks = task_list;
        app.task_list_state.select(Some(0));

        Ok(())
    }

    // Updates task in database
    async fn update_task(&mut self, t: Task) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE task SET name = $1, due_date = $2, completed = $3 WHERE id = $4")
            .bind(t.name)
            .bind(t.due_date)
            .bind(t.completed)
            .bind(t.id)
            .execute(&self.db_pool)
            .await?;

        Ok(())
    }

    async fn create_task(&mut self, t: Task) -> Result<(), sqlx::Error> {
        let created_task_id = sqlx::query(
            "INSERT INTO task (name, due_date, completed) VALUES ($1, $2, $3) RETURNING id",
        )
        .bind(t.name.clone())
        .bind(t.due_date)
        .bind(t.completed)
        .fetch_one(&self.db_pool)
        .await?;

        let mut app = self.app.lock().await;
        app.task_list.tasks.push(Task {
            id: created_task_id.get("id"),
            ..t
        });
        app.task_list.tasks.sort();

        Ok(())
    }
}
