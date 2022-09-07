/// A [`BlitTarget`] can be blitted to by a [`Blitable`](crate::blitable::Blitable)
pub trait BlitTarget {
    /// Draw a single point with the provided brush
    fn draw<Pos: Into<Point>>(&mut self, brush: char, pos: Pos);
    
    /// Draw a hollow square with the provided 'brush'
    fn draw_square<Pos: Into<Point>>(&mut self, brush: char, pos: Pos, width: i16, height: i16);

    /// Draw a filled square with the provided 'brush'
    fn draw_filled_square<Pos: Into<Point>>(&mut self, brush: char, pos: Pos, width: i16, height: i16);
    
    /// Draw a hollow circle with the provided 'brush'
    fn draw_circle<Pos: Into<Point>>(&mut self, brush: char, pos: Pos, radius: u16);
    
    /// Draw a filled circle with the provided 'brush'
    fn draw_filled_circle<Pos: Into<Point>>(&mut self, brush: char, pos: Pos, radius: u16);
}

use crate::Point;