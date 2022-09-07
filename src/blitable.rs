/// A type that may be 'blitted' to a [`Window`].
/// 
/// 'Blitting' involves drawing the contents of a type to a [`Window`]. Note that [`Window`]s themselves implement [`Blitable`], so you can, in fact,
/// blit a [`Window`] to another [`Window`]. This is useful to create 'sub windows'.
pub trait Blitable {
    /// Blit to the specified [`Window`] at an offset from the origin.
    fn blit<const WIDTH: usize, const HEIGHT: usize, Pos: Into<Point>>(&self, target: &mut Window::<WIDTH, HEIGHT>, pos: Pos);
}

use crate::{Window, Point};