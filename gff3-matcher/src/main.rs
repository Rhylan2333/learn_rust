use std::env;

fn main() {
    println!("{}", "#".repeat(80));
    println!("####                  A mRNA-DNA or DNA-mRNA matcher tool!                  ####");
    println!("#### Powered by the Rust programming language, this command line tool       ####");
    println!("#### matches mRNA IDs to their corresponding DNA IDs (vice versa) by        ####");
    println!("#### parsing a GFF3 file. Hoping this tool can improve your efficiency :D   ####");
    println!("{}", "#".repeat(80));

    let args: Vec<String> = env::args().collect();

    let mode = &args[1];
    let file = &args[2];

    print!("Use \"{}\" mode ", mode);
    println!("for file \"{}\".", file);
    if mode == "R2D" {
        println!("R2D")
    }
    else if mode == "D2R" {
        println!("D2R")
    }
    else {
        println!("Invalid mode! only \"R2D\" or \"D2R\" is available.")
    }
}
