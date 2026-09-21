use std::io::{self, Write};
use crate::database::Database;
mod database;
mod storage;

fn main() {
    let mut database_inst: Database = Database::new();
    println!("Insert commands (SET, GET, DELETE, EXIST) along with key/values as appropriate");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
             Ok(_) => {
                let words: Vec<&str> = input.split(' ').collect();
                if words.len() > 3 {
                    println!("ERROR: insert a valid COMMAND");
                    continue;
                }
                let command = words[0].trim();

                match command {
                    "help" => {
                        println!("Available commands:");
                        println!("  SET  - sets an item in the hashmap");
                        println!("  EXISTS   - checks if value exists in hashmap");
                        println!("  GET   - fetch a existing value from the hashmap");
                        println!("  DELETE   - removes a key-value pair from hashmap, returns value");
                    }
                    "SET" => {
                        let key_to_insert = words[1].trim();
                        let value_to_insert = words[2].trim();
                        if !database_inst.exists(key_to_insert) {
                            database_inst.set(key_to_insert.to_string(), value_to_insert.to_string());
                        }
                        else {
                            println!("Value already exists"); 
                        }
                    }
                    "GET" => {
                        let key_to_query = words[1].trim();
                        if database_inst.exists(key_to_query) {
                            let value: Option<&String> = database_inst.get(key_to_query);
                            println!("{}",value.map(|s| s.as_str()).unwrap_or(""));
                        }
                        else {
                            println!("(NIL)");
                        }
                    }
                    "DELETE" => {
                        let key_to_delete = words[1].trim();
                        if database_inst.exists(key_to_delete) {
                            let removed_value = database_inst.delete(key_to_delete);
                            if removed_value {
                                println!("Remove operation successful Key:{}", key_to_delete);
                            }
                        }
                        else {
                            println!("No value to Delete");
                        }
                    }
                    "EXISTS" => {
                        let key_to_query = words[1].trim();
                        if database_inst.exists(key_to_query) {
                            println!("true");
                        }
                        else {
                            println!("false");
                        }
                    }
                    "EXIT" => {
                        println!("Goodbye!");
                        break;
                    }
                    "" => {
                    }
                    other => {
                        println!("Unknown command: '{}'. Type 'help' for options.", other);
                    }
                }
             }
            Err(error) => {
                println!("Error reading input: {}", error);
                break;
            }
        }
    }

}
