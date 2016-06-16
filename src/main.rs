#![feature(range_contains)]

use std::ops::{Index, IndexMut};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    X,
    O,
}
use Piece::*;

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

impl Index<(usize, usize)> for Board {
    type Output = Option<Piece>;

    fn index(&self, (row, col): (usize, usize)) -> &Option<Piece> {
        if (0..3).contains(row) && (0..3).contains(col) {
            &self.squares[row * 3 + col]
        } else {
            panic!("{},{} out of range for a tic-tac-toe board", row, col)
        }
    }
}

impl IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Option<Piece> {
        if (0..3).contains(row) && (0..3).contains(col) {
            &mut self.squares[row * 3 + col]
        } else {
            panic!("{},{} out of range for a tic-tac-toe board", row, col)
        }
    }
}

fn main() {
    let mut board = Board::new();
    board[(1, 1)] = Some(X);
}
