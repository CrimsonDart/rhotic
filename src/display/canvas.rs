use softbuffer::Buffer;
use winit::window::Window;

use super::image::{Image, ColorRect};


pub struct Canvas<'a> {
    buffer: Buffer<'a, &'a Window, &'a Window>,
    width: usize,
    height: usize
}

impl<'a> Canvas<'a> {
    pub fn draw_pixel<C: Into<u32>>(&mut self, x: usize, y: usize, color: C) -> bool {
        if x >= self.width && y >= self.height { return false; }

        self.buffer[y * self.width + x] = color.into();

        true
    }

    pub fn draw_image<R: ColorRect<u32>>(&mut self, x: isize, y: isize, image: &R) {
        let bytes = image.get_bytes();

        let mut gx = x;
        let mut gy = y;

        for counter in 0..image.get_bytes().len() {

            if gx < self.width && gy < self.height {
                self.buffer[gy * self.width + gx] = bytes[counter];
            }

            if gx == image.get_width() + x {
                gx = x;
                gy += 1;
            } else {
                gx += 1;
            }
        }
    }
}
