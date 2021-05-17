//! # Minigrep
//!
//! `minigrep` is a minimal file grep implementation.
use std::env;
use std::error::Error;
use std::fs;

/// Configuration for grepping; represents the
/// command-line params and any environment variables
/// set when minigrep is invoked.
pub struct Config {
  /// String being searched for
  pub query: String,
  /// file to search
  pub filename: String,
  /// whether the search should be case-sensitive
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    args.next(); // Skip the executable name

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a query string"),
    };

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };
    // if args.len() < 3 {
    //   return Err("not enough arguments");
    // }
    // let query = args.next().unwrap();
    // let filename = args.next().unwrap();
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(&config.filename)?;
  let results;
  if config.case_sensitive {
    results = search(&config.query, &contents);
  } else {
    results = search_case_insensitive(&config.query, &contents);
  }
  for line in results {
    println!("{}", line);
  }
  Ok(())
}

pub fn usage() {
  eprintln!("Usage: minigrep query filename(s)")
}

/// Search for a string in some text
/// # Examples
/// ```
/// let query = "foo";
/// let contents = "foo bar";
/// assert_eq!(vec!["foo bar"], minigrep::search(&query, &contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  // 13.3 functional language features
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
  // let mut results: Vec<&'a str> = Vec::new();
  // for line in contents.lines() {
  //   // do something with line
  //   if line.contains(query) {
  //     results.push(line);
  //   }
  // }
  // results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results: Vec<&'a str> = Vec::new();
  let query = query.to_lowercase();
  for line in contents.lines() {
    // do something with line
    if line.to_lowercase().contains(&query) {
      results.push(line);
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    assert_eq!(vec!["safe, fast, productive."], search(&query, &contents));
  }

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}
