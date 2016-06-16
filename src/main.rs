#![feature(range_contains)]

use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter, Error};
use std::io;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Piece {
    X,
    O,
}
use Piece::*;

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match *self {
            X => write!(f, "X"),
            O => write!(f, "O"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Board {
    squares: [Option<Piece>; 9],
}

impl Board {
    fn new() -> Board {
        Board {
            squares: [None; 9],
        }
    }

    fn numpad_to_position(key: usize) -> Option<(usize, usize)> {
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
    match game() {
        Ok(()) => (),
        Err(e) => println!("The game failed with a fatal error: {:?}", e),
    }
}

fn game() -> io::Result<()> {
    let mut board = Board::new();
    let mut turn = X;
    let stdin = io::stdin();
    let mut buffer = String::with_capacity(32);
    loop {
        println!("{}'s turn (q to quit):\n{}", turn, board);
        let input = {
            buffer.clear();
            try!(stdin.read_line(&mut buffer));
            buffer.trim()
        };

        if input == "q" { break; }
        match input.parse::<usize>() {
            Ok(n @ 1...9) => {
                if let Some((row, col)) = Board::numpad_to_position(n) {
                    if board[(row, col)].is_none() {
                        board[(row, col)] = Some(turn);
                    } else {
                        println!("That space is already occupied");
                        continue;
                    }
                } else {
                    println!("Enter a number on the board");
                    continue;
                }
            }
            Ok(_) | Err(_) => {
                println!("Enter a number on the board");
                continue;
            }
        }
        turn = match turn { X => O, O => X };
    }
    println!("Goodbye!");
    Ok(())
}
