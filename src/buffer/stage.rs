use crate::{display::event_loop::Input, file::toml::Toml};

pub trait Stage where Self: Default {
    fn poll(&mut self, input: &Input) -> anyhow::Result<()>;
    const NAME: &'static str;
}

pub trait Configurable where Self: Default + Stage {
    fn configure(&mut self, config: Toml) -> anyhow::Result<()>;
    fn default_configuration() -> Toml;
    const CONFIG_FILE_NAME: &'static str;
}
