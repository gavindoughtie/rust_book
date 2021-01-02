use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    let bytes_read = io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    println!(
        "Which is {str_len} long and you typed {bytes_read} bytes",
        bytes_read = bytes_read,
        str_len = guess.len()
    );
}
