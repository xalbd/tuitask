use chrono::{offset::Local, Days, NaiveDate};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Task {
    pub due_date: NaiveDate,
    pub name: String,
    pub completed: bool,
    pub id: i32,
    pub category_name: String,
}

#[derive(Clone, PartialEq)]
pub enum TaskDate {
    Task(Task),
    Date(NaiveDate),
}

pub struct TaskList {
    pub tasks: Vec<Task>,
    pub current_taskdate: TaskDate, // updates when UI renders task list in some format
    pub selected_index: Option<usize>, // same as above, facilities editing/removing currently selected task
}

impl TaskList {
    pub fn new() -> Self {
        TaskList {
            tasks: Vec::new(),
            current_taskdate: TaskDate::Date(NaiveDate::from_ymd_opt(1, 1, 1).unwrap()),
            selected_index: None,
        }
    }

    pub fn get_upcoming_list(&mut self, selected: usize, buffer: usize) -> Vec<TaskDate> {
        let mut current_date = Local::now().date_naive();

        self.tasks.sort();
        let mut current_task = 0;

        let mut output: Vec<TaskDate> = Vec::new();
        while output.len() < selected + buffer {
            let new_item: TaskDate;
            if current_task < self.tasks.len() && self.tasks[current_task].due_date < current_date {
                if output.len() == selected {
                    self.selected_index = Some(current_task);
                }
                new_item = TaskDate::Task(self.tasks[current_task].clone());
                current_task += 1;
            } else {
                new_item = TaskDate::Date(current_date);
                current_date = current_date + Days::new(1);
            }

            if output.len() == selected {
                self.current_taskdate = new_item.clone();
            }
            output.push(new_item);
        }

        output
    }
}
