use std::cell::Cell;

use crate::display::{Point, types::{DisplayPosition, DisplaySized, Pixel}};

use super::Widget;

pub struct Background {
    size: Cell<Point<u32>>,
}

impl Background {
    pub fn fit_to_window(&self, window_size: Pixel) {
        self.size.set(window_size)
    }
}

impl Default for Background {
    fn default() -> Self {
        Self {
            size: Cell::new(Point::new(0,0)),
        }
    }
}

impl DisplayPosition for Background {
    fn get_position(&self) -> Point<u32> {
        Pixel::new(0, 0)
    }
}

impl DisplaySized for Background {
    fn get_size(&self) -> Point<u32> {
        self.size.get()
    }
}

impl Widget for Background {
    fn draw(&self, mut buffer: super::DrawBuffer, state: &crate::state::application::State) {

        for Point {x, y} in self.size.get().iter() {
            buffer.draw_to(x, y, 0xFF323232);
        }
    }
    fn mouse_hover(&mut self, state: &crate::state::application::State) {}
}
