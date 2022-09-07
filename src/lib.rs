mod blit_target;
mod blitable;
mod frame;
mod point;
mod window;

pub use blitable::Blitable;
pub use blit_target::BlitTarget;
pub use frame::{Frame, FrameType};
pub use point::Point;
pub use window::Window;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
