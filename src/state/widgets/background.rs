use crate::display::{Point, types::{DisplayPosition, DisplaySized}};

use super::Widget;


pub struct Background {
    pub size: Point<u32>,
    pub position: Point<u32>
}

impl Default for Background {
    fn default() -> Self {
        Self {
            size: Point::new(0,0),
            position: Point::new(0,0)
        }
    }
}

impl DisplayPosition for Background {
    fn get_position(&self) -> Point<u32> {
        self.position
    }
}

impl DisplaySized for Background {
    fn get_size(&self) -> Point<u32> {
        self.size
    }
}

impl Widget for Background {
    fn draw(&self, mut buffer: super::DrawBuffer, state: &crate::state::application::State) {

        for Point {x, y} in self.size.iter() {
            buffer.draw_to(x, y, 0xFF323232);
        }

    }
}
