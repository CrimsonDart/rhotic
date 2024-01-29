

use std::fs::File;
use zerocopy::AsBytes;


use image::codecs::png::PngDecoder;
use image::{self, ImageDecoder, ImageError};


use super::{Rgba, Point};


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
            match index {
                0 => {
                    color.set_red(buf[pixel * 4] as u32);
                },
                1 => {
                    color.set_green(buf[(pixel * 4) + 1] as u32);
                },
                2 => {
                    color.set_blue(buf[(pixel * 4) + 2] as u32);
                },
                3 => {
                    color.set_alpha(buf[(pixel * 4) + 3] as u32);
                },
                _ => {}
            }
        }
        out_buf.push(color);
    }

    Ok(ImageHandle::Image {
        vector: out_buf,
        size: Point::new(width, height)
    })
}


pub enum ImageHandle {
    Handle {
        path: &'static str
    },
    Image {
        vector: Vec<Rgba>,
        size: Point<u32>
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


    pub fn to_rect(self, offset: Point<u32>) -> Result<DisplayImage, &'static str> {
        use ImageHandle::*;

        match self {
            Handle => {
                Err("Tried to convert image without Loading first!")
            },
            Image {
                vector,
                size
            }
                => {
                Ok(DisplayImage {
                    vector,
                    size,
                    offset
                })
            }
        }
    }
}

pub struct DisplayImage {
    pub vector: Vec<Rgba>,
    pub offset: Point<u32>,
    pub size: Point<u32>
}
