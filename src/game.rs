use std::fmt;
use pieces::*;
//use moves::*;
use pieces::Shape::*;
use pieces::Color::*;

pub struct Game {
    board: [[Square; 8]; 8],
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.board.len() {
            try!(write!(f, "{:?}\n", self.board[row]))
        }
        write!(f, "\n")
    }
}

pub fn game() -> Game {
    Game {
        board: [
            [sq(Wh, R), sq(Wh, Kn), sq(Wh, B), sq(Wh, Q), sq(Wh, K), sq(Wh, Kn), sq(Wh, Kn), sq(Wh, R)],
            [sq(Wh, P), sq(Wh, P),  sq(Wh, P), sq(Wh, P), sq(Wh, P), sq(Wh, P),  sq(Wh, P),  sq(Wh, P)],
            [None,      None,       None,      None,      None,      None,       None,       None     ],
            [None,      None,       None,      None,      None,      None,       None,       None     ],
            [None,      None,       None,      None,      None,      None,       None,       None     ],
            [None,      None,       None,      None,      None,      None,       None,       None     ],
            [sq(Bl, P), sq(Bl, P),  sq(Bl, P), sq(Bl, P), sq(Bl, P), sq(Bl, P),  sq(Bl, P),  sq(Bl, P)],
            [sq(Bl, R), sq(Bl, Kn), sq(Bl, B), sq(Bl, Q), sq(Bl, K), sq(Bl, Kn), sq(Bl, Kn), sq(Bl, R)]],
    }
}

pub fn put_on_board(game: &mut Game,
                    square: Square,
                    Position(i, j): Position) {
    game.board[i as usize][j as usize] = square;
}

pub fn remove_piece(game: &mut Game,
                    Position(i, j): Position) {
    game.board[i as usize][j as usize] = None;
}

pub fn on_board(i: usize, j: usize) -> bool {
    i < 8 && j < 8
}
