extern crate csv;

use std::error::Error;
use std::fs::File;
use std::process;

mod csv_data_frame;
use csv_data_frame::CSVDataFrame;

fn fopen(path: &str) -> Result<File, Box<dyn Error>> {
    let file = File::open(path)?;
    Ok(file)
}

fn main() {
    let file_path = "./res/sample.csv";

    let data_frame = match CSVDataFrame::new(fopen(&file_path).unwrap()) {
        Ok(data_frame) => {
            println!("Successfully parsed CSV");
            data_frame
        },
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(1);
        }
    };

    println!("{:?}", data_frame);

    for data in &data_frame.dat {
        println!("{}, {}", data.get(&data_frame.headers[0]).unwrap(), data.get(&data_frame.headers[1]).unwrap() );
    }

    println!("[Filter example]");
    // for data in data_frame.dat.iter().filter(|d| d.get("Header").unwrap_or(&"".to_string()) == "INCLUDE") {
    //     println!("{:?}", data);
    // }
    let ff: Vec<_> = data_frame.dat.iter().filter(|d| d.get("Header").unwrap_or(&"".to_string()) == "INCLUDE").collect();
    println!("{:?}", ff);
}
