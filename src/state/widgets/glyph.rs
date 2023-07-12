use crate::{display::{types::{DisplayPosition, Pixel, DisplaySized}, Point, Rgba}, basic::Name};

use super::Widget;
use bitvec::{prelude::*, index::BitIdx};





pub struct Glyph {
    pub position: Pixel,
    pub character: char,
    pub color: u32
}

impl DisplayPosition for Glyph {
    fn get_position(&self) -> crate::display::Point<u32> {
        self.position
    }
}

impl DisplaySized for Glyph {
    fn get_size(&self) -> crate::display::Point<u32> {
        Point::new(16, 16)
    }
}

impl Name for Glyph {
    fn name(&self) -> &'static str {
        "Glyph"
    }
}

impl Widget for Glyph {
    fn draw(&self, mut buffer: super::DrawBuffer, state: &crate::state::application::State) {

        let raster = self.raster();

        for yindx in 0..8usize {
            let row = raster[yindx];
            let arr: BitArray<u8, Msb0> = row.into_bitarray();
            for xindx in 0..8usize {
                let bit = arr[xindx];
                if bit {

                    let x = xindx as u32 * 2;
                    let y = yindx as u32 * 2;

                    buffer.draw_to(x, y, self.color);
                    buffer.draw_to(x + 1, y, self.color);
                    buffer.draw_to(x, y + 1, self.color);
                    buffer.draw_to(x + 1, y + 1, self.color);
                }
            }
        }
    }
}

impl Glyph {


    fn raster(&self) -> &'static [u8; 8] {

        match self.character {
            'A' => &glyph_upper_a,
            _ => &glyph_null
        }
    }

    pub fn new(x: u32, y: u32, character: char, color: u32) -> Self {
        Self {
            character,
            position: Point { x, y },
            color
        }
    }



}

const glyph_null: [u8; 8] =
                [0b_00000000,
                 0b_01111110,
                 0b_01000010,
                 0b_01000010,
                 0b_01000010,
                 0b_01000010,
                 0b_01111110,
                 0b_00000000];

const glyph_upper_a: [u8; 8] =
    [0b00011000,
    0b_00111100,
    0b_01100110,
    0b_01111110,
    0b_01100110,
    0b_01100110,
    0b_01100110,
    0b_00000000];
