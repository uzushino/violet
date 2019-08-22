use failure::Error;
use std::sync::mpsc::Sender;
use termion::event::Key;
use termion::input::TermRead;

use crate::event::Event;

pub struct Input {}

impl Input {
    pub fn reader(tx: Sender<Event>) -> Result<(), Error> {
        let stdin = std::io::stdin();

        for c in stdin.keys() {
            let _ = match c? {
                Key::Ctrl('c') | Key::Esc => {
                    tx.send(Event::Quit)?;
                    break;
                }
                Key::Ctrl('b') => tx.send(Event::Back)?,
                Key::Ctrl('f') => tx.send(Event::Forward)?,
                Key::Ctrl('p') => tx.send(Event::Prev)?,
                Key::Ctrl('n') => tx.send(Event::Next)?,
                Key::Char('\n') => tx.send(Event::Enter)?,
                Key::Char('\t') => tx.send(Event::Tab)?,
                Key::Char(c) => tx.send(Event::Key(c))?,
                Key::Backspace => tx.send(Event::Backspace)?,
                Key::Delete => tx.send(Event::Delete)?,
                Key::Left => tx.send(Event::Left)?,
                Key::Right => tx.send(Event::Right)?,
                Key::Down => tx.send(Event::Down)?,
                Key::Up => tx.send(Event::Up)?,
                Key::F(5) => tx.send(Event::Reload)?,
                Key::F(2) => tx.send(Event::Save)?,
                _ => {}
            };
        }

        Ok(())
    }
}
