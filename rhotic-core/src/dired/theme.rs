use crate::buffer::stage::Configurable;
use toml::Table;

use crate::display::font::Face;
use crate::display::types::Rgba;
use crate::display::font::{Style, Underline};







#[derive(Debug)]
pub struct DiredTheme {

    pub file_color: Rgba,
    pub directory_color: Rgba,
    pub select_color: Rgba
}

impl Default for DiredTheme {
    fn default() -> Self {
        Self {
            file_color: Rgba::WHITE,
            directory_color: Rgba::GREEN,
            select_color: Rgba::new_opaque(0x60, 0xAF, 0xFF)
        }
    }
}
