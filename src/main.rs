use crate::config::config::Config;

mod config;

#[tokio::main]
async fn main() {
    let cfg = Config::load_config("./config/config.yaml").expect("loading application config");

    println!("{:#?}", cfg);
}
