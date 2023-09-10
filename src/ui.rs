use crate::{
    action::Action,
    app::{App, AppPopUp},
    task::TaskDate,
};
use chrono::{naive::Days, NaiveDate};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
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

    if app.pop_up.is_some() {
        match app.pop_up.as_ref().unwrap() {
            AppPopUp::TaskEditor => {
                draw_task_editor(f, app);
            }
        }
    }
}

fn draw_task_editor<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let all_area = f.size();
    let desired_area = Rect::new(
        all_area.width.saturating_sub(20) / 2,
        all_area.height.saturating_sub(5) / 2,
        20.min(all_area.width),
        5.min(all_area.height),
    );
    f.render_widget(Clear, desired_area);

    let text = format!("{} ", app.name_edit.text);
    let underlined = app.name_edit.index;

    let textarea = Paragraph::new(text)
        .block(Block::new().title("Input Test").borders(Borders::ALL))
        .style(Style::default())
        .wrap(Wrap { trim: true });
    f.set_cursor(desired_area.x + underlined as u16 + 1, desired_area.y + 1);
    f.render_widget(textarea, desired_area);
}
