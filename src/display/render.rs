
use std::{ops::Deref, boxed};

use softbuffer::{self, Surface, Buffer};


use crate::state::{application::State, widgets::{WidgetCollection, Widget, draw_to_buffer, DrawBuffer}};

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


    let buffref = &mut buffer as *mut Buffer;

    let draw_buffer = DrawBuffer {
        buffer: buffref,
        rect: widgets.background.rect,
        window_size
    };
    widgets.background.draw(draw_buffer, state);



    for widget in widgets.layer1.iter() {
        draw_to_buffer(widget, &mut buffer, window_size, state);
    }
    for widget in widgets.layer2.iter() {
        draw_to_buffer(widget, &mut buffer, window_size, state);
    }
    for widget in widgets.layer3.iter() {
        draw_to_buffer(widget, &mut buffer, window_size, state);
    }
    for widget in widgets.overlay.iter() {
        draw_to_buffer(widget, &mut buffer, window_size, state);
    }








    buffer.present().unwrap();
}
