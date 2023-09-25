mod categories;
mod task_editor;
mod upcoming;

use crate::{
    category::Category,
    database::IOEvent,
    key::Key,
    task::{TaskDate, TaskList},
};
use ratatui::widgets::ListState;

#[derive(Clone, PartialEq)]
pub enum AppMode {
    Upcoming,
    Categories,
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

impl TextBox {
    fn new(max_length: usize) -> Self {
        TextBox {
            max_length,
            ..Default::default()
        }
    }
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
    pub editing_task: bool,

    pub status_text: String,
    pub keybind_hints: String,

    pub task_list: TaskList,
    pub task_list_state: ListState,

    pub categories: Vec<Category>,
    pub category_list_state: ListState,
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
            task_edit_field: SelectedField::Name,
            name_edit: TextBox::new(36), // TODO: can this be infinite/higher?
            year_edit: TextBox::new(4),
            month_edit: TextBox::new(2),
            date_edit: TextBox::new(2),
            editing_task: false,
            status_text: "".to_string(),
            keybind_hints: "".to_string(),
            task_list: TaskList::new(),
            task_list_state: ListState::default().with_selected(Some(0)),
            categories: Vec::new(),
            category_list_state: ListState::default().with_selected(Some(0)),
        }
    }

    // Call to perform initial data loads
    pub async fn initialize(&mut self) {
        self.dispatch(IOEvent::LoadData).await;
        self.switch_mode(AppMode::Upcoming);
    }

    pub async fn do_action(&mut self, key: Key) -> AppReturn {
        if self.pop_up.is_none() {
            match key {
                Key::Number('1') => self.switch_mode(AppMode::Upcoming),
                Key::Number('2') => self.switch_mode(AppMode::Categories),
                _ => match self.mode {
                    AppMode::Upcoming => match key {
                        Key::Char('e') => {
                            if let TaskDate::Task(t) = &self.task_list.current_taskdate {
                                if !t.completed {
                                    self.editing_task = true;
                                    self.enable_pop_up(AppPopUp::TaskEditor);
                                }
                            }
                            AppReturn::Continue
                        }
                        Key::Char('a') => {
                            self.editing_task = false;
                            self.enable_pop_up(AppPopUp::TaskEditor);
                            AppReturn::Continue
                        }
                        _ => upcoming::do_action(self, key).await,
                    },
                    AppMode::Categories => categories::do_action(self, key).await,
                },
            }
        } else {
            let p = self.pop_up.as_ref().unwrap();
            match p {
                AppPopUp::TaskEditor => task_editor::do_action(self, key).await,
            }
        }
    }

    fn switch_mode(&mut self, mode: AppMode) -> AppReturn {
        self.task_list.tasks.retain(|t| !t.completed); // Purge completed tasks when switching views

        match mode {
            AppMode::Upcoming => {
                upcoming::initialize(self);
            }
            AppMode::Categories => {
                categories::initialize(self);
            }
        }

        self.mode = mode;
        AppReturn::Continue
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
        if self.io_tx.send(action).await.is_err() {
            panic!("database thread not receiving messages");
        }
    }
}
