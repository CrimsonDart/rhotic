use std::collections::HashMap;

use fontdue::{Font, layout::GlyphRasterConfig, Metrics};
use winit::keyboard::{PhysicalKey, KeyCode};

use crate::{display::{event_loop::{Input, ButtonState}, font::load_ttf, image::{Image, MonoImage}}, buffer::Buffer};

// A singeton that contains all data of the application.
pub struct State {
    pub input: Input,
    pub is_focused: bool,
    pub is_colored: bool,
    pub font: Font,
    pub char_cache: HashMap<GlyphRasterConfig, MonoImage>,
    pub glyph_scale: f32,
    pub buffer: Buffer
}

impl State {

    pub fn new() -> anyhow::Result<Self> {
        let font = load_ttf("./assets/fonts/FiraCode-Regular.ttf")?;
        Ok(Self {
            is_focused: false,
            is_colored: false,
            font,
            input: Input::default(),
            char_cache: HashMap::new(),
            glyph_scale: 20.0,
            buffer: Default::default(),
        })
    }

    pub fn advance(&mut self) {

        // println!("{:?}", self.input);

        if !self.input.text.is_empty() {
            self.buffer.text_in(self.input.text.as_str());
        }

        for (k, v) in self.input.keys.iter() {
            if *v == ButtonState::Pressed {
                self.buffer.press_key(k.clone());
            } else if let ButtonState::Echo(t) = *v {
                self.buffer.echo_key(k.clone());
            }
        }

        self.input.advance_state();
    }
}
