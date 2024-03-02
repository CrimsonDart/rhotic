use winit::window::Window;

use crate::{display::{event_loop::Input, text_render::Canvas}, file::toml::Toml, state::application::State};

pub trait Stage where Self: Default {
    fn poll(&mut self, input: &Input) -> anyhow::Result<()>;
    const NAME: &'static str;
}

pub trait Configurable where Self: Default + Stage {
    fn configure(&mut self, config: Toml) -> anyhow::Result<()>;
    fn default_configuration() -> Toml;
    const CONFIG_FILE_NAME: &'static str;
}

pub trait Render where Self: Stage {
    fn render(&self, canvas: Canvas<&Window, &Window>, stage: &State);
}

pub trait TextStage where Self: Stage {
    fn get_display_text(&self) -> String;
}

impl<T: TextStage> Render for T {
    fn render(&self, mut canvas: Canvas<&Window, &Window>, state: &State) {

    }
}
