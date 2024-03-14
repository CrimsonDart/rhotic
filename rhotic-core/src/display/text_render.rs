use softbuffer::{Buffer};
use winit::{raw_window_handle::{HasDisplayHandle, HasWindowHandle}};



use super::{image::ColorRect, Rgba};


pub struct Canvas<'a, D, W> {
    buffer: Buffer<'a, D, W>,
    width: usize,
    height: usize
}

impl<'a, D: HasDisplayHandle, W: HasWindowHandle> Canvas<'a, D, W> {

    pub fn new(buffer: Buffer<'a, D, W>, width: usize, height: usize) -> Self {
        Self {
            buffer,
            width,
            height
        }
    }

    pub fn destroy(self) -> Buffer<'a, D, W> {
        self.buffer
    }


    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

pub fn draw_image<R: ColorRect<Rgba>>(&mut self, x: isize, y: isize, image: &R) {
    let bytes = image.get_bytes();

    let mut gx = x;
    let mut gy = y;

    for counter in 0..image.get_bytes().len() {

        let (nx, ny) = (gx as usize, gy as usize);

        if nx < self.width && ny < self.height && gx >= 0 && gy >= 0 {
            self.buffer[ny * self.width + nx] = bytes[counter].into();
        }

        if gx == image.get_width() as isize + x - 1 {
            gx = x;
            gy += 1;
        } else {
            gx += 1;
        }
    }
}

pub fn draw_monochrome_image<R: ColorRect<u8, u8>, C: Into<u32>>

    (
        &mut self,
        x: isize, y: isize,
        image: &R,
        black: Rgba,
        white: Rgba
    ) {

    let bytes = image.get_bytes();

    let mut gx = x;
    let mut gy = y;

    for counter in 0..image.get_bytes().len() {

        if gx >= 0 && gy >= 0 {
            let (nx, ny) = (gx as usize, gy as usize);

            if nx < self.width() && ny < self.height() {

                let color = match bytes[counter] {
                    0 => { black },
                    255 => {white},
                    b => { black.blend(white, b) }
                };

                self.buffer[ny * self.width + nx] = color.into();
            }
        }

        if gx == image.get_width() as isize + x - 1 {
            gx = x;
            gy += 1;
        } else {
            gx += 1;
        }
    }
}

pub fn draw_rectangle(&mut self, x: isize, y: isize, rect_width: usize, rect_height: usize, color: Rgba) {
    let mut gx = x;
    let mut gy = y;

    for _ in 0..(rect_width * rect_height) {

        if gx >= 0 && gy >= 0  && gx < self.width as isize && gy < self.height as isize {
            self.buffer[gy as usize * self.width + gx as usize] = color.into();
        }

        if gx == rect_width as isize + x - 1 {
            gx = x;
            gy += 1;
        } else {
            gx += 1;
        }
    }
}


}
