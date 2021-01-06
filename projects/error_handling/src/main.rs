use std::fs::File;
use std::io::Read;

fn main() {
    println!("Hello, error_handling!");
    let f = File::open("./src/hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut hello = String::new();
    
    let bytes = f.read_to_string(&mut hello).expect("Failed to read file");

    print!("read {}:\n{}\n", bytes, hello);
}
