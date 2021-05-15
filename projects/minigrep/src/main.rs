use std::env;
use std::fs;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config {query, filename}
    }
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config {query, filename}
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let config = Config::new(&args);
        println!("searching for {} in {}", config.query, config.filename);
        let contents = fs::read_to_string(config.filename).expect("Something went wrong reading the file");
        println!("With text:\n{}", contents);
    } else {
        usage();
    }
}

fn usage() {
    println!("Usage: minigrep query filename(s)");
}
