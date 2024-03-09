use std::path::PathBuf;

use rhotic_macro::text_and_render;

use crate::{buffer::{text_buffer::Page, stage::{Stage, Render}}, display::font::FontManager};




pub struct Dired {
    path: PathBuf,
    page: Page,
    cursor: usize,
}

impl Stage for Dired {

    fn init(init_args: ()) -> anyhow::Result<Self> {
        Ok(Self {
            path: PathBuf::new(),
            page: Default::default(),
            cursor: 0
        })
    }

    fn poll(&mut self, input: &crate::display::event_loop::Input) -> anyhow::Result<()> {
        Ok(())
    }

    const NAME: &'static str = "Dired";
}

impl Render<&mut FontManager> for Dired {
    fn render(&self, canvas: &mut crate::display::text_render::Canvas<&winit::window::Window, &winit::window::Window>, v: &mut FontManager) {

    }
}
