use std::sync::Arc;

use structopt::StructOpt;
use tokio::sync::Mutex;

use crate::state::State;

#[derive(StructOpt, Debug)]
pub struct SetVariableOpt {
    #[structopt(short, long, alias = "n")]
    name: String,
    #[structopt(short, long, alias = "v")]
    value: String,
}

pub async fn set_variable(_opt: SetVariableOpt, _state: Arc<Mutex<State>>) {
    _state.lock().await.set_variable(_opt.name, _opt.value);
}
