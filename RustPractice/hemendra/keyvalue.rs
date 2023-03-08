use std::collections::HashMap;

use std::io::{self, Write};

struct KeyValueStore {
    data: HashMap<String, String>,
}

impl KeyValueStore {
    fn notnew() -> KeyValueStore {
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

fn main() {
    let mut store = KeyValueStore::notnew();

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
