use std::env;
use std::process;

use mini_grep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let config = Config::new(&args);

    println!("Query: {}", config.query);
    println!("File path: {}", config.filename);

    if let Err(e) = mini_grep::run(config) {
        println!("{}", e);
        process::exit(1);
    }
}
