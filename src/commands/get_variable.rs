use std::sync::Arc;

use structopt::StructOpt;
use tokio::sync::Mutex;

use crate::state::State;

#[derive(StructOpt, Debug)]
pub struct GetVariableOpt {
    #[structopt(short, long, alias = "n")]
    name: String,
}

pub async fn get_variable(_opt: GetVariableOpt, _state: Arc<Mutex<State>>) {
    let v = _state.lock().await.get_variable(&_opt.name).cloned();
    if let Some(v) = v {
        println!("{}", v.value);
    } else {
        println!("Variable not found");
    }
}
