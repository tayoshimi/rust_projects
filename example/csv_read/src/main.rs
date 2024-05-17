extern crate csv;

use std::env;
use std::error::Error;
use std::fs::File;
use std::process;

fn fopen(path: &str) -> Result<File, Box<dyn Error>> {
    let file = File::open(path)?;
    Ok(file)
}

fn csv_parse(file: File) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .quote(b'\'')
        .has_headers(false)
        .from_reader(file);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let file_path = "./res/sample.csv";
    
    match csv_parse(fopen(&file_path).unwrap()) {
        Ok(_) => println!("Successfully parsed CSV"),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}
