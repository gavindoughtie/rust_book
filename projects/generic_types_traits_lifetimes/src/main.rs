mod generics;

use generics::{returns_summarizable, Point, Summary, Tweet};

fn main() {
    let rant = Tweet {
        username: "@realTrump".to_string(),
        content: "madness".to_string(),
        reply: false,
        retweet: true,
    };
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

    let string_point = Point {
        x: "foo".to_string(),
        y: "bar".to_string(),
    };
    println!("string_point: {}", string_point);
    println!("summary: {}", &returns_summarizable().summarize());

    // let mut points = [Point{x: 5.0, y: 0.0}, Point{x: 15.0, y: 0.0}, Point{x: 225.0, y: 0.0}];
    // points.sort_unstable();
    // println!("sorted points: {:#?}", points);

    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);

    println!("The largest char is {}", result);

    // TODO(gavin.doughtie): How can we get the compiler to
    // fail if we try to pass an empty list?
    // let empty_list: Vec<char> = vec![];
    // let result = largest(&empty_list);
    // println!("The largest element in the empty list is {}", result);

    // Lifetimes
    {
        let r;

        {
            let x = 5;
            r = &x;
            println!("r: {}", r);
        }

        //println!("r: {}", r);
    }

    let string1 = String::from("long string is long");
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);

    let l = longest_with_an_announcement("foo", "foobar", "this is the announcement");

    println!("Longest: {}", l);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
