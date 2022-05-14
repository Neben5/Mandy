pub struct Coordinates {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for Coordinates {
    fn from((a, b): (usize, usize)) -> Self {
        Self { x: a, y: b }
    }
}
impl From<(u16, u16)> for Coordinates {
    fn from((a, b): (u16, u16)) -> Self {
        Self {
            x: a as usize,
            y: b as usize,
        }
    }
}
