use std::collections::HashMap;
use crate::storage::Storage;
use crate::command::Command;
use crate::storage::Wal;
pub struct Database {
    data: HashMap<String, String>,
    storage:Storage,
}

impl Database {
    pub fn new() -> Self {
        let file_path: &str = "storage/data.db";
        let wal_path = "stroage/database.wal";
        let wal:Wal = Wal::new(wal_path);
        let storage_var: Storage = Storage::new(file_path, wal);
        let data_map = match storage_var.load() {
            Ok(data) => data,
            Err(e) => {
                // Handle the error
                panic!("Failed to load database: {}", e);
            }};
        
        Self{ data:data_map, storage:storage_var}
    }

    pub fn set(&mut self, key: String, command:Command, value: String) {
        self.data.insert(key, value);
        match self.storage.save(command, &self.data) {
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

    pub fn delete(&mut self, command:Command, key: &str) -> bool {
        self.data.remove(key);
        match self.storage.save(command, &self.data) {
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