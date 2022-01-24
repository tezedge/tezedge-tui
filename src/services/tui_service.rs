use crossterm::event::{self, Event, KeyCode};
use tokio::sync::mpsc;

use crate::terminal_ui::TuiEvent;

use super::ServiceWorkerAsyncResponder;

pub trait TuiService {
    fn send_event(&self, event: TuiEvent);
}

pub struct TuiServiceDefault {
    sender: mpsc::Sender<TuiEvent>,
    pub receiver: mpsc::Receiver<TuiEvent>,
}

impl TuiServiceDefault {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(100);

        Self {
            sender: tx,
            receiver: rx,
        }
    }

    pub async fn capture_events(&self) {
        loop {
            match event::read() {
                Ok(Event::Key(key)) => {
                    if let Err(err) = self
                        .sender
                        .send(TuiEvent::Input(key.code, key.modifiers))
                        .await
                    {
                        eprintln!("{}", err);
                        break;
                    }
                    if key.code == KeyCode::Char('q') {
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
                Ok(Event::Resize(_, _)) => {
                    if let Err(err) = self.sender.send(TuiEvent::Resize).await {
                        eprintln!("{}", err);
                        break;
                    }
                }
                Ok(Event::Mouse(_)) => {
                    if let Err(err) = self.sender.send(TuiEvent::Mouse).await {
                        eprintln!("{}", err);
                        break;
                    }
                }
            }
        }
    }
}
