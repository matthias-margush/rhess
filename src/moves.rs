use pieces::*;
use std::iter::Iterator;

pub fn on_board(&Position(i, j): &Position) -> bool {
    i >= 0 && i < 8 && j >= 0 && j < 8
}

type Move = fn (Position) -> Position;

pub struct Movement {
    position: Position,
    next: Move,
}

impl Iterator for Movement {
    type Item = Position;
    fn next(&mut self) -> Option<Position> {
        let &mut Movement{mut position, next} = self;
        position = next(position);
        Some(position)
    }
}

impl Position {
    pub fn sweep(&self, mov: Move) -> Movement {
        Movement {
            position: self.clone(),
            next: mov,
        }
    }
}

fn up_right (Position(i, j): Position) -> Position { Position(i + 1, j + 1) }
fn up_left (Position(i, j): Position) -> Position { Position(i + -1, j + 1) }
fn down_right (Position(i, j): Position) -> Position { Position(i + 1, j + -1) }
fn down_left (Position(i, j): Position) -> Position { Position(i + -1, j + -1) }

pub fn possible_bishop_moves(position: Position) -> Vec<Position> {
    let up_right = position.sweep(up_right).take_while(on_board);
    let up_left = position.sweep(up_left).take_while(on_board);
    let down_right = position.sweep(down_right).take_while(on_board);
    let down_left = position.sweep(down_left).take_while(on_board);
    up_right.chain(up_left).chain(down_right).chain(down_left).collect()
}
