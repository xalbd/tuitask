use crate::{app::App, task::Task};
use sqlx::Row;
use std::sync::Arc;

pub enum IOEvent {
    LoadTasks,
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
            IOEvent::LoadTasks => self.load_tasks().await?,
            IOEvent::UpdateTask(t) => self.update_task(t).await?,
            IOEvent::CreateTask(t) => self.create_task(t).await?,
        };

        Ok(())
    }

    // Loads all incomplete tasks to task list
    async fn load_tasks(&mut self) -> Result<(), sqlx::Error> {
        self.update_status("loading tasks".to_string()).await;

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
        app.status_text = "tasks loaded".to_string();

        Ok(())
    }

    async fn update_task(&mut self, t: Task) -> Result<(), sqlx::Error> {
        self.update_status("updating task".to_string()).await;

        sqlx::query("UPDATE task SET name = $1, due_date = $2, completed = $3 WHERE id = $4")
            .bind(t.name)
            .bind(t.due_date)
            .bind(t.completed)
            .bind(t.id)
            .execute(&self.db_pool)
            .await?;

        self.update_status("update successful".to_string()).await;

        Ok(())
    }

    async fn create_task(&mut self, t: Task) -> Result<(), sqlx::Error> {
        self.update_status("creating task".to_string()).await;

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
        app.status_text = "task created".to_string();

        Ok(())
    }

    async fn update_status(&mut self, s: String) {
        let mut app = self.app.lock().await;
        app.status_text = s;
    }
}
