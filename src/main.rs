extern crate termion;

use ethers::{
    prelude::*,
    providers::{Http, Provider},
};

use termion::cursor;
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use std::collections::HashMap;
use std::io::{Write, stdout, stdin};

struct Variable {
    value: String,
}

struct State {
    variables_by_name: HashMap<String, Variable>,
}

impl State {
    fn new() -> State {
        State {
            variables_by_name: HashMap::new(),
        }
    }

    fn set_variable(&mut self, name: String, value: String) {
        self.variables_by_name.insert(name, Variable { value });
    }

    fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables_by_name.get(name)
    }
}

#[test]
fn test_state() {
    let mut state = State::new();
    state.set_variable("foo".to_string(), "bar".to_string());
    assert_eq!(state.get_variable("foo").unwrap().value, "bar");
    assert_eq!(state.get_variable("foo").unwrap().value, "bar");
}

fn eval_expression(expression: &str, state: &State) -> String {
    let mut result = String::new();
    let mut in_variable = false;
    let mut variable_name = String::new();
    for c in expression.chars() {
        if c == '$' {
            in_variable = true;
        } else if in_variable {
            if c == ' ' {
                in_variable = false;
                if let Some(variable) = state.get_variable(&variable_name) {
                    result.push_str(&variable.value);
                }
                result.push(' ');
                variable_name.clear();
            } else {
                variable_name.push(c);
            }
        } else {
            result.push(c);
        }
    }
    if in_variable {
        if let Some(variable) = state.get_variable(&variable_name) {
            result.push_str(&variable.value);
        }
    }
    result
}

#[test]
fn test_eval_expression() {
    let mut state = State::new();
    state.set_variable("foo".to_string(), "bar".to_string());
    assert_eq!(eval_expression("hello", &state), "hello");
    assert_eq!(eval_expression("hello $foo", &state), "hello bar");
    assert_eq!(eval_expression("hello $foo world", &state), "hello bar world");
    assert_eq!(eval_expression("hello $foo world $foo", &state), "hello bar world bar");
    assert_eq!(eval_expression("hello $foo world $bar", &state), "hello bar world ");
}

fn main() {
    let provider = Provider::<Http>::try_from("http://localhost:8545").unwrap();

    // let stdin = stdin();
    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    // write!(stdout, "{}{}", termion::clear::All, cursor::Goto(1, 1)).unwrap();
    // stdout.flush().unwrap();

    // for c in stdin.events() {
    //     let evt = c.unwrap();
    //     match evt {
    //         Event::Key(Key::Ctrl('q')) => break,
    //         // Event::Mouse(me) => {
    //         //     match me {
    //         //         MouseEvent::Press(_, x, y) => {
    //         //             write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
    //         //         },
    //         //         _ => (),
    //         //     }
    //         // }
    //         Event::Key(Key::Backspace) => {
    //             write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
    //             write!(stdout, "{}", termion::clear::UntilNewline).unwrap();
    //         }
    //         Event::Key(Key::Left) => {
    //             write!(stdout, "{}", termion::cursor::Left(1)).unwrap();
    //         }
    //         Event::Key(Key::Char(c)) => {
    //             if c == '\t' {
    //                 write!(stdout, "tab").unwrap();
    //             } else {
    //                 write!(stdout, "{}", c).unwrap();
    //             }
    //         }
    //         _ => {}
    //     }
    //     stdout.flush().unwrap();
    // }



}