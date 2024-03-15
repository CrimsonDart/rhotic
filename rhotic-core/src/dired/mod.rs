use std::{path::PathBuf, str::FromStr, ffi::OsString, fs::{ReadDir, DirEntry}, io::Error};

use anyhow::bail;
use fontdue::layout::{Layout, TextStyle};


use crate::{buffer::{text_buffer::Page, stage::{Stage, Render, layout, get_image, InputEvent, StateCommand}}, display::{font::FontManager, Rgba, image::MonoImage, event_loop::{Key}}};

mod theme;

pub struct Dired {
    path: PathBuf,
    cursor: usize,
    theme: theme::DiredTheme,
    files: Vec<FileEntry>,
    scroll_top: usize,
    scroll_window_len: usize
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
enum FileType {
    File,
    Dir,
    Symlink,
    BlockedDir,
    BlockedFile,
    Invalid,
    Other,
}

#[derive(Debug, PartialEq, Eq)]
struct FileEntry {
    name: OsString,
    file_type: FileType,
}

impl Dired {
    fn update_files(&mut self) -> bool {

        self.files = match self.path.read_dir() {
            Ok(k) => k.map(|x| { FileEntry::new(x) }).collect(),
            Err(_) => return false
        };

        self.files.sort();

        true
    }
}

impl FileEntry {
    fn new(en: Result<DirEntry, Error>) -> Self {

        let en = match en {
            Ok(k) => k,
            Err(_) => {return Self::default();}
        };

        let name = en.file_name();
        let ft = en.file_type();

        let ft = match ft {
            Ok(k) => {
                if k.is_dir() {
                    FileType::Dir
                } else if k.is_file() {
                    FileType::File
                } else if k.is_symlink() {
                    FileType::Symlink
                } else {
                    FileType::Other
                }
            },
            Err(_) => {
                FileType::Invalid
            }
        };

        Self {
            name,
            file_type: ft
        }
    }

    fn get_text_style(&self, font_manager: &FontManager) -> TextStyle<FileType> {
        TextStyle {
            text: match self.name.to_str() {
                Some(s) => s,
                None => "-- Conversion Error --"
            },
            px: font_manager.scale,
            font_index: 0,
            user_data: self.file_type
        }
    }
}

impl Default for FileEntry {
    fn default() -> Self {
        Self { name: OsString::from("-- No Files found --"), file_type: FileType::Invalid }
    }
}

impl PartialOrd for FileEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.name.partial_cmp(&other.name)
    }
}

impl Ord for FileEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl Stage for Dired {

    fn init(init_args: &[&str]) -> anyhow::Result<Self> {

        let path_buf = if let Some(path) = init_args.get(0) {
            PathBuf::from_str(path)?
        } else {
            bail!("Tried to open Dired without a path. A path is needed!")
        };

        if !path_buf.exists() {
            bail!("Failed to open Dired; Directory does not exist.")
        }

        if !path_buf.is_dir() {
            bail!("Failed to open Dired; Can only open with a Directory path!")
        }

        let mut buf = Self {
            path: path_buf,
            cursor: 0,
            theme: Default::default(),
            files: vec![],
            scroll_top: 0,
            scroll_window_len: 40,
        };

        buf.update_files();

        Ok(buf)
    }

    fn send_event(&mut self, input: InputEvent) -> StateCommand {

        use Key::*;
        use InputEvent::*;

        match input {
            Press(k) | Echo(k) => match k {
                Arrowdown => if self.cursor + 1 != self.files.len() {
                    self.cursor += 1;

                    if self.cursor > self.scroll_top + self.scroll_window_len - 5 {
                        self.scroll_top += 1;
                    }
                },
                Arrowup => if self.cursor != 0 {
                    self.cursor -= 1;

                    if self.cursor < self.scroll_top + 5 {
                        self.scroll_top = self.scroll_top.checked_sub(1).unwrap_or(0);
                    }
                },
                Arrowleft => {
                    if self.path.pop() {
                        self.cursor = 0;
                        self.scroll_top = 0;
                        self.update_files();
                    }
                },
                Arrowright => {


                    let selected = {
                        let mut new_path = self.path.clone();
                        new_path.push(&self.files[self.cursor].name);
                        new_path
                    };

                    if selected.is_dir() {
                        match selected.read_dir() {
                            Ok(_) => {
                                self.cursor = 0;
                                self.scroll_top = 0;
                                self.path = selected;
                                self.update_files();
                            },
                            Err(e) => {
                                return StateCommand::Log(format!("{e}"));
                            }
                        }
                    } else {
                        return StateCommand::Log(String::from("The file {selected} is not a directory."));
                    }
                },
                _ => {}
            },
            Text(_t) => {

            },
            _ => {}
        }
        StateCommand::None
    }

    const NAME: &'static str = "Dired";
}

impl Render<&mut FontManager> for Dired {
    fn render(&self, canvas: &mut crate::display::text_render::Canvas<&winit::window::Window, &winit::window::Window>, v: &mut FontManager) {

        let mut layout: Layout<FileType> = Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);

        for i in self.scroll_top..(self.scroll_top + self.scroll_window_len) {
            if let Some(file) = self.files.get(i) {
                layout.append(v.fonts.as_slice(), &file.get_text_style(v));
                layout.append(v.fonts.as_slice(), &TextStyle { text: "\n", px: v.scale, font_index: 0, user_data: FileType::Other });
            }
        }

        let glyphs = layout.glyphs();
        let (mut gx, mut gy) = (0,0);
        let cursor = self.cursor - self.scroll_top;

        if let Some(lines) = layout.lines() {
            if let Some(line) = lines.get(cursor) {
                canvas.draw_rectangle(
                    0,
                    line.baseline_y as isize - line.max_ascent as isize,
                    canvas.width(),
                    line.max_new_line_size as usize,
                    self.theme.select_color
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
                self.theme.select_color
            } else {
                Rgba::DARK_GRAY
            };

            let (_metrics, image) = get_image(glyph, v);

            canvas.draw_monochrome_image::<MonoImage, u8>(
                glyph.x as isize,
                glyph.y as isize,
                image,
                line_background_color,
                match glyph.user_data {
                    FileType::File => self.theme.file_color,
                    FileType::Dir => self.theme.directory_color,
                    FileType::Symlink => self.theme.symlink_color,
                    FileType::Other => self.theme.error_color,
                    _ => Rgba::BLACK
                }
            );
        }
    }
}
