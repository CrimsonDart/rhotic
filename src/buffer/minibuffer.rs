use std::fmt::Display;

use serde::{Serializer, ser::SerializeStruct};
use winit::keyboard::KeyCode;


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Minibuffer {
    bites: Vec<Bite>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bite {
    shift: bool,
    alt: bool,
    control: bool,
    key: KeyCode
}

impl Serialize for Bite {

}
