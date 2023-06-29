use image::buffer;
use softbuffer::Buffer;
use winit::window;

use crate::display::{Rect, Point, types::RectIter};

use self::background::Background;

use super::application::State;

pub mod button;
pub mod background;

pub trait Widget {
    fn get_rect(&self) -> Rect;
    fn draw(&self, buffer: DrawBuffer, state: &State);
}

pub fn draw_to_buffer<'a>(widget: &Box<dyn Widget>, buffer: &mut Buffer<'a>, window_size: Point<u32>, state: &State) {

    let buffref = buffer as *mut Buffer<'a>;


    let draw_buffer = DrawBuffer {
        buffer: buffref,
        rect: widget.get_rect(),
        window_size
    };
    widget.draw(draw_buffer, state);
}

pub struct DrawBuffer<'a> {
    pub buffer: *mut Buffer<'a>,
    pub rect: Rect,
    pub window_size: Point<u32>
}

impl<'a> DrawBuffer<'a> {
    pub fn draw_to(&mut self, x: u32, y: u32, color: u32) {

        let size = &self.rect.size;
        let offset = &self.rect.offset;
        let window_size = &self.window_size;

        if x > size.x {return;}
        if y > size.y {return;}
        if x > window_size.x {return;}
        if y > window_size.y {return;}

        let index = (offset.y + y) * size.x + (offset.x + x);

        unsafe {
            let length = <*const Buffer<'a>>::as_ref(self.buffer).unwrap().len();
            if (index as usize) < length {
                <*mut Buffer<'a>>::as_mut(self.buffer).unwrap()[index as usize] = color;
            }
        }
    }

    pub fn iter(&self) -> RectIter {
        let mut rect = self.rect.clone();
        rect.offset = Point::new(0, 0);
        rect.into_iter()
    }
}

pub struct WidgetCollection {
    pub background: Background, // should always be a Background object.
    pub layer1: Vec<Box<dyn Widget>>,
    pub layer2: Vec<Box<dyn Widget>>,
    pub layer3: Vec<Box<dyn Widget>>,
    pub overlay: Vec<Box<dyn Widget>>
}

impl WidgetCollection {
    pub fn new() -> Self {

        Self {
            background: Background { rect: Rect::new(0,0,0,0) },
            layer1: vec!(),
            layer2: vec!(),
            layer3: vec!(),
            overlay: vec!()
        }
   }
}
