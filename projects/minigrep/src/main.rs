use minigrep::Config;
use std::env;
use std::process;

/// Top level minigrep function
fn main() {
    // Reconfiguring config reading for 13.3 in functional language features.
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    // let args: Vec<String> = env::args().collect();

    // let config = Config::new(&args).unwrap_or_else(|err | {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     minigrep::usage();
    //     process::exit(1);
    // });
    if let Err(e) = minigrep::run(&config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
