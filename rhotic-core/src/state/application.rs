use std::path::PathBuf;

use crate::dired::Dired;
use crate::{buffer::stage::*, display::font::FontManager, file::toml::Toml};


use crate::{display::event_loop::Input, buffer::textstage::TextEdit};

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
            stage: Stage::init(["/home/james/Library"].as_slice())?,
        })
    }

    pub fn send_event(&mut self, event: InputEvent) {

        use StateCommand::*;

        match self.stage.send_event(event) {
            StartStage(c) => {},
            None => {}
        }
    }
}
