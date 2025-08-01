#![warn(clippy::pedantic, clippy::style)]
mod editor;

use editor::Editor;

fn main() {
    let mut editor = Editor::default();
    editor.run();
}
