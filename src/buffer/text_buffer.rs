

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    text: Vec<String>
}

impl Page {

    pub fn insert_line(&mut self, mut line: usize, text: &str) {
        if line == self.text.len() {
            self.push_line(text);
            return;
        }

        text.split('\n').for_each(|x| {
            self.text.insert(line, x.into());
            line += 1;
        });
    }

    pub fn push_line(&mut self, text: &str) {
        text.split('\n').for_each(|x| {
            self.text.push(x.into())
        });
    }

    pub fn remove_line(&mut self, line: usize) -> String {
        self.remove_line(line)
    }

    pub fn get_line(&self, line: usize) -> Option<&str> {
        self.text.get(line).map(|x| x.as_str())
    }

    pub fn pop_line(&mut self, line: usize) -> Option<String> {
        self.text.pop()
    }

    pub fn insert_char(&mut self, line: usize, index: usize, c: char) {

        let mut mut_line = match self.text.get_mut(line) {
            Some(l) => l,
            None => return
        };

        if index == mut_line.chars().count() {
            self.push_char(line, c);
            return;
        }

        if c == '\n' {





            let chars = mut_line.chars();
            mut_line.clear();

            for character in chars {
                if index == mut_line.len {
                    break;
                }
                mut_line.text.push(character);
                mut_line.len += 1;
            }

            let mut s = Line::default();
            for character in chars {
                s.text.push(character);
                s.len += 1;
            }
            self.text.insert(line + 1, s);
            return;
        }

        for (byte_index, _) in mut_line.char_indices().nth(index) {
            mut_line.insert(byte_index, c);
        }
    }

    pub fn push_char(&mut self, line: usize, c: char) {
        if c == '\n' {
            self.insert_line(line + 1, "");
            return;
        }

        let mut l = match self.get_line_mut(line) {
            Some(l) => l,
            None => return
        };
        l.text.push(c);
        l.len += 1;
    }

    pub fn remove_char(&mut self, line: usize, index: usize) -> Option<char> {

        let mut l = self.get_line_mut(line)?;
        let byte_index = l.text.char_indices().nth(index)?.0;

        let rem = l.text.remove(byte_index);
        l.len -= 1;

        Some(rem)
    }

    pub fn get_char(&self, line: usize, index: usize) -> Option<char> {
        let l = self.get_line(line)?;
        l.text.chars().nth(index)
    }

    pub fn pop_char(&mut self, line: usize) -> Option<char> {
        let mut l = self.get_line_mut(line)?;

        let c = l.text.pop();
        l.len -= 1;
        c
    }

    pub fn push_str(&mut self, mut line: usize, s: &str) {
        let mut splits = s.split('\n');

        match splits.next() {
            Some(s) => {
                let l = match self.get_line_mut(line) {
                    Some(l) => l,
                    None => return
                };

                l.len += s.chars().count();
                l.text.push_str(s);
            },
            None => return
        }

        for s in splits {
            line += 1;
            self.insert_line(line, s);
        }
    }

    pub fn insert_str(&mut self, mut line: usize, index: usize, s: &str) {
        let l = match self.get_line_mut(line) {
            Some(l) => l,
            None => return
        };

        if l.len == index {
            self.push_str(line, s);
            return;
        }

        let byte_index = match l.text.char_indices().nth(index){
            Some(b) => b,
            None => return
        }.0;
        let (left, right) = l.text.split_at(byte_index);

        let mut splits = s.split('\n');

        match splits.next() {
            Some(s) => {
                l.clear();
                l.text.push_str(left);
                l.text.push_str(s);
                l.len = left.chars().count() + s.chars().count();
            },
            None => return
        }

        for s in splits {
            line += 1;
            self.insert_line(line, s);
        }

        let l = match self.get_line_mut(line) {
            Some(s) => s,
            None => return
        };
        l.text.push_str(right);
        l.len += right.chars().count();
    }

    pub fn get_str(&self, line: usize, start: usize, end: usize) -> Option<&str> {
        let l = self.get_line(line)?;

        let start = l.text.char_indices().nth(start)?.0;
        let end = l.text.char_indices().nth(end)?.0;

        l.text.get(start..=end)
    }

    pub fn remove_str(&self, line: usize, start: usize, end: usize) -> Option<String> {
        let mut l = self.get_line_mut(line)?;

        let chars = l.text.chars();
        l.clear();
        let mut counter = 0;
        let range = start..=end;
        let mut out = String::new();

        for c in chars {
            if range.contains(&counter) {
                counter += 1;
                out.push(c);
                continue;
            }
            counter += 1;
            l.text.push(c);
            l.len += 1;
        }
        Some(out)
    }

    pub fn as_string(&self) -> String {
        let mut s = String::new();
        use std::fmt::Write;

        let mut is_first = true;
        for line in self.text {
            if !is_first {
                write!(s, "\n{}", line.text);
                is_first = false;
            } else {
                write!(s, "{}", line.text);
            }
        }
    }

    pub fn new() -> Self {
        Self {
            text: Vec::new()
        }
    }
}

impl From<&str> for Page {
    fn from(value: &str) -> Self {

        let splits = value.split('\n');

    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            text: Vec::new()
        }
    }
}

