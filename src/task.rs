use chrono::NaiveDate;

#[derive(Clone)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub due_date: NaiveDate,
    pub completed: bool,
}

#[derive(Clone)]
pub enum TaskDate {
    Task(Task),
    Date(NaiveDate),
}
