#![feature(range_contains)]

use std::ops::{Index, IndexMut};
use std::fmt::{Display, Formatter, Error};
use std::ascii::AsciiExt;
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
        Board { squares: [None; 9] }
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

    fn is_full(&self) -> bool {
        self.squares.iter().all(Option::is_some)
    }

    fn game_over(&self) -> bool {
        self.winner().is_some() || self.is_full()
    }

    fn winner(&self) -> Option<Piece> {
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

fn main() {
    match menu() {
        Ok((x, o, cat)) => println!("Final scores: X: {} O: {} Cat: {}", x, o, cat),
        Err(e) => println!("The game failed with a fatal error: {:?}", e),
    }
}

fn menu() -> io::Result<(u32, u32, u32)> {
    game()
}

fn prompt_confirm(msg: &str) -> bool {
    let mut buffer = String::with_capacity(16);
    loop {
        println!("{}", msg);
        let input = {
            match io::stdin().read_line(&mut buffer) {
                Ok(_) => (),
                Err(_) => return true,
            }
            buffer.trim()
        };

        if input.eq_ignore_ascii_case("y") || input.eq_ignore_ascii_case("yes") || input == "" {
            return true;
        } else if input.eq_ignore_ascii_case("n") || input.eq_ignore_ascii_case("no") {
            return false;
        } else {
            println!("Please answer y or n");
        }
    }
}

fn game() -> io::Result<(u32, u32, u32)> {
    let stdin = io::stdin();
    let mut buffer = String::with_capacity(32);
    let (mut x, mut o, mut cat) = (0, 0, 0);
    'game: loop {
        let mut board = Board::new();
        let mut turn = X;

        loop {
            println!("{}'s turn (q to quit):\n{}", turn, board);
            let input = {
                buffer.clear();
                try!(stdin.read_line(&mut buffer));
                buffer.trim()
            };

            if input == "q" {
                if prompt_confirm("Are you sure you want to quit? [Y/n]") {
                    break 'game;
                } else {
                    continue;
                }
            }

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

            if board.game_over() {
                match board.winner() {
                    Some(X) => {
                        println!("Xs win!");
                        x += 1;
                    }
                    Some(O) => {
                        println!("Os win!");
                        o += 1;
                    }
                    None => {
                        println!("It's the cat's game!");
                        cat += 1;
                    }
                }
                break;
            }

            turn = match turn {
                X => O,
                O => X,
            };
        }

        if !prompt_confirm("Play again? [Y/n]") {
            break 'game;
        }
    }
    println!("Goodbye!");
    Ok((x, o, cat))
}
