fn multi_value_return(val: i32) -> (i32, f64, u8, i32) {
    let val = if val > 60 { 300 } else { val };
    let tup: (i32, f64, u8, i32) = (500, 6.4, 1, val);
    // println!("tup: {}", tup);
    // let tup = (500, 6.4, 1, val); // Infers tuple type
    let (_, y, _, z) = tup;

    println!("The value of y is: {}, the input value is {}", y, z);
    tup // Omitting the semicolon at the end of this line means "return tup"
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    let x = x * 5; // A NEW immutable x shadowing x
    println!("The value of x is: {}", x);

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 as f64 * 30.0;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;

    // let a = [1, 2, 3, 4, 5];

    let y = {
        let x = 3;
        x + 1
    };

    // note no parens
    if y < 5 {
        println!("condition was true");
    } else if y % 2 == 0 {
        println!("y is divisible by 2");
    } else {
        println!("condition was false");
    }

    println!(
        "sum: {}, difference: {} product: {}, quotient: {}, remainder: {}, y: {}",
        sum, difference, product, quotient, remainder, y
    );

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("c {} z {} heart_eyed_cat {}", c, z, heart_eyed_cat);
    let (_, _, _, input) = multi_value_return(158);
    println!("multi_value return is {}", input);

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for (index, element) in a.iter().enumerate() {
        println!("element {} index {}", element, index);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    twelve_days();
}

fn twelve_days() {
    let verses = [
        "partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for i in 0..12 {
        carol_verse(i, &verses);
    }
}

fn count_article(count: usize) -> String {
    let articles = [
        "A", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
        "Twelve",
    ];
    articles[count].to_string()
}

fn count_order(count: usize) -> String {
    let orders = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    orders[count].to_string()
}

fn carol_verse(count: usize, verses: &[&str]) {
    let verse = verses[count];
    println!(
        "\nOn the {day} day of Christmas, my true love gave to me\n{count} {item}",
        day = count_order(count),
        count = count_article(count),
        item = verse
    );
    for number in (0..count).rev() {
        println!(
            "{and}{count} {item}",
            and = if number == 0 { "And " } else { "" },
            count = if number == 0 {
                "a".to_string()
            } else {
                count_article(number)
            },
            item = verses[number]
        );
    }
}
