use bson::Document;
use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::{env, collections::btree_set::Iter};
use std::error::Error;
use tokio;
use chrono::{TimeZone, Utc};
use mongodb::bson::doc;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead, BufReader};
use rayon::prelude::*;
use futures::stream::{StreamExt, TryStreamExt};

// Used to get the input string
fn get_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to readline");
    let input = input.trim();
    println!("You entered {}\n", input);
    return (&input).to_string();
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
   // Load the MongoDB connection string from an environment variable:
   let client_uri =
      env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

   // A Client is needed to connect to MongoDB:
   // An extra line of code to work around a DNS issue on Windows:
   let options =
      ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
         .await?;
   let client = Client::with_options(options)?;

    // Print the databases in our MongoDB cluster:
    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    // Get IO
    println!("\nEnter Database to Query");
    let db = get_input();

    // Get the 'tables' collection from the 'db' database:
    let collections = client.database(&db).list_collection_names(None).await?;
    for collection in collections {
        println!("{}", collection);
    }

    // Get the table to query
    println!("\nEnter Table to Query");
    let tableName = get_input();

    // get table data and put it to a list
    let table: mongodb::Collection<Document> = client.database(&db).collection(&tableName);


    // get each key
    let result = table.find_one(None, None).await?;
    
    // find the key from the result
    match result {
        Some(document) => {
            for (key, value) in document.iter() {
                println!("{}", key);
            }
        },
        None => println!("No document found"),
    }
    println!("\nEnter Key to Query");
    let key = get_input();

    // search every value in the table
    let mut cursor = table.find(None, None).await?;
    let v: Vec<_> = cursor.try_collect().await?;
    let mut plotV: Vec<String> = Vec::new();

    for movie in v.iter() {
        tokenize(movie, &mut plotV, &key)
    }


    // Below code is hidden by comments since it only inserts for movies and no other tables
    // in future versions we will be sure to add functionality to insert for any table.
    /*
    
        let new_doc = doc! {
        "title": "Parasite",
        "year": 2020,
        "plot": "A poor family, the Kims, con their way into becoming the servants of a rich family, the Parks. But their easy life gets complicated when their deception is threatened with exposure.",
        "released": Utc.ymd(2020, 2, 7).and_hms(0, 0, 0),
        };


        
        let insert_result = table.insert_one(new_doc.clone(), None).await?;
        println!("New document ID: {}", insert_result.inserted_id);
    */
    
    // Map phase
    let intermediate_results = plotV
        .par_iter()
        .map(|line| {
            let words = line.split_whitespace();
            let mut word_counts = HashMap::new();
            for word in words {
                let count = word_counts.entry(word.to_string()).or_insert(0);
                *count += 1;
            }
            word_counts
        })
        .collect::<Vec<HashMap<String, u32>>>();

    // Reduce phase
    let mut final_results = HashMap::new();
    for word_counts in intermediate_results {
        for (word, count) in word_counts {
            let final_count = final_results.entry(word).or_insert(0);
            *final_count += count;
        }
    }
    let mut output_file = File::create("output.txt").unwrap();
    for (word, count) in &final_results {
        writeln!(output_file, "{}: {}", word, count).unwrap();
    }

    // Query phase
    let stdin = io::stdin();
    loop {
        println!("Enter a word to get its count (or \"exit\" to quit):");
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let query_word = input.trim();
        if query_word == "exit" {
            break;
        }
        match final_results.get(query_word) {
            Some(count) => println!("Count for {}: {}", query_word, count),
            None => println!("No count found for {}", query_word),
        }
    }

   Ok(())
}

// Tokenizes database elements and inserts them to a document
fn tokenize(movie: &Document, mut plotV: &mut Vec<String>, key: &str) {
    let plot = movie.get_str(key);

    match plot {
        Ok(v) => plotV.push(v.to_string()),
        Err(e) => print!(""),
    }
    
}

//set MONGODB_URI='mongodb+srv://reedhavens:reedhavens@cluster0.jbhu5b9.mongodb.net/test?retryWrites=true&w=majority'

// victor@12
// Sadler_Derek@comcast.net
// Derek
// Sadler

// user sadlerderek
// password
// Mr1EAKDg76hZ7k7X


//set MONGODB_URI='mongodb+srv://reedhavens:reedhavens@rustquickstart-123ab.mongodb.net/test?retryWrites=true&w=majority'

//mongoDB login
//email: reed.havens@wsu.edu
//password: @Kendall1
//need to do: add IP address
