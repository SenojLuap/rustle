
#[derive(Default, Clone, Copy)]
/// A two dimensional coordinate.
pub struct Point {
    pub x : i16,
    pub y : i16,
}

impl Point {
    
    const ORIGIN: Point = Point{x: 0_i16, y: 0_i16};
    
    /// Construct a new [`Point`]
    pub fn new(x: i16, y: i16) -> Self {
        Point{x, y}
    }
}

impl From<(i16, i16)> for Point {
    fn from(tupl: (i16, i16)) -> Self {
        Point{x: tupl.0, y: tupl.1}
    }
}

impl Add<Point> for Point {
    type Output = Self;
    
    fn add(self, rhs: Point) -> Self::Output {
        Point{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl Add<(i16, i16)> for Point {
    type Output = Self;

    fn add(self, rhs: (i16, i16)) -> Self::Output {
        Point{x: self.x + rhs.0, y: self.y + rhs.1}
    }
}

use std::ops::Add;