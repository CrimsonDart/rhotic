use std::{collections::HashMap, cell::OnceCell, sync::OnceLock};

use winit::keyboard::KeyCode;

use crate::display::{text_render::{Render, Renderer}, Rgba, font::FontManager};

use super::{text_buffer::Page, stage::Stage};

pub struct TextStage {
    pub page: Page,
    x: usize,
    y: usize,
    pub mode: Mode,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mode {
    Insert,
    Command
}

impl Stage for TextStage {

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

impl Default for TextStage {
    fn default() -> Self {
        Self {
            page: Default::default(),
            x: 0,
            y: 0,
            mode: Mode::Insert
        }
    }
}

impl TextStage {
    // Forces the cursor in bounds of the text.
    fn validate_cursor(&mut self) {

        if self.page.len() <= self.y {
            self.y = self.page.len() - 1;
        }

        let c = self.page.get_line(self.y).unwrap().chars().count();

        if c < self.x {
            self.x = c;
        }
    }

    pub fn move_cursor_left(&mut self) -> bool {
        self.validate_cursor();
        if self.x != 0 {
            self.x -= 1;
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

    pub fn get_real_cursor(&self) -> (usize, usize) {
        ({
            let len = self.page.get_line(self.y).unwrap_or("").chars().count();

            if self.x > len {
                len
            } else {
                self.x
            }

        }, self.y)
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
