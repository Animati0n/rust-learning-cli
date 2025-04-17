use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.

#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

/// Custom  error handleing

struct CustomError(String);

/// chagpt code  

/// for handling erron differnet type.

/// `PathBuf` cannot be formatted with the default formatter; call `.display()` on it
/// .map_err(|err| CustomError(format!("Error reading `{}`: {}",&args.path,err)))?;

impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// `CustomError` doesn't implement `Debug`
/// `CustomError` cannot be formatted using `{:?}`
/// fn main()->Result<(),CustomError> {

impl std::fmt::Debug for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }

}

/// expected `Result<(), CustomError>`, found `()`
/// fn main()->Result<(),CustomError> {

impl std::error::Error for CustomError {}



fn main()->Result<(),CustomError> {

    let args = Cli::parse();

    let content= std::fs::read_to_string(&args.path)
        .map_err(|err| CustomError(format!("Error reading `{}`: {}",&args.path.display(),err)))?;

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("{}",line);
        }
    }
    Ok(())
}

