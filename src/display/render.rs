


use fontdue::layout::{Layout, TextStyle};
use softbuffer::{self, Buffer};
use winit::window::Window;


use crate::state::application::State;

use super::{types::Pixel, Rgba, image::{Image, MonoImage, ColorRect}};


// IF THE WINDOW BORDERS RESIZE FASTER THAN THE WINDOW ITSELF THEN IT'S BECAUSE
// THE RENDERING TAKES TOO LONG
//
//
//
pub fn render(mut buffer: Buffer<&Window, &Window>, window_size: Pixel, state: &mut State) {


    buffer.fill(Rgba::new(32, 32, 32, 255).into());

    let size = state.glyph_scale;


    let font = &state.font;

    let line_size = font.horizontal_line_metrics(size).unwrap().new_line_size as usize;

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

    let mut new_lines = 0;
    let mut char_index = 0;

    let mut horizontal_cursor_offset = 0;


    for glyph in glyphs {

        if new_lines == state.buffer.line && char_index == state.buffer.cindex {
            horizontal_cursor_offset = glyph.x as isize;
        }

        if glyph.parent == '\n' {
            new_lines += 1;
            char_index = 0;
        } else {
            char_index += 1;
        }

        if !glyph.char_data.rasterize() {
            continue;
        }

        let image = match state.char_cache.get(&glyph.key) {
            Some(s) => s,
            None => {
                let r = font.rasterize_indexed(glyph.key.glyph_index, glyph.key.px).1;

                let new_image = MonoImage {
                    bytes: r,
                    width: glyph.width,
                    height: glyph.height

                };

                state.char_cache.insert(glyph.key, new_image);
                state.char_cache.get(&glyph.key).unwrap()
            }
        };
        //println!("'{}': {metrics:?}", glyph.parent);

        //println!("{} | {}", image.len(), metrics.width * metrics.height);

        draw_monochrome_image::<MonoImage, u8>(&mut buffer, window_size.x as usize, window_size.y as usize, glyph.x as isize, glyph.y as isize, image, Rgba::new(0,0,0,255), Rgba::new(255,255,255,255));

    }

    let line_position = layout.lines().unwrap()[new_lines];

    let top_of_line = line_position.baseline_y - line_position.max_ascent;



    draw_rectangle(&mut buffer, window_size.x as usize, window_size.y as usize, horizontal_cursor_offset, top_of_line as isize as isize, 2, line_position.max_new_line_size as usize, Rgba::new_opaque(200, 200, 255));
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


pub fn draw_image<'a, R: ColorRect<Rgba>>(mut buffer: Buffer<&Window, &Window>, win_width: usize, win_height: usize, x: isize, y: isize, image: &R) {
    let bytes = image.get_bytes();

    let mut gx = x;
    let mut gy = y;

    for counter in 0..image.get_bytes().len() {

        let (nx, ny) = (gx as usize, gy as usize);

        if nx < win_width && ny < win_height && gx >= 0 && gy >= 0 {
            buffer[ny * win_width + nx] = bytes[counter].into();
        }

        if gx == image.get_width() as isize + x - 1 {
            gx = x;
            gy += 1;
        } else {
            gx += 1;
        }
    }
}

pub fn draw_monochrome_image<'a, R: ColorRect<u8, u8>, C: Into<u32>>(buffer: &mut Buffer<&Window, &Window>, win_width: usize, win_height: usize, x: isize, y: isize, image: &R, black: Rgba, white: Rgba) {
    let bytes = image.get_bytes();

    let mut gx = x;
    let mut gy = y;

    for counter in 0..image.get_bytes().len() {

        if gx >= 0 && gy >= 0 {
            let (nx, ny) = (gx as usize, gy as usize);

            if nx < win_width && ny < win_height {

                let color = {
                    let mono = bytes[counter];
                    Rgba::new(mono, mono, mono, 255)
                };

                buffer[ny * win_width + nx] = color.into();
            }
        }

        if gx == image.get_width() as isize + x - 1 {
            gx = x;
            gy += 1;
        } else {
            gx += 1;
        }
    }
}

pub fn draw_rectangle(buffer: &mut Buffer<&Window, &Window>, win_width: usize, win_height: usize, x: isize, y: isize, rect_width: usize, rect_height: usize, color: Rgba) {
    let mut gx = x;
    let mut gy = y;

    for _ in 0..(rect_width * rect_height) {

        if gx >= 0 && gy >= 0  && gx < win_width as isize && gy < win_height as isize {
            buffer[gy as usize * win_width + gx as usize] = color.into();
        }

        if gx == rect_width as isize + x - 1 {
            gx = x;
            gy += 1;
        } else {
            gx += 1;
        }
    }
}
