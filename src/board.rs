use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter, Error};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Piece {
    X,
    O,
}
use self::Piece::*;

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            X => write!(f, "X"),
            O => write!(f, "O"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Board {
    squares: [Option<Piece>; 9],
}

impl Board {
    pub fn new() -> Board {
        Board { squares: [None; 9] }
    }

    pub fn numpad_to_position(key: usize) -> Option<(usize, usize)> {
        match key {
            7 => Some((0, 0)),
            8 => Some((0, 1)),
            9 => Some((0, 2)),
            4 => Some((1, 0)),
            5 => Some((1, 1)),
            6 => Some((1, 2)),
            1 => Some((2, 0)),
            2 => Some((2, 1)),
            3 => Some((2, 2)),
            _ => None,
        }
    }

    pub fn is_full(&self) -> bool {
        self.squares.iter().all(Option::is_some)
    }

    pub fn game_over(&self) -> bool {
        self.winner().is_some() || self.is_full()
    }

    pub fn winner(&self) -> Option<Piece> {
        if let Some(row) = (0..3).find(|&row| {
            self[(row, 0)].is_some() && self[(row, 0)] == self[(row, 1)] &&
            self[(row, 1)] == self[(row, 2)]
        }) {
            self[(row, 0)]
        } else if let Some(col) = (0..3).find(|&col| {
            self[(0, col)].is_some() && self[(0, col)] == self[(1, col)] &&
            self[(1, col)] == self[(2, col)]
        }) {
            self[(0, col)]
        } else if self[(0, 0)].is_some() &&
                  self[(0, 0)] == self[(1, 1)] &&
                  self[(1, 1)] == self[(2, 2)] {
            self[(0, 0)]
        } else if self[(0, 2)].is_some() &&
                  self[(0, 2)] == self[(1, 1)] &&
                  self[(1, 1)] == self[(2, 0)] {
            self[(0, 2)]
        } else {
            None
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
                    // This changes 012 345 678 into 789 456 123 for the numpad buttons
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
