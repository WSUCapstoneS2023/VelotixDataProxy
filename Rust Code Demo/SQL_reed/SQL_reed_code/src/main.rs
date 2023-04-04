// read input
use std::io;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rayon::prelude::*;
//run 'Cargo add rayon' in the terminal

// postgress
use postgres::{Client, NoTls, Error};

#[derive(Debug)]
struct User {
    id: i32,
    name: String,
    email: String,
}



fn main() -> Result<(), Error> {
    // connect to postgres database
    let args: Vec<String> = env::args().collect();

    // Connect to the database required if the program crashes this may be the main error
    let mut client = Client::connect(
        "host=localhost user=postgres password=password dbname=test-reed port=5433",
        NoTls,
    )?;

    // create a table if it does not exits in the database file
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                email           TEXT NOT NULL
        );",
        &[]
    );
    
    // Deletes all users (REMOVE THIS IF YOU DO NOT WANT DATABASE INFO TO BE DELETED)
    client.execute("DELETE FROM users", &[])?;

    // If there is more than one argument when loading the file you will open a file to read data
    // EX: cargo run example.txt will open a text file to read and add to database otherwise you will have to manually add info
    if args.len() > 1 
    {
        println!("The first argument is {}", args[1]);

        // get argument and open file
        let file_name = &args[1];
        let file = File::open(file_name).expect("Failed to read file");
        let file_reader = BufReader::new(file);

        // used for checking which parameter is being viewed for the user
        let mut i = 0;

        // set the user attributes
        let mut id: i32 = 0;
        let mut name = String::new();
        let mut email = String::new();

        // for every line in the file
        for line in file_reader.lines()
        {
            let content = line.unwrap();
            println!("{}", content);

            // set an id name or email based on the line
            match i {
                0 =>
                {
                    id = content.trim().parse::<i32>().unwrap();
                },
                1 =>
                {
                    name = content;
                },
                2 =>
                {
                    email = content;
                },
                _ => {
                    println!("Error");
                },
            }
            i = i + 1;

            // if the input is larger then 2 add the name to list and set i to zero
            if i == 3
            {
                // set user to add
                let me = User {
                    id: id,
                    name: (&name).to_string(),
                    email: (&email).to_string(),
                };
            
                // insert value to list
                client.execute(
                    "INSERT INTO users (id, name, email) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
                    &[&me.id, &me.name, &me.email],
                )?;
                println!("inserted");
                i = 0;
            }
        }
    }
    else // manually add user information if no text file is provided
    {
        let mut run = true;

        // while the user is still adding names to the program
        while run
        {
            // get id input
            println!("Enter an id:");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("failed to read line");
            let id = id.trim().parse::<i32>().expect("Invalid input");
            println!("You entered: {}", id);
        
    
            // get name input
            println!("Enter a name:");
            let mut name = String::new();
            io::stdin().read_line(&mut name).expect("failed to readline");
            print!("You entered {}", name);
        
            // get email input
            println!("Enter a email:");
            let mut email = String::new();
            io::stdin().read_line(&mut email).expect("failed to readline");
            print!("You entered {}", email);
        
            // set user to add
            let me = User {
                id: id,
                name: (&name[..name.len()-2]).to_string(),
                email: (&email[..email.len()-2]).to_string(),
            };
        
            // insert value to list
            client.execute(
                "INSERT INTO users (id, name, email) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
                &[&me.id, &me.name, &me.email],
            )?;
    
            // checks if user wants to continue adding to database
            let mut input = String::new();
            println!("Enter if you want to keep adding names (enter false to stop)");
            io::stdin().read_line(&mut input).expect("failed to readline");
            
            // get the input and loops if input is not false
            if input.trim() == "false"
            {
                run = false;
            }
            else if input.trim() != "true"
            {
                println!("error invalid input");
            }
        }

    }


    let mut allUsers: Vec<User> = Vec::new();

    // Get information from database and output it
    for row in client.query("SELECT id, name, email FROM users", &[])? {
        let person = User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        };
        println!("Found person {}", person.name);
        allUsers.push(person);
    }



    let num_wsu_emails = allUsers.par_iter().filter(|&x| x.email.contains("wsu.edu")).count() as f32;
    let num_non_wsu_emails = allUsers.par_iter().filter(|&x| !x.email.contains("wsu.edu")).count() as f32;

    let mut email_buffer: &str = "";
    let mut email_buffer2: &str = "";

    let all_wsu_emails = allUsers.par_iter()
        .map(|x| x.email.clone())
        .filter(|x| x.contains("wsu.edu"))
        .reduce(|| email_buffer.to_string(), |x, y| x + " -- " + &y + "\n");

    
    let all_non_wsu_emails = allUsers.par_iter()
        .map(|x| x.email.clone())
        .filter(|x| !x.contains("wsu.edu"))
        .reduce(|| email_buffer2.to_string(), |x, y| x + " -- " + &y + "\n");

    let all_wsu_users = allUsers.par_iter()
        .map(|x| x.clone())
        .filter(|x| x.email.contains("wsu.edu"));

    let all_non_wsu_users = allUsers.par_iter()
        .map(|x| x.clone())
        .filter(|x| !x.email.contains("wsu.edu"));

    let first_five_users = allUsers.par_iter()
        .map(|x| x.clone())
        .filter(|x| x.id < 5);

    let dr_users = allUsers.par_iter()
        .map(|x| x.clone())
        .filter(|x| x.name.contains("Dr."));

    let t1: Vec<_> = all_wsu_users.collect();
    println!("\n\nListing all WSU Users:\n----------------------\n");
    for user in t1 {
        println!("ID: {}, Username: {}, Email: {}", user.id, user.name, user.email)
    }

    let t2: Vec<_> = all_non_wsu_users.collect();
    println!("\n\nListing all non-WSU Users:\n----------------------\n");
    for user in t2 {
        println!("ID: {}, Username: {}, Email: {}", user.id, user.name, user.email)
    }

    let t3: Vec<_> = first_five_users.collect();
    println!("\n\nListing first five Users:\n----------------------\n");
    for user in t3 {
        println!("ID: {}, Username: {}, Email: {}", user.id, user.name, user.email)
    }

    let t4: Vec<_> = dr_users.collect();
    println!("\n\nListing Users w/ PhDs, MDs:\n----------------------\n");
    for user in t4 {
        println!("ID: {}, Username: {}, Email: {}", user.id, user.name, user.email)
    }
    //run 'Cargo add rayon' in the terminal to enable Rayon crate


    



    //println!("\n\nPrinting all WSU emails:\n {}\n\n", all_wsu_emails);
    //println!("Printing all non-WSU emails:\n {}\n\n", all_non_wsu_emails);












    Ok(())
}

