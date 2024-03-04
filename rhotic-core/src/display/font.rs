use std::collections::HashMap;
use std::fmt::Display;
use std::{fs::File, error::Error};
use std::io::prelude::*;

use anyhow::bail;
use fontdue::layout::GlyphRasterConfig;
use fontdue::{Font, FontSettings, Metrics};
use toml::{Table, Value};

use super::Rgba;
use super::image::MonoImage;
use super::types::{CharsToRgbaError, TomlToRgbaError};

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

/// Faces
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Face {
    pub fore: Rgba,
    pub back: Rgba,
    pub scale: f32,
    pub style: Style,
    pub underline: Underline,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Style {
    None,
    Bold,
    Italic,
    BoldItalic,
    Oblique,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Underline {
    None,
    Normal(Rgba),
    Squiggly(Rgba)
}

impl TryFrom<String> for Style {
    type Error = StringToStyleError;
    fn try_from(mut value: String) -> Result<Self, Self::Error> {

        use Style::*;

        value.to_lowercase();
        Ok(match value.as_str() {
            "none" => Style::None,
            "bold" => Bold,
            "italic" => Italic,
            "bolditalic" | "italicbold" | "bold_italic" | "italic_bold" => BoldItalic,
            "oblique" => Oblique,
            _ => { return Err(StringToStyleError(value)); }
        })
    }
}

pub struct StringToStyleError(String);

impl TryFrom<Table> for Underline {
    type Error = TomlToUnderlineError;
    fn try_from(value: Table) -> Result<Self, Self::Error> {
        let color = if let Some(v) = value.get("color") {
            Rgba::try_from(v.clone())?
        } else {
            return Err(TomlToUnderlineError::ColorFieldMissing);
        };

        if let Some(Value::String(s)) = value.get("type") {
            Ok(match s.as_str() {
                "none" => Underline::None,
                "normal" => Underline::Normal(color),
                "squiggly" => Underline::Squiggly(color),
                _ => {
                    return Err(TomlToUnderlineError::TypeParseError);
                }
            })
        } else {
            Err(TomlToUnderlineError::TypeFieldMissing)
        }
    }
}

pub enum TomlToUnderlineError {
    ColorFieldMissing,
    TypeFieldMissing,
    RgbaParseError(TomlToRgbaError),
    TypeParseError
}

impl From<TomlToRgbaError> for TomlToUnderlineError {
    fn from(value: TomlToRgbaError) -> Self {
        TomlToUnderlineError::RgbaParseError(value)
    }
}

impl Default for Style {
    fn default() -> Self {
        Style::None
    }
}

impl Default for Underline {
    fn default() -> Self {
        Underline::None
    }
}

impl Default for Face {
    fn default() -> Self {
        Self {
            fore: Rgba::WHITE,
            back: Rgba::BLACK,
            scale: 1.0,
            style: Default::default(),
            underline: Default::default()
        }
    }
}

impl TryFrom<Table> for Face {
    type Error = TomlFaceParseError;

    fn try_from(value: Table) -> Result<Self, Self::Error> {

        let fore = if let Some(f) = value.get("fore") {
            Rgba::try_from(f.clone())?
        } else {
            return Err(TomlFaceParseError::FieldMissing(TomlFaceFields::Foreground));
        };

        let back = if let Some(f) = value.get("back") {
            Rgba::try_from(f.clone())?
        } else {
            Rgba::new(0, 0, 0, 0)
        };

        let scale = if let Some(Value::Float(f)) = value.get("scale") {
            *f as f32
        } else {
            1.0
        };

        let style: Style = if let Some(Value::String(s)) = value.get("style") {
            s.clone().try_into()?
        } else {
            Style::None
        };

        let underline: Underline = if let Some(Value::Table(t)) = value.get("underline") {
            Underline::try_from(t.clone())?
        } else {
            Underline::None
        };

        Ok(
            Self {
                fore,
                back,
                scale,
                style,
                underline
            }
        )
    }
}

pub enum TomlFaceParseError {
    RgbaParse(TomlToRgbaError),
    FieldMissing(TomlFaceFields),
    StringParse(String),
    UnderLineParse(TomlToUnderlineError)
}

impl From<TomlToUnderlineError> for TomlFaceParseError {
    fn from(value: TomlToUnderlineError) -> Self {
        TomlFaceParseError::UnderLineParse(value)
    }
}

impl From<TomlToRgbaError> for TomlFaceParseError {
    fn from(value: TomlToRgbaError) -> Self {
        TomlFaceParseError::RgbaParse(value)
    }
}

impl From<StringToStyleError> for TomlFaceParseError {
    fn from(value: StringToStyleError) -> Self {
        TomlFaceParseError::StringParse(value.0)
    }
}

pub enum TomlFaceFields {
    Foreground,
    Background,
    Scale,
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
