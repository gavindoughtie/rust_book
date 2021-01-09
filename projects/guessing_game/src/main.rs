use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn check_guess(guess: &Guess, target: &Guess) -> bool {
    println!("You guessed: {}", guess.value());

    match guess.value().cmp(&target.value()) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            return true;
        }
    }

    return false;
}

fn main() {
    println!("Guess the number!");

    let secret_number = Guess::new(rand::thread_rng().gen_range(1, 101));

    // TODO(gavin): command-line arg to cheat
    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if check_guess(&Guess::new(guess), &secret_number) {
            break;
        }
    }
}
