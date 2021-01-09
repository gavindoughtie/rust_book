use std::fs::File;
use std::io::{ErrorKind, Read};

fn main() {
    let filename = "hello4.txt";

    // let mut f = File::open(filename).expect("couldn't open");

    let mut f = match File::open(filename) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => {
                    // fc.write_str("Welcome to ");
                    // fc.write_str(filename);
                    fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let mut hello = String::new();
    let bytes = f.read_to_string(&mut hello).expect("Failed to read file");

    print!("read {}, {} bytes:\n{}\n", filename, bytes, hello);
}
