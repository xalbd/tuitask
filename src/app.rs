mod categories;
mod category_editor;
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
    CategoryEditor,
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
    Category,
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
    pub category_edit_state: ListState,
    pub editing_task: bool,
    pub editing_category: bool,

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
            name_edit: TextBox::new(36), // TODO: can this be infinite/higher? also this restricts both category and task name lengths now and needs to be dynamic, maybe
            year_edit: TextBox::new(4),
            month_edit: TextBox::new(2),
            date_edit: TextBox::new(2),
            category_edit_state: ListState::default(),
            editing_task: false,
            editing_category: false, //TODO: NEED TO SET THIS PROPERLy
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
                    AppMode::Categories => match key {
                        Key::Char('e') => {
                            self.editing_category = true;
                            self.enable_pop_up(AppPopUp::CategoryEditor);
                            AppReturn::Continue
                        }
                        Key::Char('a') => {
                            self.editing_category = false;
                            self.enable_pop_up(AppPopUp::CategoryEditor);
                            AppReturn::Continue
                        }
                        _ => categories::do_action(self, key).await,
                    },
                },
            }
        } else {
            let p = self.pop_up.as_ref().unwrap();
            match p {
                AppPopUp::TaskEditor => task_editor::do_action(self, key).await,
                AppPopUp::CategoryEditor => category_editor::do_action(self, key).await,
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
            AppPopUp::CategoryEditor => {
                category_editor::initialize(self);
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
