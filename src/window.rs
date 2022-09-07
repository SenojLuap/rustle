/// A render context and/or target. Represents a two dimensional array of characters, with associated functions to manipulate the
/// contents.
pub struct Window<const WIDTH: usize, const HEIGHT: usize> {
    data : [[char; WIDTH]; HEIGHT],
    x_clip : Range<i16>,
    y_clip : Range<i16>,
}


impl<const WIDTH: usize, const HEIGHT: usize> Window<WIDTH, HEIGHT> {
    
    /// Construct a new [`Window`], initialized to empty space
    pub fn new() -> Result<Self, &'static str> {
        let x_clip = 0..i16::try_from(WIDTH).map_err(|_| "Window may be no wider than i32::MAX")?;
        let y_clip = 0..i16::try_from(HEIGHT).map_err(|_| "Window may be no taller than i32::MAX")?;
        Ok(Window{data: [[' '; WIDTH]; HEIGHT], x_clip, y_clip})
    }

    /// Construct a new [`Window`], initialized to the provided character
    pub fn new_with_fill(fill: char) -> Result<Self, &'static str> {
        let x_clip = 0..i16::try_from(WIDTH).map_err(|_| "Window may be no wider than i32::MAX")?;
        let y_clip = 0..i16::try_from(HEIGHT).map_err(|_| "Window may be no taller than i32::MAX")?;
        Ok(Window{data: [[fill; WIDTH]; HEIGHT], x_clip, y_clip})
    }
    
    /// Render the [`Window`]'s current state
    pub fn render(&self) -> [[char; WIDTH]; HEIGHT] {
        self.data.clone()
    }
    
    /// The width of the [`Window`]
    pub const fn width() -> usize {
        WIDTH
    }
    
    /// The height of the [`Window`]
    pub const fn height() -> usize {
        HEIGHT
    }

    /// Generalized algorithm for drawing either a hollow or filled circle
    fn draw_any_circle<DrawFunc>(radius: u16, mut draw_func: DrawFunc) where DrawFunc : FnMut(i16, i16) {
        let radius = i16::try_from(radius).unwrap_or(i16::MAX); // Technically this is undefined/hidden behavior

        let mut d = 3 - (2*radius);
        let mut x = 0;
        let mut y = radius;
        draw_func(x, y);
        while x <= y {
            x += 1;
            if d < 0 {
                d = d + (4*x) + 6;
            } else {
                d = d + 4*(x - y) + 10;
                y -= 1;
            }
            draw_func(x, y);
        }
    }
}


impl<const WIDTH: usize, const HEIGHT: usize> BlitTarget for Window<WIDTH, HEIGHT> {
    /// Draw a single point with the provided brush
    fn draw<Pos: Into<Point>>(&mut self, brush: char, pos: Pos) {
        let pos = pos.into();
        if self.x_clip.contains(&pos.x) && self.y_clip.contains(&pos.y) {
            self.data[usize::try_from(pos.y).unwrap()][usize::try_from(pos.x).unwrap()] = brush;
        }
    }
    
    /// Draw a hollow square with the provided 'brush'
    fn draw_square<Pos: Into<Point>>(&mut self, brush: char, pos: Pos, width: i16, height: i16) {
        let pos = pos.into();
        for x in pos.x..(pos.x+width) {
            self.draw(brush, (x, pos.y));
            self.draw(brush, (x, pos.y+height-1));
        }
        for y in (pos.y+1)..(pos.y+height-1) {
            self.draw(brush, (pos.x, y));
            self.draw(brush, (pos.x+width-1, y));
        }
    }

    /// Draw a filled square with the provided 'brush'
    fn draw_filled_square<Pos: Into<Point>>(&mut self, brush: char, pos: Pos, width: i16, height: i16) {
        let pos = pos.into();
        for y in pos.y..(pos.y+height) {
            for x in pos.x..(pos.x+width) {
                self.draw(brush, (x, y));
            }
        }
    }

    /// Draw a hollow circle with the provided 'brush'
    fn draw_circle<Pos: Into<Point>>(&mut self, brush: char, pos: Pos, radius: u16) {
        let pos = pos.into();

        let draw_func = move |x: i16, y: i16| {
            self.draw(brush, (pos.x+x, pos.y+y));
            self.draw(brush, (pos.x-x, pos.y+y));
            self.draw(brush, (pos.x+x, pos.y-y));
            self.draw(brush, (pos.x-x, pos.y-y));
            self.draw(brush, (pos.x+y, pos.y+x));
            self.draw(brush, (pos.x-y, pos.y+x));
            self.draw(brush, (pos.x+y, pos.y-x));
            self.draw(brush, (pos.x-y, pos.y-x));
        };
        Window::<WIDTH, HEIGHT>::draw_any_circle(radius, draw_func);
    }


    /// Draw a filled circle with the provided 'brush'
    fn draw_filled_circle<Pos: Into<Point>>(&mut self, brush: char, pos: Pos, radius: u16) {
        let pos = pos.into();

        let draw_func = move |x: i16, y: i16| {
            for sub_y in x..y {
                self.draw(brush, (pos.x+x, pos.y+sub_y));
                self.draw(brush, (pos.x-x, pos.y+sub_y));
                self.draw(brush, (pos.x+x, pos.y-sub_y));
                self.draw(brush, (pos.x-x, pos.y-sub_y));
                self.draw(brush, (pos.x+sub_y, pos.y+x));
                self.draw(brush, (pos.x-sub_y, pos.y+x));
                self.draw(brush, (pos.x+sub_y, pos.y-x));
                self.draw(brush, (pos.x-sub_y, pos.y-x));
            }
        };
        Window::<WIDTH, HEIGHT>::draw_any_circle(radius, draw_func);
    }

}

impl<const SRC_WIDTH: usize, const SRC_HEIGHT: usize> Blitable for Window<SRC_WIDTH, SRC_HEIGHT> {
    fn blit<Pos: Into<Point>, Target: BlitTarget>(&self, target: &mut Target, pos: Pos) {
        let pos = pos.into();
        for row in self.data.iter().enumerate() {
            let y = i16::try_from(row.0).unwrap();
            for col in row.1.iter().enumerate() {
                let x = i16::try_from(col.0).unwrap();
                target.draw(*col.1, (x+pos.x, y+pos.y));
            }
        }
    }
}

use std::ops::Range;

use crate::{Point, Blitable, BlitTarget};