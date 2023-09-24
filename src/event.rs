use crate::key::Key;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::mpsc::{self};

pub enum AppEvent {
    Input(Key),
    Tick,
}
pub struct AppEventHandler {
    rx: mpsc::Receiver<AppEvent>,
    _tx: mpsc::Sender<AppEvent>,
    stopped: Arc<AtomicBool>,
}

impl AppEventHandler {
    pub fn new(tick_rate: Duration) -> AppEventHandler {
        let (tx, rx) = mpsc::channel(100);
        let stopped = Arc::new(AtomicBool::new(false));

        let event_tx = tx.clone();
        let event_stopped = stopped.clone();

        // Spawns thread to handle keypress events
        tokio::spawn(async move {
            loop {
                let event = if crossterm::event::poll(tick_rate).unwrap() {
                    if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                        AppEvent::Input(Key::from(key))
                    } else {
                        AppEvent::Tick
                    }
                } else {
                    AppEvent::Tick
                };

                if event_stopped.load(Ordering::Relaxed) {
                    break;
                } else if event_tx.send(event).await.is_err() {
                    panic!("event handler not receiving tick")
                }
            }
        });

        AppEventHandler {
            rx,
            _tx: tx,
            stopped,
        }
    }

    pub async fn next(&mut self) -> AppEvent {
        self.rx.recv().await.unwrap_or(AppEvent::Tick)
    }

    pub fn close(&mut self) {
        self.stopped.store(true, Ordering::Relaxed);
    }
}
