use std::fmt::{Formatter, Display};
use std::ops::Add;

pub type Pixel = Point<u32>;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Rgba {
    value: u32
}

impl Rgba {
    pub fn new(red: u32, green: u32, blue: u32, alpha: u32) -> Self {
        Self {
            value: blue | (green << 8) | (red << 16) | (alpha << 24)
        }
    }

    pub fn red(&self) -> u32 {
        (self.value & 0x00FF0000 ) >> 16
    }

    pub fn green(&self) -> u32 {
        (self.value & 0x0000FF00) >> 8
    }

    pub fn blue(&self) -> u32 {
        self.value & 0x000000FF
    }

    pub fn alpha(&self) -> u32 {
        (self.value & 0xFF000000) >> 24
    }

    pub fn set_red(&mut self, red: u32) {
        self.value = (self.value & 0xFF00FFFF) | (red << 16)
    }

    pub fn set_green(&mut self, green: u32) {
        self.value = (self.value & 0xFFFF00FF) | (green << 8)
    }

    pub fn set_blue(&mut self, blue: u32) {
        self.value = (self.value & 0xFFFFFF00) | blue
    }

    pub fn set_alpha(&mut self, alpha: u32) {
        self.value = (self.value & 0x00FFFFFF) | (alpha << 24)
    }
}

impl Display for Rgba {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl From<Rgba> for u32 {
    fn from(value: Rgba) -> Self {
        value.value
    }
}

impl From<u32> for Rgba {
    fn from(value: u32) -> Self {
        Self {
            value
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

        let mut index = &mut self.index;
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
    fn test_red() {

        let mut color = Rgba::new(255, 0, 0, 0);

        assert_eq!(color.red(), 255);

        color.set_red(128);
        assert_eq!(color.red(), 128);
    }

    #[test]
    fn test_green() {

        let mut color = Rgba::from(0x0000FF00);

        assert_eq!(color.green(), 255);

        color.set_green(128);
        color.set_red(32);
        assert_eq!(color.green(), 128);
    }
}
