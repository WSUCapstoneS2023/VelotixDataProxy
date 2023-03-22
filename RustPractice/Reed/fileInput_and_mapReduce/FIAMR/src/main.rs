use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::{self, prelude::*, BufReader};
use std::thread;
use std::thread::JoinHandle;


fn main() {
    println!("Hello, world!");
    //-> io::Result<()> 

    //readByLine("data.txt");
    //readEntire("data.txt");

    mapSum("data.txt")

}

fn readEntire(file: &str) -> String {
    
    let path = Path::new("data.txt");
    let display = path.display();

    //Open file with error checking
    let mut file = match File::open(&path){
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    //Read from file with error checking 
    let mut s = String::new();
    
    match file.read_to_string(&mut s) {
        Err(why) => panic!("Couldn't read {}: {}", display, why),
        Ok(_) => println!("{} contains: ", display),
    };

    //println!("{}", s)

    

    return s;

    //Ok(());
}

fn readByLine(file: &str) -> io::Result<()> {
    let path = Path::new(&file);
    let display = path.display();
    let mut file1 = match File::open(&path){
        Err(why) => panic!("Couldn't open {}: {}", display, why),
        Ok(file) => file,
    };
    let reader = BufReader::new(file1);

    for line in reader.lines() {
        println!("Line: {}\n", line?);
    }

    Ok(())
}

fn mapSum(file: &str) {
    
    let mut children = vec![];

    let data = readEntire(file);
    //println!("{}", data);
    let mut chunked_data = String::new();

    chunked_data = data.split_whitespace();
   
    let c: String = chunked_data.clone().collect();
    //let cd1 = chunked_data;

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment:
            let result = data_segment
                        // iterate over the characters of our segment..
                        .chars()
                        // .. convert text-characters to their number value..
                        .map(|c| c.to_digit(10).expect("should be a digit"))
                        // .. and sum the resulting iterator of numbers
                        .sum();

            // println! locks stdout, so no text-interleaving occurs
            println!("processed segment {}, result={}", i, result);

            // "return" not needed, because Rust is an "expression language", the
            // last evaluated expression in each block is automatically its value.
            result

    }));


    }

}