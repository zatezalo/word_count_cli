mod web_counter;
mod file_counter;
mod utils;
use reqwest::Error;
use utils::*;
use web_counter::web_count;
use file_counter::count;

#[tokio::main]
async fn  main() -> Result<(), Error>  {
    let config = Config::new();
    count(&config);
    println!("###############################");
    web_count(&config).await;
    Ok(())
}


