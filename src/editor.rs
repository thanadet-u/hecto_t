use crossterm::event::{Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read}; // need these to check if ctrl is being pressed
use crossterm::terminal::{disable_raw_mode, enable_raw_mode}; // using crossterm to read and deal with events

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?} \r");
        }
        println!("Goodbye! \r\n");
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        // return Ok box with nothing in it, or return error
        enable_raw_mode()?; // ? unwrap the result, if its an error it return the error immediately
        loop {
            if let Key(KeyEvent {
                code,
                modifiers,
                kind,
                state,
            }) = read()?
            {
                println!(
                    "Code {code:?}, Modifiers {modifiers:?}, Kind {kind:?}, State {state:?} \r"
                );
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true;
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?;
        Ok(()) // if we get to this line then there should be no error so we return
    }
}
