use crate::{buffer::stage::*, display::font::FontManager, file::toml::Toml};


use crate::{display::event_loop::Input, buffer::textstage::TextEdit};

// A singeton that contains all data of the application.
pub struct State {
    pub input: Input,
    pub is_focused: bool,
    pub font_manager: FontManager,
    pub stage: TextEdit,
}

impl State {

    pub fn new() -> anyhow::Result<Self> {

        Ok(Self {
            is_focused: false,
            font_manager: FontManager::new()?,
            input: Input::default(),
            stage: Default::default(),
        })
    }

    pub fn advance(&mut self) {

        let result = self.stage.poll(&self.input);

        if let Err(e) = result {
            println!("{e}");
        }

        self.input.advance_state();
    }
}
