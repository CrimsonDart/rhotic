use softbuffer::Buffer;
use winit::window::Window;

use super::image::Image;


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

    pub fn draw_image<C: Into<u32>>(&mut self, x: usize, y: usize, image: &Image) -> bool {

        true
    }

}
