use crate::buffer::stage::Configurable;
use toml::Table;

use crate::display::font::Face;
use crate::display::types::Rgba;
use crate::display::font::{Style, Underline};







#[derive(Debug)]
pub struct DiredTheme {
    array: [crate::display::font::Face; 4]
}

pub enum DiredFaces {
    File,
    Directory,
    FileSelect,
    DirSelect,
}

impl Default for DiredTheme {
    fn default() -> Self {
        Self {
            array: [
                Face::default(),
                Face {
                    fore: Rgba::WHITE,
                    back: Rgba::DARK_GRAY,
                    scale: 1.0,
                    style: Style::Bold,
                    underline: Underline::None
                },
                Face {
                    fore: Rgba::WHITE,
                    back: Rgba::new_opaque(0x60, 0xAF, 0xFF),
                    scale: 1.0,
                    style: Style::Bold,
                    underline: Underline::None
                },
                Face {
                    fore: Rgba::WHITE,
                    back: Rgba::new_opaque(0x60, 0xAF, 0xFF),
                    scale: 1.0,
                    style: Style::None,
                    underline: Underline::None
                },
            ]
        }
    }
}

impl std::ops::Index<DiredFaces> for DiredTheme {
    type Output = crate::display::font::Face;
    fn index(&self, rdx: DiredFaces) -> &Self::Output {
        &self.array[rdx as usize]
    }
}

impl std::ops::IndexMut<DiredFaces> for DiredTheme {
    fn index_mut(&mut self, index: DiredFaces) -> &mut Self::Output {
        &mut self.array[index as usize]
    }
}

impl Configurable for DiredTheme {
    fn configure(&mut self, config: Table) -> anyhow::Result<()> {
        Ok(())
    }

    fn default_configuration() -> Table {
        let out = Table::new();
        out
    }

    const CONFIG_FILE_NAME: &'static str = "diredtheme.toml";
}
