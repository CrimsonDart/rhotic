


use std::collections::{btree_map::OccupiedEntry, hash_map::{VacantEntry, Entry}};

use fontdue::{layout::{Layout, TextStyle, GlyphRasterConfig, GlyphPosition}, Metrics};
use softbuffer::{self, Buffer};
use winit::window::Window;


use crate::{state::application::State, buffer::textstage::Mode};

use super::{types::Pixel, Rgba, image::{Image, MonoImage, ColorRect}, text_render::{self, Canvas}};


// IF THE WINDOW BORDERS RESIZE FASTER THAN THE WINDOW ITSELF THEN IT'S BECAUSE
// THE RENDERING TAKES TOO LONG
//
//
//
pub fn render(mut buffer: Buffer<&Window, &Window>, window_size: Pixel, state: &mut State) {

    buffer.fill(Rgba::DARK_GRAY.into());

    let layout = layout(&state);
    let glyphs = layout.glyphs();

    let mut new_lines = 0;
    let mut char_index = 0;

    let (cursor_x, cursor_y) = state.stage.get_real_cursor();


    for glyph in glyphs {


        if new_lines == cursor_y && char_index == cursor_x {

            let mode = state.stage.mode;

            let (metrics, image) = get_image(glyph, state);


            let cursor_left_bound = glyph.x as isize - metrics.xmin as isize;

            let line_position = layout.lines().unwrap()[new_lines];
            let top_of_line = line_position.baseline_y - line_position.max_ascent;

            if mode == Mode::Command {
                draw_rectangle(
                    &mut buffer,
                    window_size.x as usize,
                    window_size.y as usize,
                    cursor_left_bound,
                    top_of_line as isize,
                    metrics.advance_width as usize,
                    line_position.max_new_line_size as usize,
                    Rgba::new_opaque(0x60, 0xAF, 0xFF)
                );

                if glyph.char_data.rasterize() {

                    draw_monochrome_image::<MonoImage, u8>(
                        &mut buffer,
                        window_size.x as usize,
                        window_size.y as usize,
                        glyph.x as isize,
                        glyph.y as isize,
                        image,
                        Rgba::new_opaque(0x60, 0xAF, 0xFF),
                        Rgba::WHITE
                    );
                }

            } else {
                if glyph.char_data.rasterize() {

                    draw_monochrome_image::<MonoImage, u8>(
                        &mut buffer,
                        window_size.x as usize,
                        window_size.y as usize,
                        glyph.x as isize,
                        glyph.y as isize,
                        image,
                        Rgba::DARK_GRAY,
                        Rgba::WHITE
                    );
                }

                draw_rectangle(
                    &mut buffer,
                    window_size.x as usize,
                    window_size.y as usize,
                    cursor_left_bound,
                    top_of_line as isize,
                    2,
                    line_position.max_new_line_size as usize,
                    Rgba::new_opaque(0x60, 0xAF, 0xFF)
                );

            }
        } else {
            if glyph.char_data.rasterize() {
                let image = &get_image(glyph, state).1;

                draw_monochrome_image::<MonoImage, u8>(
                    &mut buffer,
                    window_size.x as usize,
                    window_size.y as usize,
                    glyph.x as isize,
                    glyph.y as isize,
                    image,
                    Rgba::DARK_GRAY,
                    Rgba::WHITE
                );
            }

        }



        if glyph.parent == '\n' {
            new_lines += 1;
            char_index = 0;
        } else {
            char_index += 1;
        }
    }

    buffer.present().unwrap();
}

fn layout(state: &State) -> Layout {
    let mut layout = Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
    let text = state.stage.page.as_string();

    let text = TextStyle {
        text: text.as_str(),
        px: state.font_manager.scale,
        font_index: 0,
        user_data: ()
    };
    layout.append(&[&state.font_manager.fonts[0]], &text);
    layout
}

fn get_image<'a>(glyph: &GlyphPosition, state: &'a mut State) -> &'a (Metrics, MonoImage) {

    state.font_manager.cache.entry(glyph.key).or_insert({
        let (metrics, raster) = state.font_manager.fonts[0].rasterize_indexed(glyph.key.glyph_index, glyph.key.px);

        let new_image = MonoImage {
            bytes: raster,
            width: glyph.width,
            height: glyph.height
        };
        (metrics, new_image)

    })
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

pub fn draw_monochrome_image<'a, R: ColorRect<u8, u8>, C: Into<u32>>

    (
        buffer: &mut Buffer<&Window, &Window>,
        win_width: usize, win_height: usize,
        x: isize, y: isize,
        image: &R,
        black: Rgba,
        white: Rgba
    ) {

    let bytes = image.get_bytes();

    let mut gx = x;
    let mut gy = y;

    for counter in 0..image.get_bytes().len() {

        if gx >= 0 && gy >= 0 {
            let (nx, ny) = (gx as usize, gy as usize);

            if nx < win_width && ny < win_height {



                let color = match bytes[counter] {
                    0 => { black },
                    255 => {white},
                    b => { black.blend(white, b) }
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
