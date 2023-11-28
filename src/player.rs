use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colour {
    Black,
    White
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