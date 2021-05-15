use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 3 {
        let query = &args[1];
        let filename = &args[2];
        println!("searching for {} in {}", query, filename)
    } else {
        usage();
    }
}

fn usage() {
    println!("Usage: minigrep query filename(s)");
}
