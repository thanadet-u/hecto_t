use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

fn main() {
    // so we use a library, crossterm to enter raw mode

    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;

        // when read q, exits
        if c == 'q' {
            // we only disable raw mode if we exit using q...
            disable_raw_mode().unwrap();
            break;
        }

        println!("{}", c);
    }
}
