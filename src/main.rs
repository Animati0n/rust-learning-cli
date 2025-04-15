use std::io::prelude::*;
use clap::Parser;
use std::fs::File;
use std::io::BufReader;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main()-> std::io::Result<()> {

    let args = Cli::parse();
    
    let file=File::open(&args.path)?;
    let reader = BufReader::new(file);
    
    println!("File \n {:?}",reader);
    
    //let mut line = String::new();
    
    let content = reader.lines();
    
    println!("Files \n {:?}",content);

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line);
        }
    }
   Ok(())
}

