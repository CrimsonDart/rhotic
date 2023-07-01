use image::buffer;
use softbuffer::Buffer;
use winit::window;

use crate::display::{Point, types::{RectIter, DisplayRect, DisplayPosition, DisplaySized}};

use self::background::Background;

use super::application::State;

pub mod button;
pub mod background;

pub trait Widget where Self: DisplayPosition + DisplaySized {
    fn draw(&self, buffer: DrawBuffer, state: &State);
}

pub fn draw_to_buffer<'a>(widget: &Box<dyn Widget>, buffer: &mut Buffer<'a>, window_size: Point<u32>, state: &State) {

    let buffref = buffer as *mut Buffer<'a>;


    let draw_buffer = DrawBuffer {
        buffer: buffref,
        size: widget.get_size(),
        position: widget.get_position(),
        window_size
    };
    widget.draw(draw_buffer, state);
}

pub struct DrawBuffer<'a> {
    pub buffer: *mut Buffer<'a>,
    pub size: Point<u32>,
    pub position: Point<u32>,
    pub window_size: Point<u32>
}

impl<'a> DrawBuffer<'a> {
    pub fn draw_to(&mut self, x: u32, y: u32, color: u32) {

        let size = self.size;
        let offset = self.position;
        let window_size = self.window_size;

        let display_point = Point::new(x + offset.x, y + offset.y);

        if display_point.x >= window_size.x {
            return;
        }

        let index = display_point.y * window_size.x + display_point.x;
        let index = index as usize;

        let buffer = unsafe {
            <*mut Buffer<'a>>::as_mut(self.buffer).unwrap()
        };

        let length = buffer.len();
        if (index) < length {
            buffer[index] = color;
        }
    }
}

impl<'a> DisplayPosition for DrawBuffer<'a> {
    fn get_position(&self) -> Point<u32> {
        self.position
    }
}

impl<'a> DisplaySized for DrawBuffer<'a> {
    fn get_size(&self) -> Point<u32> {
        self.size
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
            background: Background::default(),
            layer1: vec!(),
            layer2: vec!(),
            layer3: vec!(),
            overlay: vec!()
        }
   }
}
