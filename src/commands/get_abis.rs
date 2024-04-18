use std::sync::Arc;

use structopt::StructOpt;
use tokio::sync::Mutex;

use crate::state::State;

#[derive(StructOpt, Debug)]
pub struct GetAbisOpt {}

pub async fn get_abis(_opt: GetAbisOpt, _state: Arc<Mutex<State>>) {
    let abis = _state.lock().await.get_abis().clone();
    for (name, _) in abis {
        println!("{}", name);
    }
}
