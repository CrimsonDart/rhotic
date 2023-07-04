use crate::display::event_loop::Input;
use super::widgets::WidgetCollection;

// A singeton that contains all data of the application.
pub struct State {
    pub input: Input,
    pub is_focused: bool,
    pub widgets: WidgetCollection,
}

impl State {
    pub fn advance(&mut self) {

        #[cfg(debug_assertions)] {
            println!("{}", self.input);
        }




















        self.input.advance_state();
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            input: Input::default(),
            is_focused: false,
            widgets: WidgetCollection::new()
        }
    }
}
