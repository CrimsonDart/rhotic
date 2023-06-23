use std::{error::Error, fs::File, io, os::unix::prelude::FileExt};
use std::io::prelude::*;

use rusttype::Font;


pub fn load_ttf(path: &str) -> io::Result<Font> {




    let mut file = File::open(path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf);


    let font = Font::try_from_vec(buf).unwrap();

    Ok(font)
}
