use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::style::Print;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use crossterm::{Command, queue};
use std::io::Write;
use std::io::{Error, stdout};

pub struct Terminal {}

pub struct Size {
    pub columns: u16,
    pub rows: u16,
}

#[derive(Copy, Clone)]
pub struct Coordinate {
    pub x: u16,
    pub y: u16,
}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor(Coordinate { x: (0), y: (0) })?;
        Self::execute()?;
        Ok(())
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen() -> Result<(), Error> {
        // clear screen
        Self::queue_command(Clear(ClearType::All))?;
        Ok(())
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))?;
        Ok(())
    }

    pub fn move_cursor(coord: Coordinate) -> Result<(), Error> {
        Self::queue_command(MoveTo(coord.x, coord.y))?;
        Ok(())
    }

    pub fn size() -> Result<Size, Error> {
        let (c, r) = size()?;
        Ok(Size {
            columns: c,
            rows: r,
        })
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()?;
        Ok(())
    }

    pub fn print(string: &str) -> Result<(), Error> {
        Self::queue_command(Print(string))?;
        Ok(())
    }

    pub fn print_at(string: &str, coord: Coordinate) -> Result<(), Error> {
        Self::move_cursor(coord)?;
        Self::print(string)?;
        Ok(())
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    // T is a type Command, cmd is type T
    pub fn queue_command<T: Command>(cmd: T) -> Result<(), Error> {
        queue!(stdout(), cmd)?;
        Ok(())
    }
}
