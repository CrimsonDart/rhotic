use crate::{display::{Rect, Point}, state::application::State};

use super::{Widget, DrawBuffer};





struct Button {
    rect: Rect,
}

impl Button {
    pub fn new(rect: Rect) -> Self {
        Self {rect}
    }
}

impl Widget for Button {

    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn draw(&self, mut buffer: DrawBuffer, state: &State) {

        for Point {x, y} in buffer.iter() {
            buffer.draw_to(x, y, 0xFFFFFFFF);
        } 




    }
}
