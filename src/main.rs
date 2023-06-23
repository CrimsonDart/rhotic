
mod display;
mod state;

use display::event_loop::start_event_loop;

fn main() {
    start_event_loop().unwrap()
}
