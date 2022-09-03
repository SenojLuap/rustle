#[derive(Default)]
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

