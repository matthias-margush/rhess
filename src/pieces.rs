#[derive(Copy, Clone, Debug)]
pub enum Color { Wh, Bl }

#[derive(Copy, Clone, Debug)]
pub enum Shape { R, Kn, B, K, Q, P }

#[derive(Copy, Clone, Debug)]
pub struct Piece {
    pub color: Color,
    pub shape: Shape
}

pub type Square = Option<Piece>;

#[derive(Copy, Clone, Debug)]
pub struct Position(pub i8, pub i8);

pub fn sq(color: Color, shape: Shape) -> Square {
    Some(Piece{ color: color, shape: shape })
}
