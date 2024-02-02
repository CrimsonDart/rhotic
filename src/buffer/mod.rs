use winit::keyboard::PhysicalKey;

use self::text_buffer::Page;

mod text_buffer;

pub struct Buffer {
    pub page: Page,
    pub cursor: (usize, usize),
    pub mode: Mode,
}



#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Mode {
    Insert,
    Command
}

impl Buffer {

    pub fn text_in(&mut self, text: &str) {

        if self.mode == Mode::Insert {

            for c in text.chars() {
                println!("{c:?}");
                match c {
                    '\u{8}' => {
                        if self.cursor.0 != 0 {
                            self.cursor.0 -= 1;
                            self.page.remove_char(self.cursor.1, self.cursor.0);
                        } else if self.cursor.1 != 0 {
                            let line = self.page.remove_line(self.cursor.1);
                            self.cursor.0 = self.page.get_line(self.cursor.1).unwrap_or("").chars().count() + 1;
                            self.cursor.1 -= 1;
                            self.page.push_str(self.cursor.1 - 1, line.as_str());
                        }
                    },
                    '\u{1b}' => {
                        self.mode = Mode::Command;
                    },
                    '\r' | '\n' => {
                        self.page.insert_line(self.cursor.1 + 1, "");
                        self.cursor.0 = 0;
                        self.cursor.1 += 1;

                    },
                    _  => {
                        let res = self.page.insert_char(self.cursor.1, self.cursor.0, c);
                        match res {
                            Ok(_) => {},
                            Err(e) => {
                                println!("{e}");
                            }
                        }
                        self.cursor.0 += 1;
                    }
                }
            }
        }
    }

    pub fn press_key(&mut self, key: PhysicalKey) {
        if let PhysicalKey::Code(k) = key {
            use winit::keyboard::KeyCode::*;
            match k {
                ArrowLeft => {
                    if self.cursor.0 != 0 {
                        self.cursor.0 -= 1;
                    }
                },
                ArrowRight => {
                    if self.cursor.0 != self.page.get_line(self.cursor.1).unwrap_or("").chars().count() + 1 {
                        self.cursor.0 += 1;
                    }
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
                    if self.cursor.0 != 0 {
                        self.cursor.0 -= 1;
                    }
                },
                ArrowRight => {
                    if self.cursor.0 != self.page.get_line(self.cursor.1).unwrap_or("").chars().count() + 1 {
                        self.cursor.0 += 1;
                    }
                },
                _ => {}
            }
        }
    }

    pub fn move_cursor_left(&mut self) -> bool {
        if self.cursor.0 != 0 {
            self.cursor.0 -= 1;
            return true;
        }
        false
    }

    pub fn move_cursor_right(&mut self) -> bool {
        let line_len = self.page.get_line(self.cursor.1).unwrap_or("").chars().count();
        if self.cursor.0 + 1 != line_len {
            self.cursor.0 += 1;
            return true;
        }
        false
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self {
            page: Default::default(),
            cursor: (0,0),
            mode: Mode::Command,
        }
    }
}
