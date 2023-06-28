use crate::{display::Rect, state::application::State};

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

    fn draw(&self, buffer: DrawBuffer, state: &State) {






    }
}
