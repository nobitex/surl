use std::sync::Arc;

use structopt::StructOpt;
use tokio::sync::Mutex;

use crate::state::State;

#[derive(StructOpt, Debug)]
pub struct SaveStateOpt {
    #[structopt(short, long, alias = "p")]
    path: String,
}

pub async fn save_state(_opt: SaveStateOpt, _state: Arc<Mutex<State>>) {
    _state.lock().await.save_state(&_opt.path);
}
