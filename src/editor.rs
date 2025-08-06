use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};

mod terminal;
use terminal::Terminal;

use crate::editor::terminal::Coordinate;

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }

    fn repl(&mut self) -> Result<(), std::io::Error> {
        loop {
            self.refresh_screen()?;

            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(()) // if we get to this line then there should be no error so we return
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            // println!("{code:?} \r");
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
                // }
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print("\r\nGoodbye!\r")?;
            Terminal::execute()?;
        } else {
            Self::draw_rows()?;
            Self::print_welcome()?;
            Terminal::move_cursor(Coordinate { x: 0, y: 0 })?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let row: u16 = Terminal::size()?.rows;
        for r in 0..row {
            Terminal::clear_line()?;
            if r == 0 {
                Terminal::print(" \r")?;
            } else {
                Terminal::print("~\r")?;
            }
            if r + 1 < row {
                Terminal::print("\r\n")?;
            }
        }
        Ok(())
    }

    pub fn print_welcome() -> Result<(), std::io::Error> {
        let y_down: u16 = (Terminal::size()?.rows) / 3;
        let width: u16 = Terminal::size()?.columns;

        let name = NAME.to_string();
        let version = format!("version {VERSION}");

        // let welcome_message = format!("{NAME} v{VERSION}");
        let mut message_len = name.len() as u16;
        let mut x_padding = (width - message_len) / 2;

        Terminal::print_at(
            &name,
            Coordinate {
                x: x_padding,
                y: y_down,
            },
        )?;

        message_len = version.len() as u16;
        x_padding = (width - message_len) / 2;
        Terminal::print_at(
            &version,
            Coordinate {
                x: x_padding,
                y: y_down + 1,
            },
        )?;

        Ok(())
    }
}
