use std::time::Instant;

use winit::{keyboard::PhysicalKey, event::MouseScrollDelta};

use crate::display::types::Pixel;

#[derive(Clone, Debug)]
pub enum InputEvent {
    Text(String),
    Press(PhysicalKey),
    Release(PhysicalKey),
    Echo(PhysicalKey, Instant),
    MousePress(usize),
    MouseRelease(usize),
    MouseScroll(MouseScrollDelta),
    MouseMove(Pixel),
    Poll
}
