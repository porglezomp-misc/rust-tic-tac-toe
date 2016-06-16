#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    X,
    O,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Board {
    squares: [Option<Piece>; 9],
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [None; 9],
        }
    }
}

fn main() {
    let board = Board::new();
    let _ = board;
}
