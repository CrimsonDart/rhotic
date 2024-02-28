use std::collections::HashMap;
use std::fmt::Display;
use std::{fs::File, error::Error};
use std::io::prelude::*;

use fontdue::layout::GlyphRasterConfig;
use fontdue::{Font, FontSettings, Metrics};

use super::Rgba;
use super::image::MonoImage;

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

pub struct FontManager {
    pub fonts: Vec<Font>,
    pub cache: HashMap<GlyphRasterConfig, (Metrics, MonoImage)>,
    pub scale: f32,
    pub fore: Rgba,
    pub back: Rgba,
}

impl FontManager {
    pub fn new() -> anyhow::Result<Self> {
        let fonts = vec![
            load_ttf("./assets/fonts/FiraCode-Regular.ttf")?
        ];

        Ok(Self {
            fonts,
            cache: HashMap::new(),
            scale: 20.0,
            fore: Rgba::WHITE,
            back: Rgba::DARK_GRAY
        })
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
