use display::event_loop::start_event_loop;


mod display;
mod state;
mod basic;


fn main() {

    let font = display::font::load_ttf("./assets/fonts/FiraCode-Regular.ttf");

    let font = match font {
        Ok(face) => face,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let (metrics, image) = font.rasterize_indexed(1, 50.0);

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





























    //start_event_loop().unwrap()
}
