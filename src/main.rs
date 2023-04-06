mod web_counter;
mod file_counter;
mod utils;
use file_counter::count;
use utils::*;

fn main() {
    let config = Config::new();
    println!("{}  {:?}", config.hop_count, config.keywords);
    count(config);
}


