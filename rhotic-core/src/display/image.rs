

use std::fs::File;
use zerocopy::AsBytes;


use image::codecs::png::PngDecoder;
use image::{self, ImageDecoder, ImageError};



use super::{Rgba};


fn load_png(path: &str) -> Result<ImageHandle, ImageError> {

    let file = File::open(path)?;
    let png = PngDecoder::new(file)?;
    let mut buf: Vec<u8> = vec!(0; (png.total_bytes()) as usize);
    println!("{:?}", png.color_type());

    let (width, height) = png.dimensions();
    png.read_image(buf.as_bytes_mut())?;

    let mut out_buf: Vec<Rgba> = Vec::new();
    for pixel in 0..(buf.len() / 4) {

        let mut color = Rgba::default();
        for index in 0..=3 {
            color[index] = buf[pixel * 4 + index];
        }
        out_buf.push(color);
    }

    Ok(ImageHandle::Image {
        vector: out_buf,
        width: width as usize,
        height: height as usize
    })
}


pub enum ImageHandle {
    Handle {
        path: &'static str
    },
    Image {
        vector: Vec<Rgba>,
        width: usize,
        height: usize
    }
}

impl ImageHandle {

    pub fn load(self) -> Result<ImageHandle, ImageError> {
        use ImageHandle::*;

        match self {
            Handle {
                path
            } => {
                load_png(path)
            },
            _ => {
                println!("Image already loaded!");
                Ok(self)
            }
        }
    }

    pub fn to_image(self) -> Option<Image> {
        match self {
            ImageHandle::Image { vector, width, height } => {
                Some(Image {
                    bytes: vector.clone(),
                    width,
                    height
                })
            },
            ImageHandle::Handle { path: _ } => None

        }
    }

    pub fn image_ref(&self) -> Option<ImageRef> {
        match self {
            ImageHandle::Image { vector, width, height } => {
                Some(
                    ImageRef { bytes: vector.as_slice(), width: *width, height: *height }
                )
            },
            ImageHandle::Handle { path: _ } => None
        }
    }
}

pub trait ColorRect<C: Into<R>, R = u32> {
    fn get_bytes(&self) -> &[C];
    fn get_width(&self) -> usize;
    fn get_height(&self) -> usize;
}

pub struct Image {
    pub bytes: Vec<Rgba>,
    pub width: usize,
    pub height: usize,
}


impl Image {
    pub fn get_ref(&self) -> ImageRef {
        ImageRef { bytes: self.bytes.as_slice(), width: self.width, height: self.height }
    }
}

impl ColorRect<Rgba> for Image {
    fn get_bytes(&self) -> &[Rgba] {
        self.bytes.as_slice()
    }

    fn get_width(&self) -> usize {
        self.width
    }

    fn get_height(&self) -> usize {
        self.height
    }
}

pub struct MonoImage {
    pub bytes: Vec<u8>,
    pub width: usize,
    pub height: usize
}

impl ColorRect<u8, u8> for MonoImage {
    fn get_bytes(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    fn get_height(&self) -> usize {
        self.height
    }

    fn get_width(&self) -> usize {
        self.width
    }
}

pub struct ImageRef<'a> {
    bytes: &'a [Rgba],
    width: usize,
    height: usize
}
