use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, Read};

fn main() {
    enable_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        // TODO: https://github.com/pflenker/hecto-tutorial/commit/23f1c0f6c54d794a8794fa40432c8e52103cc188

        match b {
            Ok(b) => {
                // if b has no error assign it to b. (b is ok)

                let c = b as char; // 'as' is type casting

                if c.is_control() {
                    println!("Binary: {0:08b} ASCII: {0:#03} \r", b);
                } else {
                    println!("Binary: {0:08b} ASCII: {0:#03} Character: {1:#?}\r", b, c);
                }

                // when read q, exits
                if c == 'q' {
                    break;
                }
            }
            Err(err) => {
                println!("Error: {}", err);
            } // if b can unwrap then unwrap err and print
        }
    }
    disable_raw_mode().unwrap(); // always disable raw mode
}
