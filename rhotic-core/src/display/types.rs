use std::error::Error;
use std::fmt::{Formatter, Display};
use std::num::TryFromIntError;
use std::ops::{Add, Index, IndexMut};
use std::str::Chars;


use toml::Value;


pub type Pixel = Point<u32>;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red = 2,
    Green = 1,
    Blue = 0,
    Alpha = 3
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Rgba {
    value: [u8; 4]
}

impl Rgba {

    pub const BLACK: Rgba = Rgba::new_opaque(0, 0, 0);
    pub const WHITE: Rgba = Rgba::new_opaque(255, 255, 255);

    pub const GRAY: Rgba = Rgba::new_opaque(0xAA, 0xAA, 0xAA);
    pub const DARK_GRAY: Rgba = Rgba::new_opaque(55, 55, 55);

    // Primary Additive Colors
    pub const RED: Rgba = Rgba::new_opaque(255, 0, 0);
    pub const GREEN: Rgba = Rgba::new_opaque(0, 255, 0);
    pub const BLUE: Rgba = Rgba::new_opaque(0, 0, 255);

    // Primary Subtractive Colors
    pub const MAGENTA: Rgba = Rgba::new_opaque(255, 0, 255);
    pub const CYAN: Rgba = Rgba::new_opaque(0, 255, 255);
    pub const YELLOW: Rgba = Rgba::new_opaque(255, 255, 0);

    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            value: [blue, green, red, alpha]
        }
    }

    pub const fn new_opaque(red: u8, green: u8, blue: u8) -> Self {
        Self {
            value: [blue, green, red, 255]
        }
    }

    pub fn blend(mut self, rhs: Self, proportion: u8) -> Self {
        use Color::*;

        self[Red] = blend_color(self[Red], rhs[Red], proportion);
        self[Green] = blend_color(self[Green], rhs[Green], proportion);
        self[Blue] = blend_color(self[Blue], rhs[Blue], proportion);

        self
    }
}

fn blend_color(a: u8, b: u8, t: u8) -> u8 {

    let (a, b, t) = (a as u16, b as u16, t as u16);
    (((b * t) + (a * (255 - t)) + 1) >> 8) as u8
}

impl Index<Color> for Rgba {
    type Output = u8;
    fn index(&self, index: Color) -> &Self::Output {
        &self.value[index as usize]
    }
}

impl IndexMut<Color> for Rgba {
    fn index_mut(&mut self, index: Color) -> &mut Self::Output {
        &mut self.value[index as usize]
    }
}

impl Index<usize> for Rgba {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.value[index]
    }
}

impl IndexMut<usize> for Rgba {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.value[index]
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut is_first = true;
        for b in self.value {
            if is_first {
                is_first = false;
                write!(f, "{b}")?;
            } else {
                write!(f, ", {b}")?;
            }
        }
        Ok(())
    }
}

impl From<Rgba> for u32 {
    fn from(value: Rgba) -> Self {
        let r = &value as *const Rgba;
        let r: *const u32 = r.cast();
        *unsafe {
            r.as_ref().unwrap()
        }
    }
}

impl From<u32> for Rgba {
    fn from(value: u32) -> Self {
        let r = &value as *const u32;
        let r: *const Rgba = r.cast();
        *unsafe {
            r.as_ref().unwrap()
        }
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self::new(0, 0, 0, 255)
    }
}

impl TryFrom<Value> for Rgba {
    type Error = TomlToRgbaError;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let out = match value {
            Value::String(s) => Rgba::try_from(s.chars())?,
            Value::Array(a) => {
                let mut color: [u8; 4] = [0,0,0,255];
                let mut iter = a.into_iter();
                for i in 0..4 {
                    if let Some(Value::Integer(int)) = iter.next() {
                        color[i] = int.try_into()?;
                    } else if i < 3 {
                        return Err(TomlToRgbaError::IncorrectArrayType(i));
                    }
                }

                Rgba::new(color[0], color[1], color[2], color[3])
            },
            _ => {
                return Err(TomlToRgbaError::InvalidEntryType);
            }
        };
        Ok(out)
    }
}

pub enum TomlToRgbaError {
    InsufficientStrLen(usize),
    InvalidStr(String),
    IncorrectArrayType(usize),
    InvalidEntryType,
    IntConversionFail
}

impl From<TryFromIntError> for TomlToRgbaError {
    fn from(_value: TryFromIntError) -> Self {
        TomlToRgbaError::IntConversionFail
    }
}

impl From<CharsToRgbaError> for TomlToRgbaError {
    fn from(value: CharsToRgbaError) -> Self {
        match value {
            CharsToRgbaError::InvalidStr(s) => Self::InvalidStr(s),
            CharsToRgbaError::InsufficientLength(len) => Self::InsufficientStrLen(len)
        }
    }
}

impl<'a> TryFrom<Chars<'a>> for Rgba {
    type Error = CharsToRgbaError;

    fn try_from(mut value: Chars) -> Result<Self, Self::Error> {
        let num = {
            let mut out = [0; 4];
            for i in 0..4 {

                let (left, right) = (value.next(), value.next());


                if let (Some(n1), Some(n2)) = (left, right) {
                    if let Some(n) = hex_code_to_u8([n1, n2]) {
                        out[i] = n;
                    } else if i != 3 {
                        return Err(CharsToRgbaError::InvalidStr({
                            let mut s = String::new();
                            s.push(n1);
                            s.push(n2);
                            s
                        }
                        ));
                    } else {
                        out[3] = 0xFF;
                    }
                } else if i != 3 {
                    let len = if left.is_some() {i * 2 + 1} else { i * 2 };

                    return Err(CharsToRgbaError::InsufficientLength(len));
                } else {
                    out[3] = 0xFF;
                }
            }
            out
        };

        Ok(Self::new(num[0], num[1], num[2], num[3]))
    }
}

fn hex_code_to_u4(c: char) -> Option<u8> {
    Some(match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'A' | 'a' => 10,
        'B' | 'b' => 11,
        'C' | 'c' => 12,
        'D' | 'd' => 13,
        'E' | 'e' => 14,
        'F' | 'f' => 15,
        _ => {
            return None;
        }
    })
}

fn hex_code_to_u8(chars: [char; 2]) -> Option<u8> {
    Some(hex_code_to_u4(chars[0])? * 16 + hex_code_to_u4(chars[1])?)
}

#[derive(Debug)]
pub enum CharsToRgbaError {
    InvalidStr(String),
    InsufficientLength(usize)
}

impl Display for CharsToRgbaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use CharsToRgbaError::*;

        match self {
            InsufficientLength(n) => {
                write!(f, "Input iterator length was insufficient. Len was {n}, len requred is 6 or 8.")
            },
            InvalidStr(s) => {
                write!(f, "{s} contians characters that are not compatiable with the hex codec, which is 0-9, or A-F.")
            }
        }
    }
}

impl Error for CharsToRgbaError {}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T
}

impl<T> Point<T> {

    #[inline]
    pub fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}

impl DisplaySized for Point<u32> {
    #[inline]
    fn get_size(&self) -> Point<u32> {
        self.clone()
    }
}

impl<T: Add<Output = T>> Add for Point<T> {

    type Output = Point<T>;

    fn add(self, rhs: Self) -> Point<T> {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

pub trait DisplaySized {
    fn get_size(&self) -> Point<u32>;

    #[inline]
    fn get_area(&self) -> u32 {
        let number = self.get_size();
        number.x * number.y
    }

    #[inline]
    fn iter(&self) -> AreaIter {
        AreaIter { size: self.get_size(), index: Point { x: 0, y: 0 } }
    }
}

pub struct AreaIter {
    size: Point<u32>,
    index: Point<u32>
}

impl Iterator for AreaIter {

    type Item = Point<u32>;

    fn next(&mut self) -> Option<Self::Item> {


        let index = &mut self.index;
        let size = &self.size;

        if index.y > size.y {
            return None;
        }

        let out = Some(*index);

        index.x = index.x + 1;
        if index.x > size.x {
            index.x = 0;
            index.y = index.y + 1;
        }

        out
    }
}

pub trait DisplayPosition {
    fn get_position(&self) -> Point<u32>;
}

pub trait DisplayRect where Self: DisplayPosition + DisplaySized {
    fn iter(&self) -> RectIter {
        RectIter {
            offset: self.get_position(),
            area_iter: AreaIter {
                size: self.get_size(),
                index: Point::new(0,0)
            }
        }
    }

    fn contains(&self, pixel: Pixel) -> bool {

        let position = self.get_position();
        let size = self.get_size() + position;

        pixel.x >= position.x &&
        pixel.y >= position.x &&
        pixel.x <= size.x &&
        pixel.y <= size.y
    }

    /// Translates a Local Coordinate (local: Pixel) to be relative to the parent.
    /// This translation assumes a few things:
    /// the self.get_position() is relative to 0,0 of the parent.
    #[inline]
    fn map_to_parent<P>(&self, _parent: &P, local: Pixel) -> Pixel where P: DisplaySized {
        local + self.get_position()
    }

    fn index_map_to_parent<P>(&self, parent: &P, local: u32) -> Option<u32> where P: DisplaySized {
        let local = Pixel::new(
            local as u32 / self.get_size().x,
            local as u32 % self.get_size().x
        );

        let global_pixel = self.map_to_parent(parent, local);

        if parent.get_size().y <= global_pixel.y {
            return None;
        }

        let global = parent.get_size().x * global_pixel.x + global_pixel.y;
        Some(global)
    }




}

impl<T> DisplayRect for T where T: DisplayPosition + DisplaySized {}



pub struct RectIter  {
    offset: Point<u32>,
    area_iter: AreaIter
}

impl Iterator for RectIter {

    type Item = Point<u32>;
    fn next(&mut self) -> Option<Self::Item> {

        let out = self.area_iter.next();
        match out {
            Some(p) => {
                Some(p + self.offset)
            },
            None => None
        }
    }
}
