pub fn main() {
    let mut res = Window::<100, 50>::new_with_fill(LIGHT_SHADE).unwrap();
    res.draw_square(PLUS_SIGN, (1, 1), 5, 5);
    res.draw_filled_square(MEDIUM_SHADE, (3, 3), 5, 5);

    res.draw_filled_circle(DARK_SHADE, (20, 20), 23);
    
    let sub_res = Window::<10, 2>::new_with_fill(GREEK_SMALL_LETTER_PI).unwrap();
    sub_res.blit(&mut res, (30, 2));

    let frame = Frame::new(FrameType::Double, 7, 17);
    frame.blit(&mut res, (20, 20));

    for row in res.render().into_iter() {
        println!("{}", String::from_iter(row.into_iter()));
    }
}

use cp437_constants::*;

use rustle::{Window, Blitable, Frame, FrameType};
