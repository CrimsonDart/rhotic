use display::event_loop::start_event_loop;

mod display;
mod state;
mod buffer;

fn main() -> anyhow::Result<()> {

    start_event_loop()?;
    Ok(())
}
