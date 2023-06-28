use std::array::IntoIter;
use std::fmt::{Formatter, Display};
use std::ops::{Add, Range, Mul};

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
    pub fn new(x: T, y: T) -> Self {
        Self {x, y}
    }

    pub fn get_size(&self) -> T
        where T: Mul<Output = T> + Copy {

        self.x * self.y

    }
}

impl<T: Add<Output = T>> Add for Point<T> {

    type Output = Point<T>;

    fn add(self, rhs: Self) -> Point<T> {
        Point::new(self.x + rhs.x, self.y + rhs.y)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Rect {
    pub offset: Point<u32>,
    pub size: Point<u32>
}

impl Rect {
    pub fn new(ox: u32, oy: u32, sx: u32, sy: u32) -> Self {
        Self {
            offset: Point::new(ox, oy),
            size: Point::new(sx, sy)
        }
    }
}


impl IntoIterator for Rect {

    type IntoIter = RectIter;
    type Item = Point<u32>;

    fn into_iter(self) -> Self::IntoIter {
        RectIter {rect: self, index: Point::new(0,0)}
    }
}

pub struct RectIter  {
    rect: Rect,
    index: Point<u32>
}

impl Iterator for RectIter {

    type Item = Point<u32>;


    fn next(&mut self) -> Option<Self::Item> {

        let mut index = &mut self.index;
        let size = &self.rect.size;
        let offset = &self.rect.offset;


        // this means that the next iteration will be outside the range of the Rect.

        if index.y > size.y {
            return None;
        }

        let out = Some(*index + *offset);

        index.x = index.x + 1;
        if index.x > size.x {
            index.x = 0;
            index.y = index.y + 1;
        }

        out
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
