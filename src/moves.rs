use pieces::*;
use itertools::unfold;

pub fn on_board(&Position(i, j): &Position) -> bool {
    i >= 0 && i < 8 && j >= 0 && j < 8
}

pub fn possible_knight_moves(Position(i, j): Position) -> Vec<Position> {
    [-2, 2].map()
}

fn up_right (&mut Position(i, j): &mut Position) -> Option<Position> { Some(Position(i + 1, j + 1)) }
fn up_left (&mut Position(i, j): &mut Position) -> Option<Position> { Some(Position(i + -1, j + 1)) }
fn down_right (&mut Position(i, j): &mut Position) -> Option<Position> { Some(Position(i + 1, j + -1)) }
fn down_left (&mut Position(i, j): &mut Position) -> Option<Position> { Some(Position(i + -1, j + -1)) }

pub fn possible_bishop_moves(position: Position) -> Vec<Position> {
    let up_right = unfold(position, up_right).take_while(on_board);
    let up_left = unfold(position, up_left).take_while(on_board);
    let down_right = unfold(position, down_right).take_while(on_board);
    let down_left = unfold(position, down_left).take_while(on_board);
    up_right.chain(up_left).chain(down_right).chain(down_left).collect()
}
