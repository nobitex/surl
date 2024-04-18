use std::io::{Stdin, Write};

use termion::{
    cursor,
    event::{Event, Key},
    input::{MouseTerminal, TermRead},
};

pub trait TerminalWriter {
    fn clear(&mut self);
    fn read_line(&mut self) -> String;
}

pub struct Terminal<W>
where
    W: Write,
{
    stdin: std::io::Stdin,
    stdout: MouseTerminal<W>,
}

impl<W> Terminal<W>
where
    W: Write,
{
    pub fn new(stdin: Stdin, stdout: W) -> Self {
        let stdin = stdin;
        let stdout = MouseTerminal::from(stdout);
        Self { stdin, stdout }
    }
}

impl<W> TerminalWriter for Terminal<W>
where
    W: Write,
{
    fn clear(&mut self) {
        write!(self.stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn read_line(&mut self) -> String {
        for c in self.stdin.lock().events() {
            let evt = c.unwrap();
            match evt {
                Event::Key(Key::Ctrl('q')) => break,
                Event::Key(Key::Backspace) => {
                    write!(self.stdout, "{}", termion::cursor::Left(1)).unwrap();
                    write!(self.stdout, "{}", termion::clear::UntilNewline).unwrap();
                }
                Event::Key(Key::Left) => {
                    write!(self.stdout, "{}", termion::cursor::Left(1)).unwrap();
                }
                Event::Key(Key::Char(c)) => {
                    if c == '\t' {
                        write!(self.stdout, "tab").unwrap();
                    } else if c == '\n' {
                        break;
                    } else {
                        write!(self.stdout, "{}", c).unwrap();
                    }
                }
                _ => {}
            }
            self.stdout.flush().unwrap();
        }
        self.stdout.flush().unwrap();
        "asdasd".to_string()
    }
}
