mod upcoming;

use crate::{action::Action, database::IOEvent, key::Key, task::TaskDate};
use ratatui::widgets::ListState;

pub enum AppMode {
    Upcoming,
}

pub struct App {
    io_tx: tokio::sync::mpsc::Sender<IOEvent>,
    pub mode: AppMode,
    pub status_text: String,
    pub allowed_actions: Vec<Action>,
    pub task_list: Vec<TaskDate>,
    pub task_list_state: ListState,
    pub db_pool: sqlx::PgPool,
}

#[derive(PartialEq)]
pub enum AppReturn {
    Continue,
    Quit,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IOEvent>, db_pool: sqlx::PgPool) -> Self {
        Self {
            io_tx,
            mode: AppMode::Upcoming,
            status_text: "initializing".to_string(),
            allowed_actions: vec![],
            task_list: vec![],
            task_list_state: ListState::default(),
            db_pool,
        }
    }

    // Call to perform initial data loads
    pub async fn initialize(&mut self) {
        self.dispatch(IOEvent::GrabUpcoming).await;
        self.switch_mode(AppMode::Upcoming);
        self.status_text = "initialized".to_string();
    }

    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        match key {
            Key::Char('q') | Key::Ctrl('c') => AppReturn::Quit,
            _ => match self.mode {
                AppMode::Upcoming => upcoming::do_action(self, key).await,
            },
        }
    }

    pub fn switch_mode(&mut self, mode: AppMode) {
        // Update keybind hints
        match mode {
            AppMode::Upcoming => {
                upcoming::update_allowed_actions(self);
            }
        }

        // Indicate mode to both app logic and UI
        self.mode = mode;
    }

    pub async fn update_on_tick(&self) -> AppReturn {
        AppReturn::Continue
    }

    // Dispatch database work to seperate thread
    pub async fn dispatch(&self, action: IOEvent) {
        // TODO: handle error
        let _ = self.io_tx.send(action).await;
    }
}
