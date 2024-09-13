use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Side {
    Left,
    Right,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Side::Left => write!(f, "Left"),
            Side::Right => write!(f, "Right"),
        }
    }
}
