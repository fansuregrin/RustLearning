use std::fs;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

// `Box<dyn Error>` means the function will return a type that implements the Error trait,
// but we don’t have to specify what particular type the return value will be.
// The dyn keyword is short for “dynamic.” 
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // The `?` will return the error value from the current function for the caller to handle.
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}