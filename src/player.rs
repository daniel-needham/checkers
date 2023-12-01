use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colour {
    Black,
    White
}

impl fmt::Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Colour::Black => write!(f, "Black"),
            Colour::White => write!(f, "White"),
        }
    }
}

impl Colour {
    pub fn other(&self) -> Colour {
        match self {
            Colour::Black => Colour::White,
            Colour::White => Colour::Black,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Player{
    pub colour: Colour,
    pub ai: bool,
}