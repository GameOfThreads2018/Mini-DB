use std::collections::HashMap;
use crate::storage::Storage;
pub struct Database {
    data: HashMap<String, String>,
    file_path: String
}

impl Database {
    pub fn new() -> Self {
        let file_path: &str = "storage/data.db";
        let data_map = match Storage::load(file_path) {
            Ok(data) => data,
            Err(e) => {
                // Handle the error
                panic!("Failed to load database: {}", e);
            }};
        
        Self{ data:data_map, file_path:String::from(file_path)}
    }

    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
        match Storage::save(&self.file_path, &self.data) {
            Ok(()) => {
                // TODO add better logging messages
                println!("Successfully inserted a new key")
            }
            Err(e) => {
                panic!("Failed to save database: {}", e);
            }
        }
        
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key);
         match Storage::save(&self.file_path, &self.data) {
            Ok(()) => {
                // TODO add better logging messages
                println!("Successfully deleted a {}", key);
                true
            }
            Err(e) => {
                panic!("Failed to save database: {}", e);
            }
        }

    }

    pub fn exists(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}