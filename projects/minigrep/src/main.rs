use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let query = &args[1];
        let filename = &args[2];
        println!("searching for {} in {}", query, filename);
        let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
        println!("With text:\n{}", contents);
    } else {
        usage();
    }
}

fn usage() {
    println!("Usage: minigrep query filename(s)");
}
