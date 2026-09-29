use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::fs::File;
use std::fmt::Write;
use std::io::{BufRead, BufReader};
use crate::command::Command;

pub struct Storage {
    path: String,
    wal: Wal
}

pub struct Wal {
    path: String
}

impl Storage {
    pub fn new(file_path: &str, write_ahead_log:Wal) -> Self{
        Self{path:String::from(file_path), wal:write_ahead_log}
    }

    pub fn load(&self) -> Result<HashMap<String, String>, std::io::Error> {
    let file = File::open(&self.path)?;
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

    pub fn save(&mut self, command:Command, data: &HashMap<String, String>) -> Result<(), std::io::Error> {
        self.wal.append(&command)?;
        let mut result_str = String::new();
        for (key, value) in data {
            result_str.push_str(&format!("{}|{}\n", key, value));
        }
        fs::write(&self.path, result_str)?;
        Ok(())
    }
}

impl Wal {
    pub fn new(file_path:&str) -> Self{
         Self{path:String::from(file_path)}
    }
    pub fn append(&mut self, command: &Command) -> std::io::Result<()> {
         let command_string = command.to_string();
         let mut file = OpenOptions::new().append(true).create(true).open(&self.path)?;
        //  writeln!(file, "{}")?;
         Ok(())
    }

    // pub fn recover(&mut self) -> Result<Vec<Command>{
    // }
}