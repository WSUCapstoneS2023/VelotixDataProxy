// read input
use std::io;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
        "host=localhost user=postgres password=walkout dbname=test",
        NoTls,
    )?;

    
    fn query(client: &mut Client) {
       loop{ println!("Enter a search term(name or email): (enter false to stop)");
        let mut search_term = String::new();
        std::io::stdin().read_line(&mut search_term).expect("failed to read line");
    
        let search_term = search_term.trim();

        if search_term=="false"
        {
            break;
        }
    
        println!("Searching for term: {}", search_term);
    
        // let rows = client.query(
        //     "SELECT * FROM users WHERE name =$1 OR email =$1",
        //     &[&search_term],
        // ).expect("failed to execute query");
        let rows = client.query("SELECT * FROM users WHERE name LIKE $1 OR email LIKE $1", &[&format!("%{}%", search_term)]).unwrap();

        if rows.is_empty() {
            println!("No rows found for term: {}", search_term);
        } else {
            for row in rows {
                let id: i32 = row.get(0);
                let name: String = row.get(1);
                let email: String = row.get(2);
                println!("id: {}, name: {}, email: {}", id, name.trim(), email);
            }
        }}
    }
    
    // create a table if it does not exits in the database file
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
                id              INTEGER PRIMARY KEY,
                name            TEXT NOT NULL,
                email           TEXT NOT NULL
                
        );",
        &[]
    ).unwrap();

    println!("Logged in to the database");
    println!("Enter your access code");
    let mut code = String::new();
    io::stdin().read_line(&mut code).expect("Failed to read line");
    let code = (code.trim()).parse::<i32>().unwrap();
    
    let level: i32 = client.query_one("SELECT level FROM access WHERE id = $1", &[&code])?.get(0);

    // Deletes all users (REMOVE THIS IF YOU DO NOT WANT DATABASE INFO TO BE DELETED)
    //client.execute("DELETE FROM users", &[])?;

    // If there is more than one argument when loading the file you will open a file to read data
    // EX: cargo run example.txt will open a text file to read and add to database otherwise you will have to manually add info
    if level==1
    {
        println!("Level 1 access confirmed");
    }
    
    if args.len() > 1 
    { if level==1
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
        } query(&mut client);

    }
    else if level==0 {
      println!("*** PERMISSION DENIED to open and read file ****");
      println!("Level 0 access confirmed");
      query(&mut client);
      
    }

   }
    else if level==1 // manually add user information if no text file is provided
    {
        let mut run = true;

        // while the user is still adding names to the program
        while run
        {
            // get id input
            println!("Enter an id: (enter false to stop)");
            let mut id = String::new();
            io::stdin().read_line(&mut id).expect("failed to read line");
            let id = id.trim().parse::<i32>().expect("Invalid input");
            println!("You entered: {}", id);
            if (id=="false")
            {
                break;
            }
    
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
                name: (&name).to_string(),
                
                
                email: (&email).to_string(),
                
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
        query(&mut client);
        

    }

    else if level==0
    {   println!("Level 0 access confirmed");
        query(&mut client);
        
    }

    //Get information from database and output it
    // for row in client.query("SELECT id, name, email FROM users", &[])? {
    //     let person = User {
    //         id: row.get(0),
    //         name: row.get(1),
    //         email: row.get(2),
    //     };
    //     println!("Found person {}", person.name);
    // }
    
    Ok(())
}
