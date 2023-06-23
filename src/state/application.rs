use rusttype::Font;

use crate::display::{Rgba, event_loop::{MouseState, KeyboardState}};








// A singeton that contains all data of the application.
pub struct State {
    pub font: Font<'static>,
    pub display_text: String,
    pub background_color: Rgba,
    pub text_color: Rgba,
    pub mouse_state: MouseState,
    pub keyboard_state: KeyboardState,
    pub is_focused: bool
}

impl State {
    pub fn advance(&mut self) {

        println!("{}", self.mouse_state);
        println!("{}", self.keyboard_state);
        self.mouse_state.advance_state();
        self.keyboard_state.advance_state();

    }
}
