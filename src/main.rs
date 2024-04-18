extern crate termion;
mod commands;
mod state;
mod terminal;

use colored::Colorize;
use commands::{
    GetAbiOpt, GetAbisOpt, GetVariableOpt, LoadAbiOpt, LoadStateOpt, PWDOpt, SaveStateOpt,
    SetVariableOpt,
};
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

use std::io::{self, BufRead};
use std::{
    io::{stdin, Write},
    sync::Arc,
};
use structopt::StructOpt;

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
    #[structopt(name = "help")]
    Help,
}

#[tokio::main]
async fn main() {
    let context = Arc::new(Mutex::new(State::new()));

    help();
    let stdin = stdin();

    print!("{}", ">> ".green());
    io::stdout().flush().unwrap();
    for line in stdin.lock().lines() {
        let line = vec!["surl", &line.unwrap()].join(" ");
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        let matches = Opt::clap().get_matches_from_safe_borrow(parts.clone());
        if matches.is_err() {
            if parts.len() > 1 {
                println!(
                    "{}",
                    "Invalid command; type 'help' for a list of commands".red()
                );
            }
            print!("{}", ">> ".green());
            io::stdout().flush().unwrap();
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
            Opt::Help => {
                help();
            }
            _ => {}
        }
        print!("{}", ">> ".green());
        io::stdout().flush().unwrap();
    }

    // let stdin = std::io::stdin();
    // let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
    // let mut terminal = Terminal::new(stdin, stdout);
    // let line = terminal.read_line();
    // println!("line: {:?}", line);

    // let private_key = "4f3edf983ac636a65a842ce7c78d9aa706d3b113bce9c46f30d7d21715b23b1d";
    // let chain_id = 1337u64;
    // let endpoint = "http://localhost:8545";
    // let provider = Provider::<Http>::try_from(endpoint).unwrap();
    // let private_key_bytes = hex::decode(private_key).expect("Invalid hex string for from");
    // let private_key: SecretKey<_> =
    //     SecretKey::from_slice(&private_key_bytes).expect("Invalid private key");
    // let wallet = wallet::from(private_key).with_chain_id(chain_id);
    // let client = Arc::new(SignerMiddleware::new(provider.clone(), wallet));

    // println!("current directory: {:?}", std::env::current_dir().unwrap());
    // let abi_path = "./abi/erc20.abi";
    // let abi_reader = std::fs::File::open(abi_path).unwrap();
    // let abi = abi::Abi::load(abi_reader).unwrap();
    // // let abi = include_str!("../abi/erc20.abi");
    // let contract_address = "0xd45a464a2412a2f83498d13635698a041b9dbe9b";
    // let h160_contract_address = H160::from_str(contract_address).unwrap();
    // let contract = Contract::new(h160_contract_address, abi, client);
}

fn help() {
    println!("{}", "surl (Solidity CURL) CLI".blue().bold());
    println!();

    println!("{}", "Commands:".yellow());
    println!();

    println!("{}", "set-variable, sv - Set a variable".yellow());
    println!("    Params: -n, --name: The name of the variable");
    println!("            -v, --value: The value of the variable");
    println!("    Example: set-variable -n key -v value");
    println!();

    println!("{}", "get-variable, gv - Get a variable".yellow());
    println!("    Params: -n, --name: The name of the variable");
    println!("    Example: get-variable -n key");
    println!();

    println!("{}", "load-abi, la - Load an ABI".yellow());
    println!("    Params: -n, --name: The name of the ABI");
    println!("            -p, --path: The path to the ABI file");
    println!("    Example: load-abi -n erc20 -p ./abi/erc20.abi");
    println!();

    println!("{}", "get-abis-list, gal - Get a list of ABIs".yellow());
    println!("    Example: get-abis-list");
    println!();

    println!("{}", "get-abis, ga - Get an ABI".yellow());
    println!("    Params: -n, --name: The name of the ABI");
    println!("    Example: get-abis -n erc20");
    println!();

    println!("{}", "pwd - Print the current working directory".yellow());
    println!("    Example: pwd");
    println!();

    println!("{}", "load-state, ls - Load the state".yellow());
    println!("    Params: -p, --path: The path to the state file");
    println!("    Example: load-state");
    println!();

    println!("{}", "save-state, ss - Save the state".yellow());
    println!("    Params: -p, --path: The path to the state file");
    println!("    Example: save-state -p ./state.json");
    println!();

    println!("{}", "help - Show this help message".yellow());
    println!("    Example: help");
    println!();
}
