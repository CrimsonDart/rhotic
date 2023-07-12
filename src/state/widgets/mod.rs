use std::{rc::Rc, cell::{RefCell, Cell}};

use image::buffer;
use softbuffer::Buffer;
use winit::window;

use crate::{display::{Point, types::{RectIter, DisplayRect, DisplayPosition, DisplaySized, Pixel}}, basic::Name};

use self::background::Background;

use super::application::State;

pub mod button;
pub mod background;
pub mod glyph;


pub trait Widget where Self: DisplayRect + Name {
    fn draw(&self, buffer: DrawBuffer, state: &State);
}

pub fn draw_to_buffer<'a>(widget: HeapWidget, buffer: &mut Buffer<'a>, window_size: Point<u32>, state: &State) {

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

pub type HeapWidget = Rc<dyn Widget>;

pub struct WidgetCollection {
    pub background: Rc<Background>, // should always be a Background object.
    pub layer1: Vec<HeapWidget>,

    pub top_widget_cache: HeapWidget
}

impl Default for WidgetCollection {
    fn default() -> Self {

        let background = Rc::new(Background::default());
        Self {
            background: background.clone(),
            layer1: vec!(),
            top_widget_cache: background
        }
    }
}

impl WidgetCollection {
    pub fn get_top_at(&self, mouse_position: Pixel) -> HeapWidget {

        for widget in self.layer1.iter() {
            if widget.contains(mouse_position) {
                return widget.clone();
            }
        }
        self.background.clone()
    }
}

pub enum WidgetEvent {
    Hover,
    UnHover,
    Input
}
