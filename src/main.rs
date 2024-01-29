use display::event_loop::start_event_loop;

mod display;
mod state;
mod buffer;

fn main() -> anyhow::Result<()> {

    let font = display::font::load_ttf("./assets/fonts/FiraCode-Regular.ttf")?;



    let (metrics, image) = font.rasterize('M', 50.0);

    println!("{}", image.len());

    println!("{:?}", metrics);
    let width = metrics.width;

    for index in 0..image.len() {
        let value = image[index];

        print!("{:2x?}", value);
        if index % width == width - 1 {
            print!("\n");
        }
    }
    start_event_loop()?;
    Ok(())
}
