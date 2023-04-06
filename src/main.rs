
mod utils;
use utils::*;

fn main() {
    let config = Config::new();
    println!("{}  {:?}", config.hop_count, config.keywords);
}


