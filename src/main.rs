extern crate termion;
mod commands;
mod state;

use commands::{GetAbiOpt, GetAbisOpt, GetVariableOpt, LoadAbiOpt, LoadStateOpt, PWDOpt, SaveStateOpt, SetVariableOpt};
use ethers::{
    core::k256::elliptic_curve::SecretKey,
    middleware::SignerMiddleware,
    prelude::*,
    providers::{Http, Provider},
    signers::{Signer, Wallet as wallet},
    utils::hex,
};
use state::State;
use tokio::sync::Mutex;

use std::{collections::HashMap, io::BufRead, path, str::FromStr, thread::sleep, time::Duration};
use std::{
    io::{stdin, stdout, Write},
    sync::Arc,
};
use structopt::StructOpt;
use termion::cursor;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{MouseTerminal, TermRead};
use termion::raw::IntoRawMode;

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "set-variable", alias = "sv")]
    SetVariable(SetVariableOpt),
    #[structopt(name = "get-variable", alias = "gv")]
    GetVariable(GetVariableOpt),
    #[structopt(name = "load-abi", alias = "la")]
    LoadAbi(LoadAbiOpt),
    #[structopt(name = "get-abis-list", alias = "gal")]
    GetAbis(GetAbisOpt),
    #[structopt(name = "get-abis", alias = "ga")]
    GetAbi(GetAbiOpt),
    #[structopt(name = "pwd")]
    PWD(PWDOpt),
    #[structopt(name = "load-state", alias = "ls")]
    LoadState(LoadStateOpt),
    #[structopt(name = "save-state", alias = "ss")]
    SaveState(SaveStateOpt),
}

#[tokio::main]
async fn main() {
    let private_key = "4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d";
    let chain_id = 1337u64;
    let endpoint = "http://localhost:8545";
    let provider = Provider::<Http>::try_from(endpoint).unwrap();
    let private_key_bytes = hex::decode(private_key).expect("Invalid hex string for from");
    let private_key: SecretKey<_> =
        SecretKey::from_slice(&private_key_bytes).expect("Invalid private key");
    let wallet = wallet::from(private_key).with_chain_id(chain_id);
    let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    println!("current directory: {:?}", std::env::current_dir().unwrap());
    let abi_path = "./abi/erc20.abi";
    let abi_reader = std::fs::File::open(abi_path).unwrap();
    let abi = abi::Abi::load(abi_reader).unwrap();
    // let abi = include_str!("../abi/erc20.abi");
    let contract_address = "0xd45a464a2412a2f83498d13635698a041b9dbe9b";
    let h160_contract_address = H160::from_str(contract_address).unwrap();
    let contract = Contract::new(h160_contract_address, abi, client);

    let context = Arc::new(Mutex::new(State::new()));

    let stdin = stdin();
    for line in stdin.lock().lines() {
        let line = vec!["surl", &line.unwrap()].join(" ");
        let parts = line.split_whitespace().collect::<Vec<&str>>();

        let matches = Opt::clap().get_matches_from_safe_borrow(parts.clone());
        if matches.is_err() {
            println!("Invalid command");
            continue;
        }

        let opt = Opt::from_iter(parts);
        match opt {
            Opt::SetVariable(opt) => {
                commands::set_variable(opt, context.clone()).await;
            }
            Opt::GetVariable(opt) => {
                commands::get_variable(opt, context.clone()).await;
            }
            Opt::LoadAbi(opt) => {
                commands::load_abi(opt, context.clone()).await;
            }
            Opt::GetAbis(opt) => {
                commands::get_abis(opt, context.clone()).await;
            }
            Opt::GetAbi(opt) => {
                commands::get_abi(opt, context.clone()).await;
            }
            Opt::PWD(opt) => {
                commands::pwd(opt).await;
            }
            Opt::LoadState(opt) => {
                commands::load_state(opt, context.clone()).await;
            }
            Opt::SaveState(opt) => {
                commands::save_state(opt, context.clone()).await;
            }
            _ => {}
        }
    }

    // command_listener(&mut command_handler);
    // std::thread::spawn(move || {
    //     command_listener(&mut state);
    // });

    // sleep(Duration::from_secs(100000));
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
