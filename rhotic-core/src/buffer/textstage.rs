use std::{collections::HashMap, cell::OnceCell, sync::OnceLock};

use winit::keyboard::KeyCode;

use crate::display::{Rgba, font::FontManager};

use super::{text_buffer::Page, stage::{Stage, TextStage}};

use rhotic_macro::text_and_render;

#[text_and_render]
pub struct TextEdit {
    pub mode: Mode,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mode {
    Insert,
    Command
}

impl Stage for TextEdit {

    const NAME: &'static str = "Text Stage";


    fn poll(&mut self, input: &crate::display::event_loop::Input) -> anyhow::Result<()> {

        if !input.text.is_empty() && !input.is_any_key_pressed(
            &[
                KeyCode::AltLeft,
                KeyCode::AltRight,
                KeyCode::ControlLeft,
                KeyCode::ControlRight
            ]
        ) {
            self.input_text(input.text.as_str());
        }


        for (k, v) in input.keys.iter() {

        }

        Ok(())
    }
}

impl Default for TextEdit {
    fn default() -> Self {
        Self {
            page: Default::default(),
            cursor_x: 0,
            cursor_y: 0,
            mode: Mode::Insert
        }
    }
}

impl TextStage for TextEdit {
    fn get_display_text(&self) -> String {
        self.page.as_string()
    }

    fn get_cursor(&self) -> (usize, usize, super::stage::CursorLook) {

        use Mode::*;
        (
            {
                let len = self.page.get_line(self.y).unwrap_or("").chars().count();

                if self.x > len {
                    len
                } else {
                    self.x
                }
            },
            self.y,
            match self.mode {
                Insert => crate::buffer::stage::CursorLook::VerticalBar,
                Command => crate::buffer::stage::CursorLook::Block
            }
        )
    }
}

impl TextEdit {
    // Forces the cursor in bounds of the text.
    fn validate_cursor(&mut self) {

        if self.page.len() <= self.cursor_y {
            self.cursor_y = self.page.len() - 1;
        }

        let c = self.page.get_line(self.cursor_y).unwrap().chars().count();

        if c < self.cursor_x {
            self.cursor_x = c;
        }
    }

    pub fn move_cursor_left(&mut self) -> bool {
        self.validate_cursor();
        if self.cursor_x != 0 {
            self.cursor_x -= 1;
            return true;
        }
        false
    }

    pub fn insert_mode(&mut self) -> bool {
        if self.mode == Mode::Command {
            self.mode = Mode::Insert;
        }
        true
    }

    pub fn command_mode(&mut self) -> bool {
        self.mode = Mode::Command;
        true
    }

    pub fn backspace(&mut self) -> bool {

        if self.mode == Mode::Command {
            return self.move_cursor_left();
        }

        if self.x != 0 {
            self.x -= 1;
            self.page.remove_char(self.y, self.x);

        } else if self.y != 0 {
            let line = self.page.remove_line(self.y);
            self.y -= 1;
            self.x = self.page.get_line(self.y).unwrap_or("").chars().count();
            self.page.push_str(self.y, line.as_str());
        }
        true
    }

    fn input_text(&mut self, text: &str) {
        self.validate_cursor();
        if self.mode == Mode::Insert {

            for c in text.chars() {
                match c {
                    '\u{8}' | '\u{1b}' => {},
                    '\r' | '\n' => {
                        match self.page.insert_char(self.y, self.x, '\n') {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{e}");
                                return;
                            }
                        }
                        self.y += 1;
                        self.x = 0;
                    },
                    _  => {
                        let res = self.page.insert_char(self.y, self.x, c);
                        match res {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{e}");
                            }
                        }
                        self.x += 1;
                    }
                }
            }
        }
    }
}
