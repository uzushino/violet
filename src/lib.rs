mod builtin;
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
    pub fn new(input: String) -> Self {
        let prompt = Markdown::new(input);

        App {
            prompt,
        }
    }

    pub fn run(&self) -> anyhow::Result<String> {
        self.prompt.evaluate()
    }
}
