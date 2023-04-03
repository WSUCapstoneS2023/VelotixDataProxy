use std::collections::HashMap;
use rusqlite::{Connection, Result};
use std::io::{self, Write};


struct KeyValueStore {
    data: HashMap<String, String>,
}

impl KeyValueStore {
    fn new() -> KeyValueStore {
        KeyValueStore {
            data: HashMap::new(),
        }
    }

    fn set(&mut self, key: &str, value: &str) {
        self.data.insert(key.to_string(), value.to_string());
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    fn delete(&mut self, key: &str) -> bool {
        self.data.remove(key).is_some()
    }

    fn list(&self) -> Vec<(&String, &String)> {
        self.data.iter().collect()
    }
}

fn main() -> Result<()> {

    let conn = Connection::open("users.db")?;

    

    // Prompt the user to enter their username
    println!("Please enter your username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");

    // Prompt the user to enter their password
    println!("Please enter your password:");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");

    // Trim any whitespace from the username and password
    let username = username.trim();
    let password = password.trim();

    // Query the database for the user's login credentials
    let mut stmt = conn.prepare("SELECT password FROM users WHERE username = ?1")?;
    let password_result: Result<String> = stmt.query_row(&[&username], |row| row.get(0));

    match password_result {
        Ok(db_password) if db_password == password => {
            println!("Login successful!");
            let mut store = KeyValueStore::new();

            loop {
                let mut input = String::new();
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();

                let args: Vec<&str> = input.split_whitespace().collect();

                match args.as_slice() {
                    ["set", key, value] => store.set(key, value),
                    ["get", key] => match store.get(key) {
                        Some(value) => println!("{}", value),
                        None => println!("Key not found"),
                    },
                    ["delete", key] => match store.delete(key) {
                        true => println!("Key deleted"),
                        false => println!("Key not found"),
                    },
                    ["list"] => {
                        for (key, value) in store.list() {
                            println!("{}: {}", key, value);
                        }
                    }
                    ["quit"] => break,
                    _ => println!("Invalid command"),
                }
            }
        }
        _ => println!("Invalid username or password"),
    }

    Ok(())
}
