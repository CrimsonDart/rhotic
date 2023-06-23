
use softbuffer::{self, Surface};


use crate::state::application::State;

use super::types::Rgba;





// places the render function outside of the event loop to reduce clutter.
//
//
// IF THE WINDOW BORDERS RESIZE FASTER THAN THE WINDOW ITSELF THEN IT'S BECAUSE
// THE RENDERING TAKES TOO LONG
//
//
//
pub fn render(surface: &mut Surface, width: u32, height: u32, _state: &State) {


    let mut buffer = surface.buffer_mut().unwrap();


    for index in 0..(width * height) {




        buffer[index as usize] = Rgba::new(255, 255, 255, 255).into();
    }

    buffer.present().unwrap();
}
