// use csv;
use std::fs::File;
use std::{env::args, path::Path};
use std::{error::Error};

fn read_csv<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let csv_file= File::open(path).expect("un able to open file check the pah");
    todo!()
}

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        return;
    }
    let path=Path::new(&args[1]);
    let _= read_csv(path);
    println!("Hello, world!");
}
