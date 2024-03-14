
use softbuffer::{self, Buffer};
use winit::window::Window;


use crate::{state::application::State};

use super::{types::Pixel, Rgba, text_render::{Canvas}};


// IF THE WINDOW BORDERS RESIZE FASTER THAN THE WINDOW ITSELF THEN IT'S BECAUSE
// THE RENDERING TAKES TOO LONG
//
//
//
pub fn render(mut buffer: Buffer<&Window, &Window>, window_size: Pixel, state: &mut State) {

    buffer.fill(Rgba::DARK_GRAY.into());

    let mut canvas = Canvas::new(buffer, window_size.x as usize, window_size.y as usize);
    use crate::buffer::stage::Render;
    state.stage.render(&mut canvas, &mut state.font_manager);

    canvas.destroy().present().unwrap();
}

