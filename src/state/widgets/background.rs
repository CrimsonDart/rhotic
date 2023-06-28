use crate::display::{Rect, Point};

use super::{Widget, DrawBuffer};


pub struct Background {
    pub rect: Rect
}

impl Widget for Background {
    fn get_rect(&self) -> crate::display::Rect {
        self.rect
    }

    fn draw(&self, mut buffer: super::DrawBuffer, state: &crate::state::application::State) {

        for Point {x, y} in self.rect.into_iter() {
            buffer.draw_to(x, y, 0xFF323232);
        }

    }
}
