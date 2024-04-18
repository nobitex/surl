use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct PWDOpt {}

pub async fn pwd(_opt: PWDOpt) {
    println!("{:?}", std::env::current_dir().unwrap());
}
