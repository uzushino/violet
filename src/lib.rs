
use std::fs::File;
use std::io::{self, BufRead, BufReader, Stdout};
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use termion::raw::{IntoRawMode, RawTerminal};

mod cursor;
mod input;
mod event;
mod script;
mod markdown;

use event::Event;
use input::Input;
use markdown::Markdown;

pub struct App {
    source: Arc<Mutex<Option<BufReader<File>>>>,
    prompt: Arc<Mutex<Markdown<RawTerminal<Stdout>>>>,
}

impl App {
    pub fn new(input: String) -> Self {
        let stdout = io::stdout();
        let source = source();
        let stdout = stdout.into_raw_mode().unwrap();
        let prompt = Arc::new(Mutex::new(Markdown::new(stdout, input)));

        App {
            prompt,
            source: Arc::new(Mutex::new(source)),
        }
    }

    pub fn run(&self) -> Result<(), failure::Error> {
        {
            let mut p = self.prompt.lock().unwrap();

            p.render()?;
            p.flush()?;

            cursor::hide(&mut p.stdout);
        }

        let (tx, rx) = mpsc::channel();
        let th = {
            self.input_handler(tx.clone());
            self.event_handler(tx.clone(), rx)
        };

        thread::spawn(move || {
            let _ = Input::reader(tx.clone());
        });

        let _ = th.join();

        let mut f = self.prompt.lock().unwrap();
        f.flush()?;
        
        cursor::show(&mut f.stdout);

        Ok(())
    }

    pub fn input_handler(&self, tx: Sender<Event>) -> JoinHandle<()> {
        let source = self.source.clone();

        thread::spawn(move || loop {
            let mut src = source.lock().unwrap();

            if let Some(ref mut b) = *src {
                let mut buf = vec![];

                match b.read_until(b'\n', &mut buf) {
                    Ok(n) if n != 0 => {
                        if buf.ends_with(&[b'\n']) || buf.ends_with(&[b'\0']) {
                            buf.pop();
                        }
                        let l = String::from_utf8(buf).unwrap_or(String::new());
                        let _ = tx.send(Event::ReadLine(l));
                    }
                    _ => {}
                }
            }
        })
    }

    pub fn event_handler(&self, _tx: Sender<Event>, rx: Receiver<Event>) -> JoinHandle<()> {
        let prompt = self.prompt.clone();

        thread::spawn(move || {
            loop {
                match rx.recv() {
                    Ok(Event::Quit) => {
                        // Quit message.
                        break;
                    }
                    _ => {},
                };

                let _ = prompt.lock().and_then(|mut f| {
                    f.render().and_then(|_| f.flush()).unwrap();
                    Ok(())
                });
            }
        })
    }
}

fn source() -> Option<BufReader<File>> {
    unsafe {
        let isatty = libc::isatty(libc::STDIN_FILENO as i32) != 0;
        if !isatty {
            let stdin = File::from_raw_fd(libc::dup(libc::STDIN_FILENO));
            let file = File::open("/dev/tty").unwrap();
            libc::dup2(file.into_raw_fd(), libc::STDIN_FILENO);

            return Some(BufReader::new(stdin));
        }
    }

    None
}
