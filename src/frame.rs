pub struct Frame {
    frame_type: FrameType,
    pub width: u16,
    pub height: u16,
}

pub enum FrameType {
    Light,
    Double
}

impl Frame {

    const TOP_LEFT : usize = 0;
    const TOP_RIGHT : usize = 1;
    const BOTTOM_LEFT : usize = 2;
    const BOTTOM_RIGHT : usize = 3;
    const TOP_BOTTOM : usize = 4;
    const LEFT_RIGHT : usize = 5;

    pub fn new(frame_type: FrameType, width: u16, height: u16) -> Self {
        Frame{frame_type, width, height}
    }
}

impl Blitable for Frame {
    fn blit<Pos: Into<Point>, Target: BlitTarget>(&self, target: &mut Target, pos: Pos) {
        let charset = match self.frame_type {
            FrameType::Light => ['┌', '┐', '└', '┘', '─', '│'],
            FrameType::Double => ['╔', '╗', '╚', '╝', '═', '║']
        };
        let pos = pos.into();
        let width = i16::try_from(self.width).unwrap();
        let height = i16::try_from(self.height).unwrap();
        target.draw(charset[Frame::TOP_LEFT], pos);
        target.draw(charset[Frame::TOP_RIGHT], pos + (width-1, 0));
        target.draw(charset[Frame::BOTTOM_LEFT], pos + (0, height-1));
        target.draw(charset[Frame::BOTTOM_RIGHT], pos + (width-1, height-1));
        for x in 1..width-1 {
            target.draw(charset[Frame::TOP_BOTTOM], pos + (x, 0));
            target.draw(charset[Frame::TOP_BOTTOM], pos + (x, height-1));
        }
        for y in 1..height-1 {
            target.draw(charset[Frame::LEFT_RIGHT], pos + (0, y));
            target.draw(charset[Frame::LEFT_RIGHT], pos + (width-1, y));
        }
    }
}

use crate::{Blitable, Point, Canvas, BlitTarget};