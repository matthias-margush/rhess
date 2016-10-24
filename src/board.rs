use std::fmt;

pub struct Game {
    board: [[i32; 8]; 8],
}
pub fn board() -> Game {
    Game { board: [[0; 8]; 8] }
}

impl fmt::Debug for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.board.len() {
            // this is equivalent:
            // try!(write!(f, "{:?}\n", self.board[row]))

            let res = write!(f, "{:?}\n", self.board[row]);
            match res {
                Result::Ok(val) => val,
                Result::Err(_) => return res,
            }
        }
        write!(f, "\n")
    }
}
