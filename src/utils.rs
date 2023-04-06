use std::fs;

const PATH_TO_APP_PROPERTIES: &str = "app.properties";

pub struct Config {
    pub keywords:  Vec<String>,
    pub hop_count: i32,
}

impl Config {
    pub fn new() -> Config {
        let app_properties: Vec<String> =
            fs::read_to_string(PATH_TO_APP_PROPERTIES)
            .expect("unable to read file")
            .lines()
            .map(|line| line.split('=').nth(1).map(String::from).unwrap())
            .collect();
        
        let keywords: Vec<String> = app_properties.get(0)
            .unwrap_or(&"".to_string())
            .split(", ")
            .map(String::from)
            .collect();

        let hop_count = app_properties.get(1)
            .expect("missing hop count property")
            .parse::<i32>()
            .expect("faield to parse as i32");

        Config{
            hop_count,
            keywords,
        }
    }
}