use crate::{
    action::Action,
    app::{App, AppPopUp, SelectedField},
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
    let task_editor_width = 40;
    let task_editor_height = 9;

    let frame_size = f.size();
    let editor_area = Rect::new(
        frame_size.width.saturating_sub(task_editor_width) / 2,
        frame_size.height.saturating_sub(task_editor_height) / 2,
        task_editor_width.min(frame_size.width),
        task_editor_height.min(frame_size.height),
    );
    f.render_widget(Clear, editor_area);

    let hint_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(90), Constraint::Min(1)])
        .split(editor_area);

    f.render_widget(
        Block::new().title("Edit Task").borders(Borders::ALL),
        hint_layout[0],
    );

    let hint = Paragraph::new("Scroll[Tab]  Submit[Shift-Enter]");
    f.render_widget(hint, hint_layout[1]);

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(1)
        .split(hint_layout[0]);

    let textarea = Paragraph::new(app.name_edit.text.clone())
        .block(Block::new().title("Name").borders(Borders::ALL))
        .style(Style::default())
        .wrap(Wrap { trim: true });

    f.render_widget(textarea, vertical_layout[0]);

    let date_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ])
        .split(vertical_layout[1]);

    let year = Paragraph::new(app.year_edit.text.clone())
        .block(Block::new().title("Y").borders(Borders::ALL));
    f.render_widget(year, date_layout[0]);

    let month = Paragraph::new(app.month_edit.text.clone())
        .block(Block::new().title("M").borders(Borders::ALL));
    f.render_widget(month, date_layout[2]);

    let day = Paragraph::new(app.date_edit.text.clone())
        .block(Block::new().title("D").borders(Borders::ALL));
    f.render_widget(day, date_layout[4]);

    let (active_area, active_index) = match app.task_edit_field {
        SelectedField::Name => (vertical_layout[0], app.name_edit.index),
        SelectedField::Year => (date_layout[0], app.year_edit.index),
        SelectedField::Month => (date_layout[2], app.month_edit.index),
        SelectedField::Date => (date_layout[4], app.date_edit.index),
    };
    f.set_cursor(active_area.x + active_index as u16 + 1, active_area.y + 1);
}
