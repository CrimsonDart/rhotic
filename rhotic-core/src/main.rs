use display::event_loop::start_event_loop;

pub mod display;
pub mod state;
pub mod buffer;
pub mod file;
pub mod dired;

fn main() -> anyhow::Result<()> {

    start_event_loop()?;
    Ok(())
}
