use std::{collections::HashMap, cell::OnceCell, sync::OnceLock};

use super::{text_buffer::Page, stage::{Stage, Function}};



static FUNCTIONS: OnceLock<HashMap<&'static str, fn(&mut TextStage) -> bool>> = OnceLock::new();





pub struct TextStage {
    page: Page,
    x: usize,
    y: usize,
    mode: Mode
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Mode {
    Insert,
    Command
}

impl Stage for TextStage {
    fn get_functions() -> &'static [(&'static str, fn(&mut Self) -> bool)] {






        [].as_slice()
    }

    fn input_text(&mut self, text: &str) {
        self.validate_cursor();
        if self.mode == Mode::Insert {

            for c in text.chars() {
                match c {
                    '\u{8}' => {
                        if self.x != 0 {
                            self.x -= 1;
                            self.page.remove_char(self.y, self.x);

                        } else if self.y != 0 {
                            let line = self.page.remove_line(self.y);
                            self.y -= 1;
                            self.x = self.page.get_line(self.y).unwrap_or("").chars().count();
                            self.page.push_str(self.y, line.as_str());
                        }
                    },
                    '\u{1b}' => {
                        self.mode = Mode::Command;
                    },
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


}
