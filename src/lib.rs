use chrono::prelude::*;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};
use std::time;
mod builtin;
mod cursor;
mod event;
pub mod input;
pub mod markdown;
mod script;

use markdown::Markdown;

pub type AppData = Markdown;

pub struct App {
    pub prompt: AppData,
}

pub enum Action {
    Timer,
}

#[derive(Default)]
pub struct AppState;

impl App {
    pub fn new(file: String, input: String) -> Self {
        let prompt = Markdown::new(input);

        App {
            prompt,
        }
    }

    pub fn run(&self) -> Result<String, failure::Error> {
        self.prompt.evaluate()
    }
}
