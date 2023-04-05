// read input
use std::io;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

// postgress
use postgres::{Client, NoTls, Error};

// rayon
use rayon::prelude::*;

// the user structure that will be found in the table users
#[derive(Debug)]
struct User {
    id: i32,
    level: i32,
    name: String,
    email: String,  
}

// Configures Database so that it can work in any situation
fn configure_database(client: &mut Client) -> Result<(), Error>
{
    // create a table if it does not exits in the database file
    client.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id              INTEGER PRIMARY KEY,
            level           INTEGER NOT NULL,
            name            TEXT NOT NULL,
            email           TEXT NOT NULL            
        );",
        &[]
    ).unwrap();

    // If the table is empty we need a few basic users to ensure we can search
    let rows = client.query("SELECT * FROM users", &[])?;
    if rows.is_empty()
    {
        println!("Empty Database Entering Basic Users");

        client.execute(
            "INSERT INTO users (id, level, name, email) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            &[&0, &1, &"Master",&"login@wsu.edu"],
        )?;
        
        client.execute(
            "INSERT INTO users (id, level, name, email) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            &[&1, &0, &"Guest",&"guest@wsu.edu"],
        )?;
    }
    else // give the user the ability to reset the database
    {
        // Ask the user if they want to reset 
        println!("Enter an 1 if you want to reset the database (Remove Every Entry)");
        
        // Get IO
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");
        let input = input.trim().parse::<i32>().expect("Invalid input");
        println!("You entered: {}", input);
        
        // If the user wants to reset the table
        if input==1
        {   
            println!("Deleting Users and adding a new Master Access User");
            
            // Delete table
            client.execute("DELETE FROM users", &[])?;

            // Add a full access and no access user
            client.execute(
                "INSERT INTO users (id, level, name, email) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
                &[&0, &1, &"Master",&"login@wsu.edu"],
            )?;
            
            client.execute(
                "INSERT INTO users (id, level, name, email) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
                &[&1, &0, &"Guest",&"guest@wsu.edu"],
            )?;
        }
    }

    Ok(())
}

// Insert all the data in text file
fn insertFileData(client: &mut Client)  -> Result<(), Error>
{
    let args: Vec<String> = env::args().collect();
    println!("The first argument is {}", args[1]);

    // get argument and open file
    let file_name = &args[1];
    let file = File::open(file_name).expect("Failed to read file");
    let file_reader = BufReader::new(file);

    // used for checking which parameter is being viewed for the user
    let mut i = 0;

    // set the user attributes
    let mut id: i32 = 0;
    let mut level: i32 = 0;
    let mut name = String::new();
    let mut email = String::new();

    // for every line in the file
    for line in file_reader.lines()
    {
        let content = line.unwrap();

        // set an id, level, name, or email based on the line
        match i {
            0 =>
            {
                id = content.trim().parse::<i32>().unwrap();
            },
            1 =>
            {
                level =  content.trim().parse::<i32>().unwrap();
            }
            2 =>
            {
                name = content;
            },
            3 =>
            {
                email = content;
            },
            _ => {
                println!("Error");
            },
        }
        i = i + 1;

        // if the input is larger then 3 add the name to list and set i to zero
        if i == 4
        {
            // set user to add
            let me = User {
                id: id,
                level: level,
                name: (&name).to_string(),
                email: (&email).to_string(),
            };
        
            // insert value to list
            client.execute(
                "INSERT INTO users (id, level, name, email) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
                &[&me.id, &me.level, &me.name, &me.email],
            )?;
            i = 0;
        }
    } 

    Ok(())
}

// gives the user the ability to insert items into the database
fn insertManualData(client: &mut Client)  -> Result<(), Error>
{
    let mut run = true;

    // while the user is still adding names to the program
    while run
    {
        // get id input
        println!("Enter an id: (enter 0 to stop)");
        let mut id = String::new();
        io::stdin().read_line(&mut id).expect("failed to read line");
        let id = id.trim().parse::<i32>().expect("Invalid input");
        println!("You entered: {}", id);
        if id==0
        {
            break;
        }

        // get level input
        println!("Enter level access: (0 no access, 1 full access)");
        let mut level = String::new();
        io::stdin().read_line(&mut level).expect("failed to read line");
        let level = level.trim().parse::<i32>().expect("Invalid input");
        println!("You entered: {}", level);

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
            level: level,
            name: (&name).to_string(),               
            email: (&email).to_string(),
            
        };
    
        // insert value to list
        client.execute(
            "INSERT INTO users (id, level, name, email) VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
            &[&me.id, &me.level, &me.name, &me.email],
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
    Ok(())
}

// filters data based on content
fn filterData(client: &mut Client) -> Result<(), Error>
{
    let mut allUsers: Vec<User> = Vec::new();

    // Get information from database and output it
    for row in client.query("SELECT id, level, name, email FROM users", &[])? {
        let person = User {
            id: row.get(0),
            level: row.get(1),
            name: row.get(2),
            email: row.get(3),
        };
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

    Ok(())
}

// used to give the user the ability to query the database
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
 
     let rows = client.query("SELECT * FROM users WHERE name LIKE $1 OR email LIKE $1", &[&format!("%{}%", search_term)]).unwrap();

     if rows.is_empty() {
         println!("No rows found for term: {}", search_term);
     } else {
         for row in rows {
             let id: i32 = row.get(0);
             let level: i32 = row.get(1);
             let name: String = row.get(2);
             let email: String = row.get(3);
             println!("id: {}, level: {}, name: {}, email: {}", id, level, name.trim(), email);
         }
     }}
}

/*
    MAIN
*/
fn main() -> Result<(), Error> {


    // Connect to the database required if the program crashes this may be the main error
    let mut client = Client::connect(
        "host=localhost port=5432 user=postgres password=password dbname=test",
        NoTls,
    )?;
    
    // Configures Database to add initial data
    configure_database(&mut client);

    // Login Based on ID
    println!("Logged in to the database");
    println!("Enter your access code");
    let mut code = String::new();
    io::stdin().read_line(&mut code).expect("Failed to read line");
    let code = (code.trim()).parse::<i32>().unwrap();
    
    // Get the user Access based on id
    let level: i32 = client.query_one("SELECT level FROM users WHERE id = $1", &[&code])?.get(0);

    // connect to postgres database
    let args: Vec<String> = env::args().collect();

    // Confirm Access
    if level==1
    {
        println!("Level 1 access confirmed");
    }
    
    // If you are reading from a file
    if args.len() > 1 
    { 
        if level==1 // Full Access
        {
            // Insert Users and then Query from them
            insertFileData(&mut client);
            query(&mut client);

        }
        else if level==0 // No Access
        {
            // Only Query from existing pool of users
            println!("*** PERMISSION DENIED to open and read file ****");
            println!("Level 0 access confirmed");
            query(&mut client); 
        }
    }
    else if level==1 // manually add user information if no text file is provided
    {
        insertManualData(&mut client);
        query(&mut client);
    }
    else if level==0 // Only query existing data
    {   println!("Level 0 access confirmed");
        query(&mut client);    
    }

    // Filters data in users table
    filterData(&mut client);

    Ok(())
}