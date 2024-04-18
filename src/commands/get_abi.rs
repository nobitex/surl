use std::sync::Arc;

use structopt::StructOpt;
use tokio::sync::Mutex;

use crate::state::State;

#[derive(StructOpt, Debug)]
pub struct GetAbiOpt {
    #[structopt(short, long, alias = "n")]
    name: String,
}

pub async fn get_abi(_opt: GetAbiOpt, _state: Arc<Mutex<State>>) {
    let abi = _state.lock().await.get_abi(&_opt.name).cloned();
    if let Some(abi) = abi {
        println!("{:?}", abi);
    } else {
        println!("ABI not found");
    }
}
