extern crate csv;

use std::env;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::process;
use encoding_rs;

fn fopen_sjis(filename: &str) -> Result<String, Box<dyn Error>> {
    let buf = fs::read(filename)?;
    let (dec, _, _) = encoding_rs::SHIFT_JIS.decode(&buf);
    return Ok(dec.into_owned());
}

fn csv_parse<R: Read>(read: R) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .quote(b'\'')
        .has_headers(false)
        .from_reader(read);
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    let file_path = "./res/sample.csv";
    
    match csv_parse(fopen_sjis(&file_path).unwrap().as_bytes()) {
        Ok(_) => println!("Successfully parsed CSV"),
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    }
}
