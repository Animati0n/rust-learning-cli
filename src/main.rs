use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() {

    let args = Cli::parse();
    
    let content= std::fs::read_to_string(&args.path).expect("Could not read fle");
    
    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line);
        }
    }
}

/// Error handling methods other than  except() if file not found
///
/// Method 1
/// fn main() {

/// let args = Cli::parse();
/// let result = std::fs::read_to_string(&args.path);
/// match result {
///    Ok(content) => { println!("File content: {}", content); }
///        Err(error) => { println!("Oh noes: {}", error); }
///        }

///    for line in content.lines(){
///        if line.contains(&args.pattern){
///            println!("{}",line);
///        }
///    }
///}

/// The disadvantage of this method is that "content" is accessible beyond "match case".
/// -------------------------------------------------------------------------------------

/// Method 2
///
///
/// This method allows unwrapping founctionallity in  "match" using "{ value }"
/// and "panic" to exit(abort progarme) if error. 
///
///fn main() {

///    let args = Cli::parse();
/// let result = std::fs::read_to_string(&args.path);
/// let content = match result {
///     Ok(content) => { content },
///     Err(error) => { panic!("Can't deal with {}, just exit here", error); }
/// };
/// or
/// Short cut for above method is 
/// // let content = std::fs::read_to_string(&args.path).unwrap();
///
/// println!("file content: {}", content);
///    for line in content.lines(){
///        if line.contains(&args.pattern){
///            println!("{}",line);
///        }
///    }
///}
///
///--------------------------------------------------------------------------------------


/// Method 3
///
///
/// Dealing error  without "panic!()"  and return
///
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let args = Cli::parse();
///     let result = std::fs::read_to_string(&args.path);
///     let content = match result {
///           Ok(content) => { content },
///           Err(error) => { return Err(error.into()); }
///     };
///     println!("file content: {}", content);
///    for line in content.lines(){
///        if line.contains(&args.pattern){
///            println!("{}",line);
///        }
///    }
///     Ok(())
///   }
///   ===================================================================================



/// Method 4
///
/// You can append "?" operator to a value of type "Result", and Rust will internally expand
/// this to something very similar to the match we just wrote.
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let args = Cli::parse();
///     let result = std::fs::read_to_string(&args.path);
///     let content = std::fs::read_to_string(&args.path)?;
///         println!("file content: {}", content);
///    for line in content.lines(){
///        if line.contains(&args.pattern){
///            println!("{}",line);
///        }
///    }
///             Ok(())
///             }
///
///--------------------------------------------------------------------------------------
///
