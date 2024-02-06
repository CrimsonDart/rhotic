


use fontdue::layout::{Layout, TextStyle};
use softbuffer::{self, Buffer};
use winit::window::Window;


use crate::state::application::State;

type Buffa<'a> = Buffer<'a, &'a Window, &'a Window>;

use super::{types::Pixel, Rgba, image::{Image, MonoImage, ColorRect}};


// IF THE WINDOW BORDERS RESIZE FASTER THAN THE WINDOW ITSELF THEN IT'S BECAUSE
// THE RENDERING TAKES TOO LONG
//
//
//
pub fn render<'a>(buffer: &'a mut Buffa, window_size: Pixel, state: &mut State) {

    buffer.fill(Rgba::new(32, 32, 32, 255).into());

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

        let image = match state.char_cache.get(&glyph.key) {
            Some(s) => s.clone(),
            None => {
                let r = font.rasterize_indexed(glyph.key.glyph_index, glyph.key.px).1;

                let new_image = MonoImage {
                    bytes: r,
                    width: glyph.width,
                    height: glyph.height

                };

                state.char_cache.insert(glyph.key, new_image);
                state.char_cache.get(&glyph.key).unwrap().clone()
            }
        };
        //println!("'{}': {metrics:?}", glyph.parent);

        //println!("{} | {}", image.len(), metrics.width * metrics.height);

        draw_monochrome_image::<MonoImage, u8>(buffer, window_size.x as usize, window_size.y as usize, glyph.x as isize, glyph.y as isize, image, Rgba::new(0,0,0,255), Rgba::new(255,255,255,255));
    }
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


pub fn draw_image<'a, R: ColorRect<Rgba>>(mut buffer: Buffa<'a>, win_width: usize, win_height: usize, x: isize, y: isize, image: &R) -> Buffa<'a> {
    let bytes = image.get_bytes();

    let mut gx = x;
    let mut gy = y;

    for counter in 0..image.get_bytes().len() {

        if gx < 0 || gy < 0 { continue; }

        let (nx, ny) = (gx as usize, gy as usize);

        if nx < win_width && ny < win_height {
            buffer[ny * win_width + nx] = bytes[counter].into();
        }

        if gx == image.get_width() as isize + x {
            gx = x;
            gy += 1;
        } else {
            gx += 1;
        }
    }
    buffer
}

pub fn draw_monochrome_image<'a, R: ColorRect<u8, u8>, C: Into<u32>>(buffer: &'a mut Buffa, win_width: usize, win_height: usize, x: isize, y: isize, image: &R, black: Rgba, white: Rgba) {
    let bytes = image.get_bytes();

    let mut gx = x;
    let mut gy = y;

    for counter in 0..image.get_bytes().len() {

        if gx < 0 || gy < 0 { continue; }

        let (nx, ny) = (gx as usize, gy as usize);

        if nx < win_width && ny < win_height {

            let color = match bytes[counter] {
                0 | 1 => black,
                255 | 254 => white,
                c => {

                    Rgba::from(0)
                }
            };

            buffer[ny * win_width + nx] = color.into();
        }

        if gx == image.get_width() as isize + x {
            gx = x;
            gy += 1;
        } else {
            gx += 1;
        }
    }
}
