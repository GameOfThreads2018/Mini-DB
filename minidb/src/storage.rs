use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};



pub struct Storage {
    path: String
}

impl Storage {
    pub fn new(file_path: &str) -> Self{
        Self{path:String::from(file_path)}
    }

    pub fn load(path: &str) -> Result<HashMap<String, String>, std::io::Error> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut loaded_data = HashMap::new();
    for line in reader.lines() {
        let line = line?;
        let Some((key, value)) = line.split_once('|') else {
            continue;
        };
        loaded_data.insert(key.to_string(), value.to_string());
    }
    Ok(loaded_data)
}

    pub fn save(path:&str, data: &HashMap<String, String>) -> Result<(), std::io::Error> {
        let mut result_str = String::new();
        for (key, value) in data {
            result_str.push_str(&format!("{}|{}\n", key, value));
        }
        fs::write(path, result_str)?;
        Ok(())
    }
}