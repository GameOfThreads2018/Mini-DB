use std::collections::HashMap;
use std::io::{self, Write};

fn main() {
    let mut map: HashMap<String, String> = HashMap::with_capacity(100);
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
                        if !map.contains_key(key_to_insert) {
                            map.insert(key_to_insert.to_string(), value_to_insert.to_string());
                        }
                        else {
                            println!("Value already exists"); 
                        }
                    }
                    "GET" => {
                        let key_to_query = words[1].trim();
                        if map.contains_key(key_to_query) {
                            let value: Option<&String> = map.get(key_to_query);
                            println!("{}",value.map(|s| s.as_str()).unwrap_or(""));
                        }
                        else {
                            println!("(NIL)");
                        }
                    }
                    "DELETE" => {
                        let key_to_delete = words[1].trim();
                        if map.contains_key(key_to_delete) {
                            map.remove(key_to_delete);
                        }
                        else {
                            println!("No value to Delete");
                        }
                    }
                    "EXISTS" => {
                        let key_to_query = words[1].trim();
                        if map.contains_key(key_to_query) {
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
