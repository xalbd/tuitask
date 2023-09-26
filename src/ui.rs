use crate::{
    app::{App, AppMode, AppPopUp, SelectedField},
    task::{Task, TaskDate},
};
use chrono::{Local, NaiveDate};
use ratatui::{
    prelude::{Backend, Constraint, Direction, Layout, Rect},
    style::{Modifier, Style, Stylize},
    symbols::DOT,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Padding, Paragraph, Tabs},
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

    let tabs = Tabs::new(
        ["Upcoming (1)", "Categories (2)"]
            .iter()
            .cloned()
            .map(Line::from)
            .collect(),
    )
    .divider(DOT)
    .highlight_style(Style::new().bold().italic())
    .select(if app.mode == AppMode::Upcoming { 0 } else { 1 });
    f.render_widget(tabs, chunks[0]);

    let footer_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
        .split(chunks[2]);

    let hint_text = Paragraph::new(app.keybind_hints.clone());
    f.render_widget(hint_text, footer_layout[0]);

    let status_text = Paragraph::new(app.status_text.to_string());
    f.render_widget(status_text, footer_layout[1]);

    match &app.mode {
        AppMode::Upcoming => draw_upcoming(f, chunks[1], app),
        AppMode::Categories => draw_categories(f, chunks[1], app),
    }
}

fn draw_upcoming<B: Backend>(f: &mut Frame<B>, r: Rect, app: &mut App) {
    let task_display_height = r.height as usize;
    let task_display_width = r.width as usize;

    let mut dates_seen = -1;
    let list_items: Vec<ListItem> = app
        .task_list
        .get_upcoming_list(
            app.task_list_state.selected().unwrap_or(0),
            task_display_height,
        )
        .windows(2)
        .map(|i| {
            // TODO: improve scrolling behavior
            ListItem::new({
                let mut content = vec![match &i[0] {
                    TaskDate::Date(d) => {
                        dates_seen += 1;
                        Line::from(Span::styled(
                            format!(
                                "{} ({})",
                                d.format("%b %d - %a"),
                                if dates_seen == 0 {
                                    "Today".to_string()
                                } else {
                                    format!("+{}", dates_seen)
                                }
                            ),
                            Style::new().bold(),
                        ))
                    }
                    TaskDate::Task(t) => Line::from(Span::styled(
                        format!(
                            "{:-<width$}{}",
                            t.name.clone(),
                            t.category.name,
                            width = task_display_width - t.category.name.len() - 3
                        ),
                        Style::new().add_modifier(if t.completed {
                            Modifier::CROSSED_OUT
                        } else {
                            Modifier::empty()
                        }),
                    )),
                }];

                if let TaskDate::Date(..) = i[1] {
                    content.push(Line::from(""));
                }

                Text::from(content)
            })
        })
        .collect();

    let list = List::new(list_items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::new().italic())
        .highlight_symbol(">");
    f.render_stateful_widget(list, r, &mut app.task_list_state);

    if app.pop_up.is_some() {
        match app.pop_up.as_ref().unwrap() {
            AppPopUp::TaskEditor => {
                draw_task_editor(f, app);
            }
            AppPopUp::CategoryEditor => {
                draw_category_editor(f, app);
            }
        }
    }
}

fn draw_categories<B: Backend>(f: &mut Frame<B>, r: Rect, app: &mut App) {
    let blocks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(35), Constraint::Percentage(65)])
        .split(r);

    let categories_listitems: Vec<ListItem> = app
        .categories
        .iter()
        .map(|i| ListItem::new(Text::from(i.name.clone())))
        .collect();

    let categories_list = List::new(categories_listitems)
        .block(Block::new().borders(Borders::ALL))
        .highlight_style(Style::new().italic())
        .highlight_symbol(">");
    f.render_stateful_widget(categories_list, blocks[0], &mut app.category_list_state);

    let current_category_tasks: Vec<&Task> = app
        .task_list
        .tasks
        .iter()
        .filter(|t| t.category.id == app.categories[app.category_list_state.selected().unwrap()].id)
        .collect();

    let current_category_listitems: Vec<ListItem> = current_category_tasks
        .iter()
        .map(|t| ListItem::new(Text::from(t.name.clone())))
        .collect();

    let current_category_list = List::new(current_category_listitems)
        .block(Block::new().borders(Borders::ALL))
        .highlight_style(Style::new().italic())
        .highlight_symbol(">");
    f.render_widget(current_category_list, blocks[1]);
}

fn draw_task_editor<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    // NOTE: calculate required lengths BEFORE rendering
    let task_editor_width = 50; // TODO: need to be changed to minimums instead of constants
    let category_editor_height = if app.task_edit_field == SelectedField::Category {
        8
    } else {
        3
    };
    let task_editor_height = 9 + category_editor_height;

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
        .constraints(vec![Constraint::Min(0), Constraint::Length(1)])
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
        .constraints(vec![
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .margin(1)
        .split(hint_layout[0]);

    let textarea = Paragraph::new(app.name_edit.text.clone())
        .block(Block::new().title("Name").borders(Borders::ALL));
    f.render_widget(textarea, vertical_layout[0]);

    let date_blocks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Max(18),
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
            TaskDate::Task(t) => t.due_date.format("%F (%a)").to_string(),
            TaskDate::Date(d) => d.to_string(),
        })
        .block(Block::new().title("Due").borders(Borders::ALL));
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

    let weekday = Paragraph::new(
        if let Some(parsed_date) = NaiveDate::from_ymd_opt(
            app.year_edit.text.parse::<i32>().unwrap_or(i32::MAX),
            app.month_edit.text.parse::<u32>().unwrap_or(0),
            app.date_edit.text.parse::<u32>().unwrap_or(0),
        ) {
            format!("{} ({})", parsed_date.format("%a"), {
                let offset = parsed_date
                    .signed_duration_since(Local::now().date_naive()) // TODO: overflow and logic is super messy
                    .num_days();
                if offset == 0 {
                    "Today".to_string()
                } else {
                    format!("{:+}", offset)
                }
            })
        } else {
            "DATE INVALID".to_string()
        },
    )
    .block(Block::new().title("W").borders(Borders::ALL));
    f.render_widget(weekday, date_layout[3]);

    if app.task_edit_field == SelectedField::Category {
        let category = List::new(
            app.categories
                .iter()
                .map(|c| ListItem::new(Text::from(c.name.clone())))
                .collect::<Vec<ListItem>>(),
        )
        .block(Block::new().title("Category").borders(Borders::ALL))
        .highlight_style(Style::new().italic())
        .highlight_symbol(">");
        f.render_stateful_widget(category, vertical_layout[2], &mut app.category_edit_state);
    } else {
        let current_category = Paragraph::new(
            app.categories[app.category_edit_state.selected().unwrap()]
                .name
                .clone(),
        )
        .block(Block::new().title("Category").borders(Borders::ALL));
        f.render_widget(current_category, vertical_layout[2]);
    }

    let (active_area, active_index) = match app.task_edit_field {
        SelectedField::Name => (vertical_layout[0], app.name_edit.index),
        SelectedField::Year => (date_layout[0], app.year_edit.index),
        SelectedField::Month => (date_layout[1], app.month_edit.index),
        SelectedField::Date => (date_layout[2], app.date_edit.index),
        SelectedField::Category => (Rect::default(), 0),
    };

    if app.task_edit_field != SelectedField::Category {
        f.set_cursor(active_area.x + active_index as u16 + 1, active_area.y + 1);
    }
}

fn draw_category_editor<B: Backend>(f: &mut Frame<B>, app: &mut App) {}
