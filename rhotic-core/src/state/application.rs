

use crate::dired::Dired;
use crate::{buffer::stage::*, display::font::FontManager};


use crate::{display::event_loop::Input};

// A singeton that contains all data of the application.
pub struct State {
    pub input: Input,
    pub is_focused: bool,
    pub font_manager: FontManager,
    pub stage: Dired,
}

impl State {

    pub fn new() -> anyhow::Result<Self> {

        Ok(Self {
            is_focused: false,
            font_manager: FontManager::new()?,
            input: Input::default(),
            stage: Stage::init(["/home/james/.config"].as_slice())?,
        })
    }

    pub fn send_event(&mut self, event: InputEvent) {

        use StateCommand::*;

        match self.stage.send_event(event) {
            StartStage(_) => {},
            None => {}
            Log(s) => {
                println!("{s}");
            }
        }
    }
}
