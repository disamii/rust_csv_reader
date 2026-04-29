// use csv;
use std::error::Error;
use std::{env::args, path::Path};

fn read_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_path(path)?;

    for result in csv_reader.records() {
        let record = result?;
        println!("{:?}",record)
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }
    let path = Path::new(&args[1]);
    let _ = read_csv(path);
    println!("Hello, world!");
}
