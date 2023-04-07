use regex::Regex;
use reqwest;
use std::{collections::HashMap, fs};
use crate::utils::Config;

struct WebCouner {
    name: String,
    link: String,
    word_count: HashMap<String, u32>,
    // links: Vec<String>,
}

pub async fn web_count(config: &Config) {
    let websites: Vec<String> = fs::read_to_string("example_web/websites.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect();
    visit_links(websites, config).await;
}

async fn visit_links(links: Vec<String>, config: &Config) {
    for link in links {
        go_threw_websits(link.to_string(), &config).await;
    }
}

async fn go_threw_websits(link: String, config: &Config) {
    let web = WebCouner::new(config.keywords.clone(), &link.to_string()).await;
    println!("{}", web);
}

impl WebCouner {
    async fn new(keywords: Vec<String>, link: &String) -> WebCouner {
        let word_regex = Regex::new(r"\w+").unwrap();
        // let link_regex = Regex::new(r#"href=["']([^"']+)["']"#).unwrap();
        let title_regex = Regex::new(r"<title>(?P<title>.*?)</title>").unwrap();
        let response = reqwest::get(link).await.expect("Not a good link").text().await.expect("Not text");

        let name = title_regex
            .captures(&response)
            .and_then(|caps| caps.name("title"))
            .map(|title| title.as_str().to_string())
            .unwrap_or_else(|| "No title found".to_string());

        // let links: Vec<String> = link_regex
        //     .captures_iter(&response)
        //     .filter_map(|cap| cap.get(1))
        //     .map(|m| m.as_str().to_string())
        //     .collect();

        let mut word_count: HashMap<String, u32> = HashMap::new();

        for word in word_regex.find_iter(&response).map(|word| word.as_str()) {
            if keywords.contains(&word.to_lowercase()) {
                *word_count.entry(word.to_lowercase()).or_insert(0) += 1;
            }
        };

        WebCouner {
            name,
            link: link.to_string(),
            word_count,
            // links,
        }
    }
}

impl std::fmt::Display for WebCouner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} \n({})\n{:?}\n---------------------------", self.name, self.link, self.word_count)
    }
}
