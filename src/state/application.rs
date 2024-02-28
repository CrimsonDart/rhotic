use std::{collections::HashMap, fs::File};
use crate::{buffer::{stage::*, minibuffer::{CommandBinds, FunctionMap, Command}}, display::font::FontManager, file::toml::Toml};

use std::io::prelude::*;

use fontdue::{Font, layout::GlyphRasterConfig, Metrics};
use toml::{Table, Value, map::Map};
use winit::keyboard::{PhysicalKey, KeyCode};

use crate::{display::{event_loop::{Input, ButtonState}, font::load_ttf, image::{Image, MonoImage}}, buffer::{Buffer, textstage::TextStage}};

// A singeton that contains all data of the application.
pub struct State {
    pub input: Input,
    pub is_focused: bool,
    pub font_manager: FontManager,
    pub stage: TextStage,
    pub binds: CommandBinds<TextStage>
}

impl State {

    pub fn new() -> anyhow::Result<Self> {

        let table = Toml::open("./config/text.toml")?;

        Ok(Self {
            is_focused: false,
            font_manager: FontManager::new()?,
            input: Input::default(),
            stage: Default::default(),
            binds: {
                FunctionMap::<TextStage>::new().bind(
                    table
                )
            }
        })
    }

    pub fn advance(&mut self) {

        // println!("{:?}", self.input);

        if !self.input.text.is_empty() && !self.input.is_any_key_pressed(
            &[
                PhysicalKey::Code(KeyCode::AltLeft),
                PhysicalKey::Code(KeyCode::AltRight),
                PhysicalKey::Code(KeyCode::ControlLeft),
                PhysicalKey::Code(KeyCode::ControlRight)
            ]
        ) {
            self.stage.input_text(self.input.text.as_str());
        }


        for (k, v) in self.input.keys.iter() {
            if *v == ButtonState::Pressed {
                if let PhysicalKey::Code(key) = k {
                    let out = self.binds.command_call(&mut self.stage, &Command::new(*key, false, false, false));
                    if let Err(e) = out {
                        println!("{key:?}: {e:?}");

                    }
                }
            }
        }

        self.input.advance_state();
    }
}
