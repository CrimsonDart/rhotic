use std::fmt::{Formatter, Display};
use std::ops::{Add, Index, IndexMut, Sub, Mul, Div};

pub type Pixel = Point<u32>;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    Red = 0,
    Green = 1,
    Blue = 2,
    Alpha = 3
}

pub enum Rgbi {
    Black,
    DarkGray,
    LightGray,
    White,
    DarkBlue,
    Blue,
    DarkGreen,
    Green,
    DarkCyan,
    Cyan,
    DarkRed,
    Red,
    DarkMagenta,
    Magenta,
    DarkYellow,
    Yellow,
}

pub enum MCWool {
    Black,
    DarkGray,
    Gray,
    White,
    Brown,
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    Cyan,
    LightBlue,
    Blue,
    Purple,
    Magenta,
    Pink
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Rgba {
    value: [u8; 4]
}

impl Rgba {

    pub const BLACK: Rgba = Rgba::new(0, 0, 0, 255);
    pub const WHITE: Rgba = Rgba::new_opaque(255, 255, 255);




    pub const fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            value: [red, green, blue, alpha]
        }
    }

    pub const fn new_opaque(red: u8, green: u8, blue: u8) -> Self {
        Self {
            value: [red, green, blue, 255]
        }
    }
}

impl Add for Rgba {
    type Output = Rgba;
    fn add(mut self, rhs: Self) -> Self::Output {
        self[0] = self[0].checked_add(rhs[0]).unwrap_or(255);
        self[1] = self[1].checked_add(rhs[1]).unwrap_or(255);
        self[2] = self[2].checked_add(rhs[2]).unwrap_or(255);
        self[3] = self[3].checked_add(rhs[3]).unwrap_or(255);

        self
    }
}

impl Add<u8> for Rgba {
    type Output = Rgba;
    fn add(mut self, rhs: u8) -> Self::Output {
        self[0] = self[0].checked_add(rhs).unwrap_or(255);
        self[1] = self[1].checked_add(rhs).unwrap_or(255);
        self[2] = self[2].checked_add(rhs).unwrap_or(255);
        self[3] = self[3].checked_add(rhs).unwrap_or(255);

        self
    }
}

impl Sub for Rgba {
    type Output = Rgba;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self[0] = self[0].checked_sub(rhs[0]).unwrap_or(0);
        self[1] = self[1].checked_sub(rhs[1]).unwrap_or(0);
        self[2] = self[2].checked_sub(rhs[2]).unwrap_or(0);
        self[3] = self[3].checked_sub(rhs[3]).unwrap_or(0);

        self
    }
}

impl Sub<u8> for Rgba {
    type Output = Rgba;
    fn sub(mut self, rhs: u8) -> Self::Output {
        self[0] = self[0].checked_sub(rhs).unwrap_or(0);
        self[1] = self[1].checked_sub(rhs).unwrap_or(0);
        self[2] = self[2].checked_sub(rhs).unwrap_or(0);
        self[3] = self[3].checked_sub(rhs).unwrap_or(0);

        self
    }
}

impl Mul<u8> for Rgba {
    type Output = Rgba;
    fn mul(mut self, rhs: u8) -> Self::Output {
        self[0] = self[0].checked_mul(rhs).unwrap_or(255);
        self[1] = self[1].checked_mul(rhs).unwrap_or(255);
        self[2] = self[2].checked_mul(rhs).unwrap_or(255);
        self[3] = self[3].checked_mul(rhs).unwrap_or(255);

        self
    }
}

impl Div<u8> for Rgba {
    type Output = Rgba;
    fn div(mut self, rhs: u8) -> Self::Output {

        if rhs == 0 {
            panic!("Tried to divide by Zero.");
        }

        self[0] /= rhs;
        self[1] /= rhs;
        self[2] /= rhs;
        self[3] /= rhs;

        self
    }
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
    fn map_to_parent<P>(&self, parent: &P, local: Pixel) -> Pixel where P: DisplaySized {
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

#[cfg(test)]
mod test {
    use super::Rgba;


    #[test]
    fn test_array_cast() {
        let color = Rgba::new(0, 1, 2, 3);

        let refer = color.value;
        assert_eq!(refer, [0,1,2,3])


    }
}
