use rusttype::Font;

use crate::display::{Rgba, event_loop::{MouseState, KeyboardState}};
use crate::display::event_loop::Input;

use super::widgets::WidgetCollection;








// A singeton that contains all data of the application.
pub struct State {
    pub input: Input,
    pub is_focused: bool,
    pub time: u32,
    pub widgets: WidgetCollection,
}

impl State {
    pub fn advance(&mut self) {


        #[cfg(debug_assertions)] {
            println!("{}", self.mouse_state);
            println!("{}", self.keyboard_state);
            println!("{}", self.time);
        }

        self.mouse_state.advance_state();
        self.keyboard_state.advance_state();
    }
}
