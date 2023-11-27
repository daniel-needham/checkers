use std::marker::PhantomData;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Colour {
    Black,
    White
}

#[derive(Debug, PartialEq)]
pub struct Player{
    pub colour: Colour,
    pub ai: bool,
}