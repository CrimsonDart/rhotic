use rusttype::Font;

use crate::display::{Rgba, event_loop::MouseState};








// A singeton that contains all data of the application.
pub struct State {
    pub font: Font<'static>,
    pub display_text: String,
    pub background_color: Rgba,
    pub text_color: Rgba,
    pub mouse_state: MouseState
}
