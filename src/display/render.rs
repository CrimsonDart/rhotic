
use softbuffer::{self, Surface, Buffer};


use crate::state::{application::State, widgets::{WidgetCollection, Widget, draw_to_buffer}};

use super::{types::Rgba, Rect, Point};





// places the render function outside of the event loop to reduce clutter.
//
//
// IF THE WINDOW BORDERS RESIZE FASTER THAN THE WINDOW ITSELF THEN IT'S BECAUSE
// THE RENDERING TAKES TOO LONG
//
// or maybe not actually, rendering is pretty cheap....
//
//
pub fn render(mut buffer: Buffer, window_size: Point<u32>, state: &State, widgets: &WidgetCollection) {

    draw_to_buffer(&widgets.background, &mut buffer, window_size, state);









    buffer.present().unwrap();
}
