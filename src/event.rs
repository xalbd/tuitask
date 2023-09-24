use crate::key::Key;
use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::mpsc;

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
                // Handle key event during a tick
                if crossterm::event::poll(tick_rate).unwrap() {
                    if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                        if event_tx
                            .send(AppEvent::Input(Key::from(key)))
                            .await
                            .is_err()
                        {
                            panic!("event handler not receiving messages")
                        };
                    }
                }
                // Send tick event if no keystrokes were recorded in the tick
                else if event_tx.send(AppEvent::Tick).await.is_err() {
                    panic!("event handler not receiving tick")
                }

                if event_stopped.load(Ordering::Relaxed) {
                    break;
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
