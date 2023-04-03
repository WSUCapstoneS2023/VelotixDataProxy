use std::collections::HashMap;
use rusqlite::{Connection, Result};
use std::fs::OpenOptions;
use std::io::{self, Write};
use chrono::prelude::*;

struct User {
    username: String,
    password: String,
    permissions: i32,
}
struct KeyValueStore {
    data: HashMap<String, String>,
    log_file: Option<std::fs::File>,
}

impl KeyValueStore {
    fn new() -> KeyValueStore {
        KeyValueStore {
            data: HashMap::new(),
            log_file: None,
        }
    }

    fn set(&mut self, key: &str, value: &str,username:&str) {
        self.log(format!("{} set key={} value={}", username, key, value));
        self.data.insert(key.to_string(), value.to_string());
        
    }

    fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
        
    }

    fn delete(&mut self, key: &str,username:&str) -> bool {
        
        let result=self.data.remove(key).is_some();
        self.log(format!("{} deleted key={}", username, key));
        result

    }

    fn list(&self) -> Vec<(&String, &String)> {
        self.data.iter().collect()
    }
    fn enable_logging(&mut self, log_file_path: &str) -> io::Result<()> {
        self.log_file = Some(OpenOptions::new().append(true).create(true).open(log_file_path)?);
        Ok(())
    }

    fn log(&mut self, message: String) {
        if let Some(file) = &mut self.log_file {
            
            let local_time = Local::now();
            let log_message = format!("{} - {}\n", local_time.format("%Y-%m-%d %H:%M:%S").to_string(), message);
            if let Err(e) = file.write_all(log_message.as_bytes()) {
                eprintln!("Error writing to log file: {}", e);
            }
        }
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
    let mut stmt = conn.prepare("SELECT password,permissions FROM users WHERE username = ?1")?;
    let user_result: Result<User> = stmt.query_row(&[&username], |row| Ok(User{
        username:username.to_string(),
        password:row.get(0)?,
        permissions:row.get(1)?,
    })

    );

    match user_result {
        Ok(user) if user.password == password => {
            println!("Login successful! Welcome {}",user.username);
            let mut store = KeyValueStore::new();
            store.enable_logging("log.txt").unwrap();

            loop {
                let mut input = String::new();
                print!("> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();

                let args: Vec<&str> = input.split_whitespace().collect();

                match args.as_slice() {
                    ["set", key, value] => 
                    {
                        if user.permissions>=1
                        {
                           store.set(key,value,&user.username);
                        }  
                        else{println!("you dont have permission for 'SET'")}
                    },
                    ["get", key] => match store.get(key) {
                        Some(value) => println!("{}", value),
                        None => println!("Key not found"),
                    },
                    ["delete", key] => {
                          if user.permissions>=1
                          {
                           match store.delete(key,&user.username) {
                           true => println!("Key deleted"),
                           false => println!("Key not found"),
                          }
                          }
                          else{println!("you dont have permission for 'DELETE' ")}
                       },
                    ["list"] => {
                        for (key, value) in store.list() {
                            println!("{}: {}", key, value);
                        }
                    },
                    ["quit"] => break,
                    _ => println!("Invalid command"),
                }
            }
        }
        _ => println!("Invalid username or password"),
    }

    Ok(())
}
