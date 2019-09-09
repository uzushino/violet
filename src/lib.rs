use chrono::prelude::*;
use std::io::{self, Stdout};
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time;
use termion::raw::{IntoRawMode, RawTerminal};

mod cursor;
mod event;
mod input;
pub mod markdown;
mod script;

use event::Event;
use input::Input;
use markdown::Markdown;

pub type AppData = Arc<Mutex<Markdown<RawTerminal<Stdout>>>>;

pub struct App {
    file: String,
    pub prompt: AppData,
    interval: u64,
}

pub enum Action {
    Timer,
}

#[derive(Default)]
pub struct AppState;

impl App {
    pub fn new(file: String, input: String, interval: u64) -> Self {
        let stdout = io::stdout();
        let stdout = stdout.into_raw_mode().unwrap();
        let prompt = Arc::new(Mutex::new(Markdown::new(stdout, input)));

        App {
            file,
            prompt,
            interval,
        }
    }

    pub fn run(&self) -> Result<(), failure::Error> {
        {
            let mut p = self.prompt.lock().unwrap();
            cursor::hide(&mut p.stdout);
            p.render()?;
            p.flush()?;
        }

        let (tx, rx) = mpsc::channel();
        let th = {
            if self.interval > 0 {
                self.timer_handler(tx.clone());
            }
            self.event_handler(tx.clone(), rx)
        };

        thread::spawn(move || {
            let _ = Input::reader(tx.clone());
        });

        let _ = th.join();

        let _ = self.prompt.lock().and_then(|mut f| {
            cursor::show(&mut f.stdout);
            f.flush().unwrap();

            Ok(())
        });

        Ok(())
    }

    pub fn timer_handler(&self, tx: Sender<Event>) -> JoinHandle<()> {
        let interval = self.interval.clone();

        thread::spawn(move || loop {
            let ms = time::Duration::from_secs(interval);
            std::thread::sleep(ms);

            let _ = tx.send(Event::Reload);
        })
    }

    pub fn event_handler(&self, tx: Sender<Event>, rx: Receiver<Event>) -> JoinHandle<()> {
        let prompt = self.prompt.clone();
        let file = self.file.clone();
        let mut latest = String::default();

        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(Event::Quit) => {
                        // Quit message.
                        break;
                    }
                    Ok(Event::Save) => {
                        let _ = prompt.lock().and_then(|f| {
                            let now = Utc::now().format("%Y-%m-%dT%H:%M:%SZ");
                            let _ = f.save_as(format!("{}_{}.md", now, file).as_str());
                            Ok(())
                        });
                    }
                    Ok(Event::Reload) => {
                        let _ = prompt.lock().and_then(|mut f| {
                            f.render()
                                .and_then(|markdown| {
                                    if latest != markdown {
                                        let _ = tx.send(Event::Save);
                                        latest = markdown;
                                    };
                                    f.flush()
                                })
                                .unwrap();
                            Ok(())
                        });
                    }
                    _ => {}
                };
            }
        })
    }
}
