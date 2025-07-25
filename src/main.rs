use std::io::{self, Read};

fn main() {
    /*
       this programs reads from the standard input stream, then for each character(in this case only 1 byte) in it(b)
       its ouput in a newline

       note: the keyboard input is only sent when you press enter. canonical mode. not ideal for a text editor
    */

    for b in io::stdin().bytes() {
        let c = b.unwrap() as char;

        // when press read q, exits
        if c == 'q' {
            break;
        }

        println!("{}", c);
    }

    // TODO: we want the program to read the input immediately.
}
