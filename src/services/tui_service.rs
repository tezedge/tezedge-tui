use std::io;
use std::{io::Stdout, time::Duration};

use crossterm::event::{self, EnableMouseCapture, Event, KeyCode};
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use tokio::sync::mpsc;
use tui::{backend::CrosstermBackend, Terminal};

use crate::terminal_ui::TuiEvent;

pub struct TuiServiceDefault {
    pub terminal: Terminal<CrosstermBackend<Stdout>>,
}

pub trait TuiService {
    fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>>;
}

impl TuiService for TuiServiceDefault {
    fn terminal(&mut self) -> &mut Terminal<CrosstermBackend<Stdout>> {
        &mut self.terminal
    }
}

impl TuiServiceDefault {
    pub fn new() -> Self {
        enable_raw_mode().expect("Failed to enable raw mode");
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
            .expect("Failed to execute EnterAlternateScreen + EnableMouseCapture");
        let backend = CrosstermBackend::new(stdout);

        let terminal = Terminal::new(backend).expect("Error initializing terminal");

        TuiServiceDefault { terminal }
    }

    pub fn start(tick_rate: Duration) -> mpsc::Receiver<TuiEvent> {
        let (tx, rx) = mpsc::channel(100);

        let key_tx = tx.clone();
        tokio::task::spawn(async move {
            Self::capture_events(key_tx).await;
        });

        tokio::task::spawn(async move {
            Self::generate_tick(tx, tick_rate).await;
        });

        rx
    }

    pub async fn capture_events(sender: mpsc::Sender<TuiEvent>) {
        loop {
            match event::read() {
                Ok(Event::Key(key)) => {
                    if let Err(err) = sender.send(TuiEvent::Input(key.code, key.modifiers)).await {
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
                    if let Err(err) = sender.send(TuiEvent::Resize).await {
                        eprintln!("{}", err);
                        break;
                    }
                }
                Ok(Event::Mouse(_)) => {
                    if let Err(err) = sender.send(TuiEvent::Mouse).await {
                        eprintln!("{}", err);
                        break;
                    }
                }
            }
        }
    }

    pub async fn generate_tick(sender: mpsc::Sender<TuiEvent>, tick_rate: Duration) {
        loop {
            if let Err(err) = sender.send(TuiEvent::Tick).await {
                eprintln!("{}", err);
                break;
            }
            tokio::time::sleep(tick_rate).await;
        }
    }
}

impl Default for TuiServiceDefault {
    fn default() -> Self {
        Self::new()
    }
}
