use crossterm::event::{Event::Key, KeyCode::Char, read};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode}; // using crossterm to read and deal with events

pub struct Editor {}

impl Editor {
    pub fn default() -> Self {
        Editor {}
    }

    pub fn run(&self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?} \r");
        }
        println!("Goodbye! \r\n");
    }

    fn repl(&self) -> Result<(), std::io::Error> {
        // return Ok box with nothing in it, or return error
        enable_raw_mode()?; // ? unwrap the result, if its an error it return the error immediately
        loop {
            if let Key(event) = read()? {
                println!("{event:?} \r");
                if let Char(c) = event.code {
                    if c == 'q' {
                        break;
                    }
                }
            }
        }
        disable_raw_mode()?;
        Ok(()) // if we get to this line then there should be no error so we return
    }
}
