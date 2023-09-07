use crate::{action::Action, app::App, task::TaskDate};
use chrono::{naive::Days, NaiveDate};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw<B>(f: &mut Frame<B>, app: &mut App)
where
    B: Backend,
{
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(1),
                Constraint::Max(100),
                Constraint::Length(1),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new(app.status_text.to_string())
        .style(Style::default().fg(Color::LightCyan))
        .block(Block::default().style(Style::default().fg(Color::White)));
    f.render_widget(title, chunks[0]);

    let mut prev_date: Option<NaiveDate> = match app.task_list.len() {
        0 => None,
        n => Some(match &app.task_list[n - 1] {
            TaskDate::Task(t) => t.due_date,
            TaskDate::Date(d) => *d,
        }),
    };
    let height = chunks[1].height;
    while prev_date.is_some()
        && (app.task_list.len() as isize) - (app.task_list_state.selected().unwrap_or(0) as isize)
            < (height as isize)
    {
        prev_date = Some(prev_date.unwrap() + Days::new(1));
        app.task_list.push(TaskDate::Date(prev_date.unwrap()))
    }

    let list_items: Vec<ListItem> = app
        .task_list
        .iter()
        .map(|i| {
            ListItem::new(Span::raw(match i {
                TaskDate::Date(d) => d.to_string(),
                TaskDate::Task(t) => format!(" {}", t.name),
            }))
        })
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Plain),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">");
    f.render_stateful_widget(list, chunks[1], &mut app.task_list_state);

    let hints: Vec<Span> = app
        .allowed_actions
        .iter()
        .map(Action::to_string)
        .map(Span::raw)
        .collect();
    let hint_text = Paragraph::new(Line::from(hints))
        .block(Block::default().style(Style::default().fg(Color::White)));
    f.render_widget(hint_text, chunks[2]);
}
