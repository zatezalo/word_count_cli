use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::Config;

const PATH_TO_EXAMPLES: &str = "example";

pub fn count(config: Config) {
    go_threw_dirs(PathBuf::from(PATH_TO_EXAMPLES), &config, 0);
}

fn go_threw_dirs(path: PathBuf, config: &Config, index: i32) {
    if index >= config.hop_count {
        return;
    }
    for entry in fs::read_dir(path).expect("failed to read directory") {
        let path = entry.unwrap().path();
        if path.is_file() {
            println!("{}", FileCouner::new(config.keywords.clone(), &path.to_str().unwrap().to_string()));
        } else if path.is_dir() {
            go_threw_dirs(path, config, index + 1);
        }
    }
}

struct FileCouner {
    name: String,
    path: String,
    word_count: HashMap<String, u32>
}

impl FileCouner {
    fn new(keywords: Vec<String>, path: &String) -> FileCouner {
        let file_lines: Vec<String> =
            fs::read_to_string(path)
            .expect("unable to read file")
            .lines()
            .map(|line| line.to_string())
            .collect();

        let file_words: Vec<String> = file_lines.iter()
            .flat_map(|line| line.split_whitespace().map(|word| word.to_string()))
            .collect();
        
        let mut word_count: HashMap<String, u32> = HashMap::new();

        for word in file_words {
            if keywords.contains(&word.to_lowercase().to_string()) {
                *word_count.entry(word).or_insert(0) += 1;
            }
        }

        let name = Path::new(path)
            .file_name()
            .expect("Unable to get file name")
            .to_string_lossy()
            .to_string();
        
        FileCouner {
            name,
            path: path.to_string(),
            word_count
        }
    }
}

impl std::fmt::Display for FileCouner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})\n{:?}\n---------------------------", self.name, self.path, self.word_count)
    }
}
