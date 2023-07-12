use std::{error::Error, fs::File, io, os::unix::prelude::FileExt};
use std::io::prelude::*;

use fontdue::{Font, FontSettings};
use owned_ttf_parser::{OwnedFace, Face};



pub fn load_ttf(path: &str) -> Result<Font, &'static str> {

    let file = File::open(path);
    let mut file = match file {
        Ok(file) => file,
        Err(_) => return Err("File Opening Error")
    };

    let mut buf = Vec::new();
    let result = file.read_to_end(&mut buf);

    if let Err(_) = result {
        return Err("File Reading Error");
    }

    let font_settings = FontSettings::default();

    let face = Font::from_bytes(buf, font_settings);
    match face {
        Ok(face) => Ok(face),
        Err(_) => Err("Face parsing error")
    }
}
