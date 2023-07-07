use crate::{display::{Point, types::{DisplayPosition, DisplaySized}}, state::application::State};

use super::{Widget, DrawBuffer};

pub struct Button {
    size: Point<u32>,
    position: Point<u32>,

}

impl Button {
    pub fn new(size: Point<u32>, position: Point<u32>) -> Self {
        Self {size, position}
    }

    #[inline]
    pub fn new_u32(sx: u32, sy: u32, px: u32, py: u32) -> Self {
        Self { size: Point::new(sx, sy), position: Point::new(px, py)}
    }
}

impl DisplayPosition for Button {
    #[inline]
    fn get_position(&self) -> Point<u32> {
        self.position
    }
}

impl DisplaySized for Button {
    #[inline]
    fn get_size(&self) -> Point<u32> {
        self.size
    }
}

impl Widget for Button {

    fn draw(&self, mut buffer: DrawBuffer, state: &State) {
        for Point {x, y} in buffer.size.iter() {

            if state.is_focused {
                buffer.draw_to(x, y, 0xFFFF0000);
            } else {
                buffer.draw_to(x, y, 0xFF00FF00);
            }
        }
    }

    fn mouse_hover(&mut self, state: &State) {}
}
