#![feature(range_contains)]

use std::io;

mod board;
mod game;

fn main() {
    match menu() {
        Ok((x, o, cat)) => {
            println!("Final scores:
  X: {}
  O: {}
Cat: {}",
                     x,
                     o,
                     cat)
        }
        Err(e) => println!("The game failed with a fatal error: {:?}", e),
    }
}

fn menu() -> io::Result<(u32, u32, u32)> {
    let mut buffer = String::with_capacity(32);
    loop {
        println!("Choose a mode:
1) local
2) host
3) join"
        );

        let input = {
            try!(io::stdin().read_line(&mut buffer));
            buffer.trim().to_lowercase()
        };

        match &input[..] {
            "1" | "l" | "local" => return game::local(),
            "2" | "h" | "host" => return game::host(),
            "3" | "j" | "join" => return game::join(),
            _ => (),
        }
    }
}
