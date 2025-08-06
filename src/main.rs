#![warn(clippy::all, clippy::pedantic, clippy::print_stdout)]
#[allow(clippy::cast_possible_truncation)]
mod editor;
use editor::Editor;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
