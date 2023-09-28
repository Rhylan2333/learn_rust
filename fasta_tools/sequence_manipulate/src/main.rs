use std::env;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    // Retrieve the file name from command line arguments
    let filename = env::args()
        .nth(1) // Using the nth method to get the second parameter (file name)
        .expect("Please provide a file name as a command line argument.");

    // Open the file and read its lines
    let file = File::open(filename);
    match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                match line {
                    Ok(line) => println!("{}", line),
                    Err(err) => eprintln!("Error reading line: {}", err),
                }
            }
            Ok(())
        }
        Err(err) => {
            eprintln!("Error opening file: {}", err);
            Err(err)
        }
    }
}
