use std::fs::File;
use std::io::{ErrorKind, Read, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);
    let filename = &args[1];
    let mut f = match File::open(filename) {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(filename) {
                Ok(mut fc) => {
                    fc.write_fmt(format_args!("Welcome to {}\n", filename)).expect("Failed to write");
                    let fc = File::open(filename).expect("failed to re-open new file");
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
    let compact_filename = format!("compact_{}", filename);
    let compact = open_file_compact(&compact_filename);
    println!("open_file_compact read: {}", &compact);
}

fn open_file_compact(filename: &str) -> String {
    let mut f = File::open(filename).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            let mut fc = File::create(filename).unwrap_or_else(|error| {
                panic!(format!("Problem creating {} {:?}", filename, error));
            });
            fc.write_fmt(format_args!("file_compact wrote {}", filename)).expect("failed to write to file after creating it");
            let fc = File::open(filename).expect("failed to read file after creating it");
            fc
        } else {
            panic!("Problem opening {} {:?}", filename, error);
        }
    });
    let mut output = String::new();
    f.read_to_string(&mut output).expect(&format!("Failed to read {}", filename));
    return output;
}
