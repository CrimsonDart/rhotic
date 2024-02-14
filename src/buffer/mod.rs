use winit::keyboard::PhysicalKey;

use self::text_buffer::Page;

mod text_buffer;
mod event;

pub struct Buffer {
    pub page: Page,
    pub line: usize,
    pub cindex: usize,
    pub cindex_mem: usize,
    pub mode: Mode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    Insert,
    Command
}

impl Buffer {

    pub fn text_in(&mut self, text: &str) {

        self.validate_cursor();
        if self.mode == Mode::Insert {

            for c in text.chars() {
                match c {
                    '\u{8}' => {
                        if self.cindex != 0 {
                            self.cindex -= 1;
                            self.page.remove_char(self.line, self.cindex);

                        } else if self.line != 0 {
                            let line = self.page.remove_line(self.line);
                            self.line -= 1;
                            self.cindex = self.page.get_line(self.line).unwrap_or("").chars().count();
                            self.page.push_str(self.line, line.as_str());
                        }
                    },
                    '\u{1b}' => {
                        self.mode = Mode::Command;
                    },
                    '\r' | '\n' => {
                        match self.page.insert_char(self.line, self.cindex, '\n') {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{e}");
                                return;
                            }
                        }
                        self.line += 1;
                        self.cindex = 0;
                    },
                    _  => {
                        let res = self.page.insert_char(self.line, self.cindex, c);
                        match res {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{e}");
                            }
                        }
                        self.cindex += 1;
                    }
                }
            }
        }
    }

    // Forces the cursor in bounds of the text.
    pub fn validate_cursor(&mut self) {

        if self.page.len() <= self.line {
            self.line = self.page.len() - 1;
        }

        let c = self.page.get_line(self.line).unwrap().chars().count();



        if c < self.cindex {
            self.cindex = c;
        }
    }




    pub fn press_key(&mut self, key: PhysicalKey) {
        if let PhysicalKey::Code(k) = key {
            use winit::keyboard::KeyCode::*;
            match k {
                ArrowLeft => {
                    self.move_cursor_left();
                },
                ArrowRight => {
                    self.move_cursor_right();
                },
                _ => {}
            }
        }


        if self.mode != Mode::Command { return; }

        if let PhysicalKey::Code(k) = key {
            use winit::keyboard::KeyCode::*;
            match k {
                KeyI => {
                    self.mode = Mode::Insert;
                },
                _ => {
                    println!("\"{k:?}\" is not defined");
                }
            }
        }
    }

    pub fn echo_key(&mut self, key: PhysicalKey) {
        if let PhysicalKey::Code(k) = key {
            use winit::keyboard::KeyCode::*;
            match k {
                ArrowLeft => {
                    self.move_cursor_left();
                },
                ArrowRight => {
                    self.move_cursor_right();
                },
                _ => {}
            }
        }
    }

    pub fn move_cursor_left(&mut self) -> bool {
        if self.cindex != 0 {
            self.cindex -= 1;
            true
        } else {
            false
        }
    }

    pub fn move_cursor_right(&mut self) -> bool {
        if self.cindex != self.page.get_line(self.line).unwrap_or("").chars().count() {
            self.cindex += 1;
            return true;
        }
        false
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            page: Default::default(),
            cindex: 0,
            line: 0,
            cindex_mem: 0,
            mode: Mode::Command,
        }
    }
}
