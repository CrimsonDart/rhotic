use std::fmt::Display;
use std::{fs::File, error::Error};
use std::io::prelude::*;

use fontdue::{Font, FontSettings};

pub fn load_ttf(path: &str) -> anyhow::Result<Font> {

    let mut file = File::open(path)?;

    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let font_settings = FontSettings::default();

    let face = Font::from_bytes(buf, font_settings);
    match face {
        Ok(ok) => Ok(ok),
        Err(s) => {
            Err(FontOpenError { error: s })?
        }
    }
}

#[derive(Debug)]
pub struct FontOpenError {
    error: &'static str
}

impl Error for FontOpenError {}

impl Display for FontOpenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}
