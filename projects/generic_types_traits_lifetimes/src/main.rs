mod generics;

use generics::{Tweet, Point};

fn main() {
    let rant = Tweet{username: "@realTrump".to_string(), content: "madness".to_string(), reply: false, retweet: true};
    println!("rant: {}", rant);
    let both_integer = Point { x: 5, y: 10 };
    println!("x: {} y: {}", both_integer.x(), both_integer.y);
    let both_float = Point { x: 1.0, y: 4.0 };
    println!("distance: {}", both_float.distance_from_origin());
    let integer_and_float = Point { x: 5, y: 4.0 };
    println!("x: {} y: {}", integer_and_float.x(), integer_and_float.y());
    println!("both_float {}", both_float);
    let mix = both_integer.mixup(both_float);
    println!("mixup {} {}", mix.x(), mix.y());
    let _nothing = Option::None::<i32>;
    let _something = Option::Some(5);
    let _something_else = Option::Some("This is a string");
    // _something_else is a string option and can't be put in here.
    let _foo = [_nothing, _something];
}
