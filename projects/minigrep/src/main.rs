use std::env;
use std::fs;
use std::process;
use std::error::Error;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err | {
        println!("Problem parsing arguments: {}", err);
        usage();
        process::exit(1);
    });
    if let Err(e) = run(&config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    println!("searching for {} in {}", &config.query, &config.filename);
    let contents =
        fs::read_to_string(&config.filename)?;
    print!("{}", contents);
    Ok(())
}

fn usage() {
    println!("Usage: minigrep query filename(s)");
}
