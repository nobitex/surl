use std::sync::Arc;

use structopt::StructOpt;
use tokio::sync::Mutex;

use crate::state::State;

#[derive(StructOpt, Debug)]
pub struct LoadStateOpt {
    #[structopt(short, long, alias = "p")]
    path: String,
}

pub async fn load_state(_opt: LoadStateOpt, _state: Arc<Mutex<State>>) {
    _state.lock().await.load_state(&_opt.path);
}
