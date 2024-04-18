use std::sync::Arc;

use structopt::StructOpt;
use tokio::sync::Mutex;

use crate::state::State;

#[derive(StructOpt, Debug)]
pub struct LoadAbiOpt {
    #[structopt(short, long, alias = "n")]
    name: String,
    #[structopt(short, long, alias = "p")]
    path: String,
}

pub async fn load_abi(_opt: LoadAbiOpt, _state: Arc<Mutex<State>>) {
    let abi_reader = std::fs::File::open(&_opt.path).unwrap();
    let abi = ethers::abi::Abi::load(abi_reader).unwrap();

    _state.lock().await.set_abi(_opt.name, abi);
}
