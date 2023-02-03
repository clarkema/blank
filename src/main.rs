use std::io::{stdin, Write};
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::screen::IntoAlternateScreen;

fn main() {
    let stdin = stdin();
    let mut screen = std::io::stdout()
        .into_raw_mode()
        .unwrap()
        .into_alternate_screen()
        .unwrap();
    write!(screen, "{}", cursor::Hide).unwrap();
    screen.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Char('\n') => break,
            Key::Ctrl('c') => break,
            _ => {}
        }
    }

    write!(screen, "{}", cursor::Show).unwrap();
}
