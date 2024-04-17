extern crate termion;

use ethers::{
    core::k256::elliptic_curve::SecretKey,
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Provider},
    signers::{Signer, Wallet as wallet},
    utils::hex,
};

use std::{collections::HashMap, str::FromStr};
use std::{
    io::{stdin, stdout, Write},
    sync::Arc,
};
use termion::cursor;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

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
    assert_eq!(
        eval_expression("hello $foo world", &state),
        "hello bar world"
    );
    assert_eq!(
        eval_expression("hello $foo world $foo", &state),
        "hello bar world bar"
    );
    assert_eq!(
        eval_expression("hello $foo world $bar", &state),
        "hello bar world "
    );
}

fn eval_command(command: &str, state: &mut State) {
    let mut parts = command.splitn(2, ' ');
    match parts.next() {
        Some("set") => {
            let mut parts = parts.next().unwrap().splitn(2, ' ');
            let name = parts.next().unwrap();
            let value = parts.next().unwrap();
            state.set_variable(name.to_string(), value.to_string());
        }
        Some("print") => {
            let expression = parts.next().unwrap();
            println!("{}", eval_expression(expression, state));
        }
        _ => println!("Unknown command"),
    }
}

#[test]
fn test_eval_command() {
    let mut state = State::new();
    eval_command("set foo bar", &mut state);
    eval_command("print hello $foo", &mut state);
    eval_command("print hello $foo world", &mut state);
    eval_command("print hello $foo world $foo", &mut state);
    eval_command("print hello $foo world $bar", &mut state);
}

fn main() {
    let private_key = "4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d";
    let chain_id = 1337u64;
    let endpoint = "http://localhost:8545";
    let provider = Provider::<Http>::try_from(endpoint).unwrap();
    let private_key_bytes = hex::decode(private_key).expect("Invalid hex string for from");
    let private_key: SecretKey<_> =
        SecretKey::from_slice(&private_key_bytes).expect("Invalid private key");
    let wallet = wallet::from(private_key).with_chain_id(chain_id);
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    let abi_path = "abi/erc20.abi";
    let abi = serde_json::from_str::<abi::Abi>(abi_path).unwrap();
    let contract_address = "0xd45a464a2412a2f83498d13635698a041b9dbe9b";
    let h160_contract_address = H160::from_str(contract_address).unwrap();
    let contract = Contract::new(h160_contract_address, abi, client);
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
