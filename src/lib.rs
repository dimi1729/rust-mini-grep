use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("File content: {}", contents);

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Config {
        // args[0] is the file thats actually running, 1, 2, ... are the user arguments
        // This means we still need at least 3 args
        assert!(
            args.len() >= 3,
            "Not enough arguments were provided, at least 3 are needed"
        );
        let query = args[1].clone();
        let filename = args[2].clone();

        return Config {
            query: query,
            filename: filename,
        };
    }
}
