pub struct Size {
    pub width : u16,
    pub height : u16,
}

impl Size {
    pub fn new(width: u16, height: u16) -> Self {
        Size{width, height}
    }
}