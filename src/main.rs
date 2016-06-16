#![feature(range_contains)]

use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter, Error};

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

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        macro_rules! write_n {
            ($n:expr) => {
                try!(match self.squares[$n] {
                    Some(X) => write!(f, " X "),
                    Some(O) => write!(f, " O "),
                    None => write!(f, " {} ", 9 - ($n/3) * 3 - (2 - $n%3)),
                })
            }
        }
        write_n!(0);
        try!(write!(f, "|"));
        write_n!(1);
        try!(write!(f, "|"));
        write_n!(2);
        try!(write!(f, "\n---+---+---\n"));
        write_n!(3);
        try!(write!(f, "|"));
        write_n!(4);
        try!(write!(f, "|"));
        write_n!(5);
        try!(write!(f, "\n---+---+---\n"));
        write_n!(6);
        try!(write!(f, "|"));
        write_n!(7);
        try!(write!(f, "|"));
        write_n!(8);
        Ok(())
    }
}

fn main() {
    let mut board = Board::new();
    board[(1, 1)] = Some(X);
    println!("{}", board);
}
