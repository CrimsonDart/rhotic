
use softbuffer::{self, Surface};


use crate::state::application::State;

use super::types::Rgba;





// places the render function outside of the event loop to reduce clutter.
pub fn render(surface: &mut Surface, width: u32, height: u32, state: &State) {

    let mut buffer = surface.buffer_mut().unwrap();

    for index in 0..(width * height) {

        let y = index / width;
        let x = index % width;


        let color = Rgba::new(x % 255,
                 y % 255,
        (x * y) % 255,
        0);


        buffer[index as usize] = color.into();
    }

    buffer.present().unwrap();
}
