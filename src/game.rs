use std::io;
use std::ascii::AsciiExt;
use std::net::{TcpListener, TcpStream};
use std::io::Write;

use board::{Board, Piece};
use board::Piece::*;
use board::InsertError::*;

pub fn local() -> io::Result<(u32, u32, u32)> {
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
                Ok(n) => {
                    match board.numpad_insert(n, turn) {
                        Ok(_) => (),
                        Err(AlreadyOccupied) => {
                            println!("That space is already occupied");
                            continue;
                        }
                        Err(InvalidButton) => {
                            println!("Enter a number on the board");
                            continue;
                        }
                    }
                }
                Err(_) => {
                    println!("Enter a number");
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

pub fn host() -> io::Result<(u32, u32, u32)> {
    let server = TcpListener::bind("127.0.0.1:12345").unwrap();
    println!("Server started at {}, waiting for connections.", server.local_addr().unwrap());
    let (connection, _) = try!(server.accept());
    network_game(X, connection)
}

pub fn join() -> io::Result<(u32, u32, u32)> {
    print!("Server: ");
    try!(io::stdout().flush());
    let mut input = String::with_capacity(32);
    try!(io::stdin().read_line(&mut input));
    let connection = try!(TcpStream::connect(input.trim()));
    network_game(O, connection)
}

fn network_game(player: Piece, _connection: TcpStream) -> io::Result<(u32, u32, u32)> {
    match player {
        X => Ok((1, 0, 0)),
        O => Ok((0, 1, 0)),
    }
}

fn prompt_confirm(msg: &str) -> bool {
    let mut buffer = String::with_capacity(32);
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
