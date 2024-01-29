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
                            self.page.remove_char(self.cursor);
                        } else if self.cursor.1 != 0 {
                            let rem = self.page.text.remove(self.cursor.1).1;
                            self.cursor.1 -= 1;
                            let local_past_str = &mut self.page.text[self.cursor.1];
                            self.cursor.0 = local_past_str.0;
                            local_past_str.1.push_str(rem.as_str());
                        }
                    },
                    '\u{1b}' => {
                        self.mode = Mode::Command;
                    },
                    '\r' => {
                        self.page.insert_new_line((0, self.cursor.1));
                        self.cursor.1 += 1;
                        self.cursor.0 = 0;
                    }
                    _  => {
                        self.page.insert_char_at(self.cursor, c);
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
                    if self.cursor.0 != self.page.text[self.cursor.1].0 {
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
                    if self.cursor.0 != self.page.text[self.cursor.1].0 {
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
        let line_len = self.page.text[self.cursor.1].0;
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
