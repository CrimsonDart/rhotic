

use crate::display::event_loop::Input;
use super::widgets::{WidgetCollection, HeapWidget};





// A singeton that contains all data of the application.
#[derive(Default)]
pub struct State {
    pub input: Input,
    pub is_focused: bool,
    pub widgets: WidgetCollection,
    pub is_colored: bool,
}

impl State {

    pub fn new() -> Self {
        Self {
            is_focused: false,
            ..Default::default()
        }
    }



    pub fn advance(&mut self) {


        #[cfg(debug_assertions)] {
            println!("{}", self.input);
        }


        let mouse_hovered = self.widgets.get_top_at(self.input.mouse_position);


        self.is_colored = mouse_hovered.name() == "Button";














        self.input.advance_state();
    }
}
