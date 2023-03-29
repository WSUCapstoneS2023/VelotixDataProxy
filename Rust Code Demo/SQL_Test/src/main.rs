// read input
use std::io;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};


// sql
use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection, Result};

// display
use druid::widget::{Align, Flex, Label, TextBox, Button};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc};

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

    // Connect to the database
    let mut client = Client::connect(
        "host=localhost user=postgres password=password dbname=test",
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
    else
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

    for row in client.query("SELECT id, name, email FROM users", &[])? {
        let person = User {
            id: row.get(0),
            name: row.get(1),
            email: row.get(2),
        };
        println!("Found person {}", person.name);
    }

    // describe the main window
    let main_window = WindowDesc::new(build_root())
    .title("SQL Test")
    .window_size((400.0, 400.0));

    // start the application
    AppLauncher::with_window(main_window)
        .launch(())
        .expect("Failed to launch application");

    //client.execute("DELETE FROM users", [])?;

    Ok(())
}

// Opens a database and returns the connection
fn set_conn(database: String) ->  Result<Connection>
{
    // Open a connection to a new or existing SQLite database file
    let conn = Connection::open(database)?;

    // Return the connection
    Ok(conn)
}


fn build_root() -> impl Widget<()>
{
    Flex::column()
        // Add a button widget to the Flex widget
        .with_child(Button::new("Click me!").on_click(|_, _, _| {
            /*conn.execute(
                "
                INSERT OR IGNORE INTO users (id, name, email) 
                VALUES (?1, ?2, ?3)", 
                (&me.id, &me.name, &me.email)
            );*/
        }))
}
