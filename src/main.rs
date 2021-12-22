use std::io;
use std::io::Write;

use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().keys();

    loop {
        let input = stdin.next();

        if let Some(Ok(key)) = input {
            match key {
                termion::event::Key::Char('q') => break,
                _ => {
                    write!(
                        stdout,
                        "{}{}Key pressed: {:?}",
                        termion::clear::All,
                        termion::cursor::Goto(1, 1),
                        key
                    )
                    .unwrap();

                    stdout.lock().flush().unwrap();
                }
            }
        }
    }
}
