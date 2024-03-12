use std::{path::PathBuf, str::FromStr, ffi::{OsStr, OsString}};

use anyhow::bail;
use rhotic_macro::text_and_render;

use crate::{buffer::{text_buffer::Page, stage::{Stage, Render, layout, get_image}}, display::{font::FontManager, Rgba, image::MonoImage}};

pub struct Dired {
    path: PathBuf,
    page: Page,
    cursor: usize,
    files: Vec<OsString>,
}

impl Dired {
    fn read_dir(&mut self) -> anyhow::Result<()> {

        let dir = self.path.read_dir()?;

        let vec: Vec<_> = dir.map(|x| {

            if let Ok(s) = x {
                s.file_name()
            } else {
                OsString::from("-- failed to display --")
            }

        }).collect();

        self.files = vec;
        self.page.clear();

        for string in self.files.iter() {

            self.page.push_line(if let Some(s) = string.to_str() {
                s
            } else {
                "-- failed to convert --"
            });
        }

        Ok(())
    }
}

impl Stage for Dired {

    fn init(init_args: &[&str]) -> anyhow::Result<Self> {

        let mut path_buf = if let Some(path) = init_args.get(0) {
            PathBuf::from_str(path)?
        } else {
            bail!("Tried to open Dired without a path. A path is needed!")
        };

        if !path_buf.is_dir() {
            path_buf.pop();
        }

        if !path_buf.is_dir() {
            bail!("Failed to open Dired; Can only open with a Directory path!")
        }

        Ok(Self {
            path: path_buf,
            page: Default::default(),
            cursor: 0,
            files: Vec::new()
        })
    }

    fn poll(&mut self, input: &crate::display::event_loop::Input) -> anyhow::Result<()> {

        self.read_dir()?;



        Ok(())
    }

    const NAME: &'static str = "Dired";
}

impl Render<&mut FontManager> for Dired {
    fn render(&self, canvas: &mut crate::display::text_render::Canvas<&winit::window::Window, &winit::window::Window>, v: &mut FontManager) {



        let layout = layout(self.page.as_string(), v);
        let glyphs = layout.glyphs();
        let (mut gx, mut gy) = (0,0);
        let cursor = self.cursor;

        if let Some(lines) = layout.lines() {
            if let Some(line) = lines.get(cursor) {
                canvas.draw_rectangle(
                    0,
                    line.baseline_y as isize - line.max_ascent as isize,
                    canvas.width(),
                    line.max_new_line_size as usize,
                    Rgba::new_opaque(0x60, 0xAF, 0xFF)
                )
            }
        }

        for glyph in glyphs {

            if glyph.parent == '\n' {
                gy += 1;
                gx = 0;
            } else {
                gx += 1;
            }

            if !glyph.char_data.rasterize() {
                continue;
            }

            let line_background_color: Rgba = if gy == cursor {
                Rgba::new_opaque(0x60, 0xAF, 0xFF)
            } else {
                Rgba::DARK_GRAY
            };

            let (_metrics, image) = get_image(glyph, v);

            canvas.draw_monochrome_image::<MonoImage, u8>(
                glyph.x as isize,
                glyph.y as isize,
                image,
                line_background_color,
                Rgba::WHITE
            );

        }

    }
}
