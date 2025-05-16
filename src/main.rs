use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args);

    println!("Query: {}", config.query);
    println!("File path: {}", config.filename);

    if let Err(e) = run(config) {
        println!("{}", e);
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("File content: {}", contents);

    Ok(())
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
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
