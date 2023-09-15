use crate::{
    app::{App, AppPopUp, SelectedField},
    task::TaskDate,
};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Max(100),
            Constraint::Length(1),
        ])
        .split(f.size());

    let title = Paragraph::new("Upcoming".to_string());
    f.render_widget(title, chunks[0]);

    let height = chunks[1].height as usize;

    let list_items: Vec<ListItem> = app
        .task_list
        .get_upcoming_list(app.task_list_state.selected().unwrap_or(0), height)
        .iter()
        .map(|i| {
            ListItem::new(Span::raw(match i {
                TaskDate::Date(d) => d.to_string(),
                TaskDate::Task(t) => format!(" {}", t.name),
            }))
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
        .highlight_symbol(">");
    f.render_stateful_widget(list, chunks[1], &mut app.task_list_state);

    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(chunks[2]);

    let hint_text = Paragraph::new(app.keybind_hints.clone());
    f.render_widget(hint_text, footer_layout[0]);

    let status_text = Paragraph::new(app.status_text.to_string());
    f.render_widget(status_text, footer_layout[1]);

    if app.pop_up.is_some() {
        match app.pop_up.as_ref().unwrap() {
            AppPopUp::TaskEditor => {
                draw_task_editor(f, app);
            }
        }
    }
}

fn draw_task_editor<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let task_editor_width = 40; // TODO: need to be changed to constants
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
        .constraints(vec![Constraint::Min(8), Constraint::Min(1)])
        .split(editor_area);

    f.render_widget(
        Block::new()
            .title(if app.editing_task {
                if let TaskDate::Task(t) = &app.task_list.current_taskdate {
                    format!("Edit \"{}\"", t.name) // TODO: overflow behavior
                } else {
                    "Edit Task".to_string()
                }
            } else {
                "Add Task".to_string()
            })
            .borders(Borders::ALL),
        hint_layout[0],
    );

    let hint = Paragraph::new("Scroll[Tab]  Submit[Enter]");
    f.render_widget(hint, hint_layout[1]);

    let vertical_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
        .margin(1)
        .split(hint_layout[0]);

    let textarea = Paragraph::new(app.name_edit.text.clone())
        .block(Block::new().title("Name").borders(Borders::ALL));
    f.render_widget(textarea, vertical_layout[0]);

    let date_blocks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Max(12),
            Constraint::Max(4),
            Constraint::Min(0),
        ])
        .split(vertical_layout[1]);

    let date_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Max(6),
            Constraint::Max(4),
            Constraint::Max(4),
            Constraint::Min(0),
        ])
        .split(if app.editing_task {
            date_blocks[2]
        } else {
            vertical_layout[1]
        });

    if app.editing_task {
        let old_date = Paragraph::new(match &app.task_list.current_taskdate {
            TaskDate::Task(t) => t.due_date.to_string(),
            TaskDate::Date(d) => d.to_string(),
        })
        .block(Block::new().borders(Borders::ALL));
        f.render_widget(old_date, date_blocks[0]);

        let arrow = Paragraph::new("->").block(Block::new().padding(Padding::uniform(1)));
        f.render_widget(arrow, date_blocks[1]);
    }

    let year = Paragraph::new(app.year_edit.text.clone())
        .block(Block::new().title("Y").borders(Borders::ALL));
    f.render_widget(year, date_layout[0]);

    let month = Paragraph::new(app.month_edit.text.clone())
        .block(Block::new().title("M").borders(Borders::ALL));
    f.render_widget(month, date_layout[1]);

    let day = Paragraph::new(app.date_edit.text.clone())
        .block(Block::new().title("D").borders(Borders::ALL));
    f.render_widget(day, date_layout[2]);

    let (active_area, active_index) = match app.task_edit_field {
        SelectedField::Name => (vertical_layout[0], app.name_edit.index),
        SelectedField::Year => (date_layout[0], app.year_edit.index),
        SelectedField::Month => (date_layout[1], app.month_edit.index),
        SelectedField::Date => (date_layout[2], app.date_edit.index),
    };
    f.set_cursor(active_area.x + active_index as u16 + 1, active_area.y + 1);
}
