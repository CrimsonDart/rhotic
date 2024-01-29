use std::ops::Range;


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    text: Vec<Line>
}

impl Page {
    pub fn insert_new_line(&mut self, cursor: (usize, usize)) {
        if cursor.1 == self.text.len() {
            self.text.push(Default::default());
        } else {
            self.text.insert(cursor.1, Default::default());
        }
    }

    pub fn remove_line(&mut self, cursor: (usize, usize)) {

    }

    pub fn insert_char_at(&mut self, cursor: (usize, usize), c: char) {

        if self.text[cursor.1].len == cursor.0 {
            self.text[cursor.1].text.push(c);
            return;
        }

        let chars = self.text[cursor.1].1.chars();
        let mut new_str = String::new();

        let mut counter = 0;
        for ic in chars {
            if counter == cursor.0 {
                new_str.push(c);
            }
            counter += 1;
            new_str.push(ic);
        }

        self.text[cursor.1].text = new_str;
        self.text[cursor.1].len += 1;
    }

    pub fn get_char_mut(&mut self, cursor: (usize, usize)) -> Option<&mut str> {
        self.text[cursor.1].1.as_mut_str().get_mut(cursor.0..cursor.0 + 1)
    }

    pub fn get_char(&self, cursor: (usize, usize)) -> char {
        self.text[cursor.1].1.chars().nth(cursor.0).unwrap_or('\u{0}')
    }

    pub fn remove_char(&mut self, cursor: (usize, usize)) -> char {
        self.text[cursor.1].0 -= 1;
        self.text[cursor.1].1.remove(cursor.0)

    }

    pub fn as_string(&self) -> String {
        let mut out = String::new();
        let mut is_first = true;
        for line in self.text.iter() {
            if !is_first {
                out.push('\n');
            } else {
                is_first = false;
            }
            out.push_str(line.as_str());
        }
        out
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            text: Default::default()
        }
    }
}


#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    len: usize,
    text: String
}

impl Line {
    pub fn new<I: Into<String>>(string: I) -> Self {
        Self {
            len: string.into().chars().count(),
            text: String::new()
        }
    }

    pub fn clear(&mut self) {
        self.len = 0;
        self.text = String::new();
    }

    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }

    pub fn push(&mut self, c: char) {
        self.text.push(c);
        self.len += 1;
    }

    pub fn push_str(&mut self, s: &str) {
        for c in s.chars() {
            self.push(c);
        }
    }

    pub fn insert(&mut self, ch: char, index: usize) {
        if index >= self.len {
            self.push(ch);
            return;
        }
        let chars = self.text.chars();
        self.clear();
        for ci in chars {
            if self.len == index {
                self.push(ch);
            }
            self.push(ci);
        }
    }

    pub fn insert_str(&mut self, s: &str, index: usize) {
        if index >= self.len {
            self.push_str(s);
            return;
        }
        let chars = self.text.chars();
        self.clear();
        for ci in chars {
            if self.len == index {
                self.push_str(s);
            }
            self.push(ci);
        }
    }

    pub fn remove(&mut self, index: usize) -> Option<char> {
        let chars = self.text.chars();
        let mut out = None;
        self.clear();
        for c in chars {
            if self.len == index {
                out = Some(c);
                continue;
            }
            self.push(c);
        }
        out
    }

    pub fn get_char(&self, index: usize) -> Option<char> {
        self.text.chars().nth(index)
    }

    pub fn get_slice(&self, range: Range<usize>) -> Option<&str> {
        self.text.get(range)
    }

    pub fn get_slice_mut(&mut self, range: Range<usize>) -> Option<&mut str> {
        self.text.get_mut(range)
    }

    pub fn pop(&mut self) -> Option<char> {
        self.text.pop()
    }
}

impl Default for Line {
    fn default() -> Self {
        Self {
            len: 0,
            text: String::new()
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn get_char_mut() {
        let mut page = Page {
            text: vec![Line::new(String::from("Ayyyy Lmao"))]
        };

        let c = page.get_char_mut((0,0)).unwrap();
        assert_eq!(c, "A");
    }
}
