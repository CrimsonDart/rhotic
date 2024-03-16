use fontdue::{layout::{Layout, TextStyle, GlyphPosition}, Metrics};
use winit::{window::Window, event::MouseScrollDelta, keyboard::SmolStr};

use crate::{display::{event_loop::{Key}, text_render::Canvas, font::FontManager, image::MonoImage, Rgba}};

use toml::Table;

pub trait Stage where Self: Sized {
    fn init(input: &[&str]) -> anyhow::Result<Self>;
    fn send_event(&mut self, input: InputEvent) -> StateCommand;
    const NAME: &'static str;
}

pub enum InputEvent<'a> {
    Press(Key),
    Echo(Key),
    Release(Key),
    MouseMove(usize, usize),
    Scroll(MouseScrollDelta),
    Text(SmolStr),
    Command(&'a [&'a str])
}

pub enum StateCommand {
    StartStage(String),
    Log(String),
    None,
    // Add log command?
}

impl<T> From<Option<T>> for StateCommand {
    fn from(_value: Option<T>) -> Self {
        StateCommand::None
    }
}

pub trait Configurable {
    fn configure(&mut self, config: Table) -> anyhow::Result<()>;
    fn default_configuration() -> Table;
    const CONFIG_FILE_NAME: &'static str;
}

pub trait Render<V = ()> {
    fn render(&mut self, canvas: &mut Canvas<&Window, &Window>, v: V);
}

pub trait TextStage {
    fn get_display_text(&self) -> String;
    fn get_cursor(&self) -> (usize, usize, CursorLook);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CursorLook {
    VerticalBar,
    HorizontalBar,
    Block,
    Box,
}

impl<T: TextStage> Render<&mut FontManager> for T {
    fn render(&mut self, canvas: &mut Canvas<&Window, &Window>, v: &mut FontManager) {
        use CursorLook::*;

        let layout = layout(self.get_display_text(), v);
        let glyphs = layout.glyphs();
        let (mut dx, mut dy) = (0,0);
        let (cx, cy, ctype) = self.get_cursor();

        for glyph in glyphs {

            const CURSOR_COLOR: Rgba = Rgba::new_opaque(0x60, 0xAF, 0xFF);

            let cursor_render = dy == cy && dx == cx;
            let (metrics, image) = get_image(glyph, v);

            let cursor_left_bound = glyph.x as isize - metrics.xmin as isize;
            let cursor_width = metrics.advance_width as usize;
            let line_position = layout.lines().unwrap()[dy];
            let line_top_bound = (line_position.baseline_y - line_position.max_ascent) as isize;
            let line_height = line_position.max_new_line_size as usize;

            if ctype != Block && glyph.char_data.rasterize() {
                canvas.draw_monochrome_image::<MonoImage, u8>(
                    glyph.x as isize,
                    glyph.y as isize,
                    image,
                    Rgba::DARK_GRAY,
                    Rgba::WHITE
                );
            }

            if cursor_render {
                match ctype {
                VerticalBar => {
                    canvas.draw_rectangle(
                        cursor_left_bound,
                        line_top_bound,
                        2,
                        line_height,
                        CURSOR_COLOR
                    );
                },
                HorizontalBar => {
                    canvas.draw_rectangle(
                        cursor_left_bound,
                        line_top_bound + line_height as isize - 2,
                        cursor_width,
                        2,
                        CURSOR_COLOR
                    );
                },
                Block => {
                    canvas.draw_rectangle(
                        cursor_left_bound,
                        line_top_bound,
                        cursor_width,
                        line_height,
                        CURSOR_COLOR
                    );
                },
                Box => {
                    // left line
                    canvas.draw_rectangle(
                        cursor_left_bound,
                        line_top_bound,
                        1,
                        line_height,
                        CURSOR_COLOR
                    );

                    // bottom line
                    canvas.draw_rectangle(
                        cursor_left_bound,
                        line_top_bound + line_height as isize - 1,
                        cursor_width,
                        1,
                        CURSOR_COLOR
                    );

                    // top line
                    canvas.draw_rectangle(
                        cursor_left_bound,
                        line_top_bound,
                        cursor_width,
                        1,
                        CURSOR_COLOR
                    );

                    // right line
                    canvas.draw_rectangle(
                        cursor_left_bound + cursor_width as isize - 1,
                        line_top_bound,
                        1,
                        line_height,
                        CURSOR_COLOR
                    );
                }
            }
            }

            if ctype == Block && glyph.char_data.rasterize() {
                canvas.draw_monochrome_image::<MonoImage, u8>(
                    glyph.x as isize,
                    glyph.y as isize,
                    image,
                    CURSOR_COLOR,
                    Rgba::WHITE
                );
            }

            if glyph.parent == '\n' {
                dy += 1;
                dx = 0;
            } else {
                dx += 1;
            }
        }
    }
}

pub fn layout(text: String, font_manager: &FontManager) -> Layout {
    let mut layout = Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);

    let text = TextStyle {
        text: text.as_str(),
        px: font_manager.scale,
        font_index: 0,
        user_data: ()
    };
    layout.append(&[&font_manager.fonts[0]], &text);
    layout
}

pub fn get_image<'a, T: Clone + Copy>(glyph: &GlyphPosition<T>, font_manager: &'a mut FontManager) -> &'a (Metrics, MonoImage) {

    font_manager.cache.entry(glyph.key).or_insert({
        let (metrics, raster) = font_manager.fonts[0].rasterize_indexed(glyph.key.glyph_index, glyph.key.px);

        let new_image = MonoImage {
            bytes: raster,
            width: glyph.width,
            height: glyph.height
        };
        (metrics, new_image)

    })
}
