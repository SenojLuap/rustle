pub trait Blitable {
    fn blit<const WIDTH: usize, const HEIGHT: usize, Pos: Into<Point>>(&self, target: &mut Window::<WIDTH, HEIGHT>, pos: Pos);
}

use crate::{Window, Point};