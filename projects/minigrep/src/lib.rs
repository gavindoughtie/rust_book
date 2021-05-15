
use std::fs;
use std::error::Error;

pub struct Config {
  query: String,
  filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &'static str> {
      if args.len() < 3 {
          return Err("not enough arguments");
      }
      let query = args[1].clone();
      let filename = args[2].clone();
      Ok(Config { query, filename })
  }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
  println!("searching for {} in {}", &config.query, &config.filename);
  let contents =
      fs::read_to_string(&config.filename)?;
  print!("{}", contents);
  Ok(())
}

pub fn usage() {
  println!("Usage: minigrep query filename(s)");
}
