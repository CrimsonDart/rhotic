use std::{error::Error, fmt::Display};



#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Page {
    text: Vec<String>
}

impl Page {

    pub fn len(&self) -> usize {
        self.text.len()
    }

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
        self.text.remove(line)
    }

    pub fn get_line(&self, line: usize) -> Option<&str> {
        self.text.get(line).map(|x| x.as_str())
    }

    pub fn pop_line(&mut self, line: usize) -> Option<String> {
        self.text.pop()
    }

    pub fn insert_char(&mut self, line: usize, index: usize, c: char) -> Result<(), InsertCharError> {

        let mut_line = match self.text.get_mut(line) {
            Some(l) => l,
            None => {
                return Err(InsertCharError::LineLookupOutOfBounds { index: line, length: self.text.len() })
            }
        };

        if index == mut_line.chars().count() {
            self.push_char(line, c);
            return Ok(());
        }

        if c == '\n' {
            let byte_index = match mut_line.char_indices().nth(index) {
                Some(b) => b,
                None => return Err(
                    InsertCharError::CharIndexingOutOfBounds { index, length: mut_line.chars().count() }
                )
            }.0;

            let temp = mut_line.clone();

            let (left, right) = temp.split_at(byte_index);
            mut_line.clear();
            mut_line.push_str(left);
            self.insert_line(line + 1, right);

            return Ok(());
        }

        for (byte_index, _) in mut_line.char_indices().nth(index) {
            mut_line.insert(byte_index, c);
        }
        Ok(())
    }

    pub fn push_char(&mut self, line: usize, c: char) {
        if c == '\n' {
            self.insert_line(line + 1, "");
            return;
        }

        match self.text.get_mut(line) {
            Some(l) => l,
            None => return
        }
        .push(c);
    }

    pub fn remove_char(&mut self, line: usize, index: usize) -> Option<char> {

        let l = self.text.get_mut(line)?;
        let byte_index = l.char_indices().nth(index)?.0;

        let rem = l.remove(byte_index);

        Some(rem)
    }

    pub fn get_char(&self, line: usize, index: usize) -> Option<char> {
        let l = self.get_line(line)?;
        l.chars().nth(index)
    }

    pub fn pop_char(&mut self, line: usize) -> Option<char> {
        let mut l = self.text.get_mut(line)?;

        let c = l.pop();
        c
    }

    pub fn push_str(&mut self, mut line: usize, s: &str) {
        let mut splits = s.split('\n');

        match splits.next() {
            Some(s) => {
                let l = match self.text.get_mut(line) {
                    Some(l) => l,
                    None => return
                };

                l.push_str(s);
            },
            None => return
        }

        for s in splits {
            line += 1;
            self.insert_line(line, s);
        }
    }

    pub fn insert_str(&mut self, mut line: usize, index: usize, s: &str) {
        let l = match self.text.get_mut(line) {
            Some(l) => l,
            None => return
        };

        if l.chars().count() == index {
            self.push_str(line, s);
            return;
        }

        let byte_index = match l.char_indices().nth(index){
            Some(b) => b,
            None => return
        }.0;
        let temp = l.clone();
        let (left, right) = temp.split_at(byte_index);

        let mut splits = s.split('\n');

        match splits.next() {
            Some(s) => {
                l.clear();
                l.push_str(left);
                l.push_str(s);
            },
            None => return
        }

        for s in splits {
            line += 1;
            self.insert_line(line, s);
        }

        let l = match self.text.get_mut(line) {
            Some(s) => s,
            None => return
        };
        l.push_str(right);
    }

    pub fn get_str(&self, line: usize, start: usize, end: usize) -> Option<&str> {
        let l = self.get_line(line)?;

        let start = l.char_indices().nth(start)?.0;
        let end = l.char_indices().nth(end)?.0;

        l.get(start..=end)
    }

    pub fn remove_str(&mut self, line: usize, start: usize, end: usize) -> Option<String> {
        let l = self.text.get_mut(line)?;

        let temp = l.clone();
        let chars = temp.chars();
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
            l.push(c);
        }
        Some(out)
    }

    pub fn as_string(&self) -> String {
        let mut s = String::new();

        let mut line_iter = self.text.iter();

        if let Some(line) = line_iter.next() {
            s.push_str(line);
            s.push(' ');
        }

        for line in line_iter {
            s.push('\n');
            s.push_str(line);
            s.push(' ');
        }

        s.replace('\t', "    ")
    }
}

impl From<&str> for Page {
    fn from(value: &str) -> Self {

        let text = value.split('\n')
            .map(|x| {
            String::from(x)
        }).collect();

        Self {
            text
        }
    }
}

impl Default for Page {
    fn default() -> Self {
        Self {
            text: vec![String::new()]
        }
    }
}

#[derive(Debug, Clone)]
pub enum InsertCharError {
    LineLookupOutOfBounds {
        index: usize,
        length: usize
    },
    CharIndexingOutOfBounds {
        index: usize,
        length: usize
    }
}

impl Error for InsertCharError {}

impl Display for InsertCharError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InsertCharError::LineLookupOutOfBounds { index, length } => {
                write!(f, "Line Lookup Error:\nIndex is {index}, while len is {length}")
            },
            InsertCharError::CharIndexingOutOfBounds { index, length } => {
                write!(f, "Char Index Error:\nIndex is {index}, while len is {length}")
            }
        }
    }
}
