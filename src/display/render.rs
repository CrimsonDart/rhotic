


use fontdue::layout::{Layout, TextStyle};
use softbuffer::{self, Buffer};
use winit::window::Window;


use crate::state::application::State;

use super::{types::Pixel, Rgba};


// IF THE WINDOW BORDERS RESIZE FASTER THAN THE WINDOW ITSELF THEN IT'S BECAUSE
// THE RENDERING TAKES TOO LONG
//
//
//
pub fn render(mut buffer: Buffer<&Window, &Window>, window_size: Pixel, state: &mut State) {

    buffer.fill(Rgba::new(32, 32, 32, 255).into());

    let cursor = state.buffer.cursor;
    let size = state.glyph_scale;


    let font = &state.font;

    let line_size = font.horizontal_line_metrics(size).unwrap().new_line_size;


    let mut layout = Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
    let text = state.buffer.page.as_string();

    let text = TextStyle {
        text: text.as_str(),
        px: size,
        font_index: 0,
        user_data: ()
    };
    layout.append(&[font], &text);

    let glyphs = layout.glyphs();

    for glyph in glyphs {


        if !glyph.char_data.rasterize() {
            continue;
        }

        let (_, image) = match state.char_cache.get(&glyph.key) {
            Some(s) => s.clone(),
            None => {
                let r = font.rasterize_indexed(glyph.key.glyph_index, glyph.key.px);
                state.char_cache.insert(glyph.key, r);
                state.char_cache.get(&glyph.key).unwrap().clone()
            }
        };
        //println!("'{}': {metrics:?}", glyph.parent);

        //println!("{} | {}", image.len(), metrics.width * metrics.height);

        let mut x_pos = glyph.x as isize;
        let mut y_pos = glyph.y as isize;

        let mut local_x = 0;

        for index in 0..image.len() {
            let gray = image[index] as u32;
            let color = Rgba::new(gray, gray, gray, 255);
            let win_width = window_size.x as isize;

            let bufdex = (y_pos * win_width + x_pos) as usize;


            if bufdex <= buffer.len() && x_pos < win_width {
                buffer[bufdex] = color.into();
            }

            local_x += 1;

            if local_x == glyph.width as isize {
                x_pos = glyph.x as isize;
                local_x = 0;
                y_pos += 1;
            } else {
                x_pos += 1;
            }
        }
    }

    for x in 0..2 {
        for y in 0..(line_size as usize) {

            buffer[y*window_size.x as usize + x] = Rgba::new(255, 0,0, 255).into();
        }
    }




    buffer.present().unwrap();
}

fn draw_horizontal_line(height: usize, buffer: &mut Buffer<&Window, &Window>, window_size: &Pixel, color: Rgba) {
    if window_size.y as usize <= height {
        return;
    }
    let width = window_size.x as usize;
    let line = height * width;
    for index in 0..width {
        buffer[index + line] = color.into();
    }
}
