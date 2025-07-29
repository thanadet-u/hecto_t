use crossterm::cursor::MoveTo;
use crossterm::event::{Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers, read};
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{ExecutableCommand, execute};
use std::io::stdout;
use std::io::{self, Write};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {
        Editor { should_quit: false }
    }

    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?; // ? unwrap the result, if its an error it return the error immediately
        let res = Self::clear_screen();
        let _ = Self::draw_rows();
        res
    }
    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()
    }
    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
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
            }
        }
    }
    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Self::clear_screen()?;
            println!("\r\nGoodbye!\r");
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error> {
        let mut out = io::stdout();
        // let size = size();
        // let row: u16 = size.0;
        // let col: u16 = size.1;
        // dbg!(row);
        // dbg!(col);
        if let Ok(term_size) = size() {
            // let col: u16 = term_size.0;
            let row: u16 = term_size.1;
            // println!("Rows: {row} \r");
            // println!("Cols: {col} \r");
            for r in 1..row {
                out.execute(MoveTo(0, r))?;
                write!(out, "~")?;
            }
            out.execute(MoveTo(0, 0))?;
        }
        Ok(())
    }
}
