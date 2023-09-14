mod task_editor;
mod upcoming;

use crate::{database::IOEvent, key::Key, task::TaskList};
use ratatui::widgets::ListState;

#[derive(Clone)]
pub enum AppMode {
    Upcoming,
}

pub enum AppPopUp {
    TaskEditor,
}

#[derive(Clone, Default)]
pub struct TextBox {
    pub text: String,
    pub index: usize,
    pub max_length: usize,
}

#[derive(PartialEq)]
pub enum SelectedField {
    Name,
    Year,
    Month,
    Date,
}

pub struct App {
    io_tx: tokio::sync::mpsc::Sender<IOEvent>,

    pub mode: AppMode,
    pub pop_up: Option<AppPopUp>,

    pub task_edit_field: SelectedField,
    pub name_edit: TextBox,
    pub year_edit: TextBox,
    pub month_edit: TextBox,
    pub date_edit: TextBox,

    pub status_text: String,
    pub keybind_hints: String,

    pub task_list: TaskList,
    pub task_list_state: ListState,
}

#[derive(PartialEq)]
pub enum AppReturn {
    Continue,
    Quit,
}

impl App {
    pub fn new(io_tx: tokio::sync::mpsc::Sender<IOEvent>) -> Self {
        Self {
            io_tx,
            mode: AppMode::Upcoming,
            pop_up: None,
            name_edit: TextBox {
                max_length: 36,
                ..Default::default()
            },
            year_edit: TextBox {
                max_length: 4,
                ..Default::default()
            },
            month_edit: TextBox {
                max_length: 2,
                ..Default::default()
            },
            date_edit: TextBox {
                max_length: 2,
                ..Default::default()
            },
            task_edit_field: SelectedField::Name,
            status_text: "initializing".to_string(),
            keybind_hints: "".to_string(),
            task_list: TaskList::new(),
            task_list_state: ListState::default(),
        }
    }

    // Call to perform initial data loads
    pub async fn initialize(&mut self) {
        self.dispatch(IOEvent::GrabUpcoming).await;
        self.switch_mode(AppMode::Upcoming);
        self.status_text = "initialized".to_string();
    }

    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if self.pop_up.is_none() {
            match key {
                Key::Char('i') => {
                    self.enable_pop_up(AppPopUp::TaskEditor);
                    AppReturn::Continue
                }
                _ => match self.mode {
                    AppMode::Upcoming => upcoming::do_action(self, key).await,
                },
            }
        } else {
            let p = self.pop_up.as_ref().unwrap();
            match p {
                AppPopUp::TaskEditor => task_editor::do_action(self, key).await,
            }
        }
    }

    fn switch_mode(&mut self, mode: AppMode) {
        match mode {
            AppMode::Upcoming => {
                upcoming::initialize(self);
            }
        }

        self.mode = mode;
    }

    fn enable_pop_up(&mut self, pop_up: AppPopUp) {
        match pop_up {
            AppPopUp::TaskEditor => {
                task_editor::initialize(self);
            }
        }

        self.pop_up = Some(pop_up);
    }

    pub fn disable_pop_up(&mut self) {
        self.pop_up = None;
        self.switch_mode(self.mode.clone());
    }

    pub async fn update_on_tick(&self) -> AppReturn {
        AppReturn::Continue
    }

    // Dispatch database work to seperate thread
    pub async fn dispatch(&self, action: IOEvent) {
        // TODO: handle error
        if self.io_tx.send(action).await.is_err() {
            panic!("database task not receiving messages");
        }
    }
}
