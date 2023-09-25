mod app;
mod category;
mod database;
mod event;
mod key;
mod task;
mod ui;

use app::{App, AppReturn};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use database::{IOEvent, IOHandler};
use event::{AppEvent, AppEventHandler};
use ratatui::{prelude::CrosstermBackend, Terminal};
use std::{error, io::stdout, sync::Arc, time::Duration};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    // Create database connection
    let url = "postgres://postgres:password@localhost:5432/task-db";
    let pool = sqlx::PgPool::connect(url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    // Set up channel to database handler
    let (io_tx, mut io_rx) = mpsc::channel::<IOEvent>(100);

    // Create app and wrap in Mutex/Arc to allow IO/UI to both mutate data
    let app = Arc::new(tokio::sync::Mutex::new(app::App::new(io_tx.clone())));
    let app_ui = Arc::clone(&app);

    // Spawn database handler task
    tokio::spawn(async move {
        let mut handler = IOHandler::new(app, pool);
        while let Some(io_event) = io_rx.recv().await {
            if handler.handle_io(io_event).await.is_err() {
                panic!("database io request failed");
            };
        }
    });

    start_ui(app_ui).await?;
    Ok(())
}

async fn start_ui(app: Arc<tokio::sync::Mutex<App>>) -> Result<(), Box<dyn error::Error>> {
    // Set up terminal window
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stdout(), EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // Initialize app
    app.lock().await.initialize().await;

    // Event loop (waiting for keypresses/automatically ticking)
    let mut app_event_handler = AppEventHandler::new(Duration::from_millis(200));
    loop {
        let mut app = app.lock().await;
        terminal.draw(|f| ui::draw(f, &mut app))?;

        let result = match app_event_handler.next().await {
            AppEvent::Input(key) => app.do_action(key).await,
            AppEvent::Tick => app.update_on_tick().await,
        };

        if result == AppReturn::Quit {
            app_event_handler.close();
            break;
        }
    }

    // Exit terminal screen
    crossterm::execute!(stdout(), LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
