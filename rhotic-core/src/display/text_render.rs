use softbuffer::{Rect, Buffer};
use winit::{window::Window, raw_window_handle::{HasDisplayHandle, HasWindowHandle}};

use crate::buffer::stage::Stage;


pub struct Canvas<'a, D, W> {
    pub buffer: Buffer<'a, D, W>,
    width: usize,
    height: usize
}

impl<'a, D, W> Canvas<'a, D, W> {
    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

pub struct Renderer;

pub trait Render<S: Stage> {
    fn render<'a, D: HasDisplayHandle, W: HasWindowHandle>(stage: &S, canvas: Canvas<'a, D, W>);
}
