use std::collections::HashMap;
use rusqlite::{Connection, Result};
use std::io::{self, Write};

struct User {
    username: String,
    password: String,
    permissions: i32,
}
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
                           store.set(key,value);
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
                           match store.delete(key) {
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
