fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("stack: {:?}", stack);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
    println!("stack: {:?}", stack);

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    let tuple = (1, 2, 3);
    let (x, y, z) = tuple;
    let (a, _, _) = tuple;
    let (q, ..) = tuple;
    println!("{}, {}, {}, {}, {}", x, y, z, a, q);

    let point = (3, 5);
    print_coordinates(&point);

    // Nonsense, since x is an irrefutable pattern
    // if let x = 5;

    // Also nonsense because Some(x) is refutable
    // let Some(x) = Some(5);


    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Ooooh, tricky: Some(y) matches Some(5) above, so y is 5
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x = 't';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };

    // verbose syntax for remapping
    let Point { x: a, y: b } = p;
    println!("a: {}, b: {}", a, b);
    assert_eq!(0, a);
    assert_eq!(7, b);

    // compact syntax when the field names match
    // the variable names:
    let Point {x, y} = p;
    println!("x: {}, y: {}", x, y);

    let p = Point { x: 0, y: 7 };

    // Match on literal values
    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(0, 160, 255);
    print_msg(msg);
    let msg = Message::Quit;
    print_msg(msg);
    let msg = Message::Move{x: 5, y: 10};
    print_msg(msg);
    let msg = Message::Write(String::from("Hello world"));
    print_msg(msg);
    let msg = Message::ChangeColorEnum(Color::Rgb(20, 30, 40));
    print_msg(msg);
    let msg = Message::ChangeColorEnum(Color::Hsv(200, 300, 400));
    print_msg(msg);

    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!("feet: {} inches: {}, x: {}, y: {}", feet, inches, x, y);
    foo(100, 200);

    let mut setting_value = Some(5); // Change to None to set setting_value below
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        }
    }

    let _x = 5;
    let y = 10;
    // Comment out the line below and you'll
    // only get an unused variable warning for y
    println!("x: {}, y: {}", _x, y);

    let s = Some(String::from("Hello!"));

    // If you use Some(_s) below, the variable is
    // bound and takes ownership away from s, so
    // the next println! using s won't compile.
    // Some(_) will match, but will not bind.
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{:?}", s);

    let origin = Point3D { x: 0, y: 0, z: 0 };
    println!("origin: {:?}", origin);

    match origin {
        Point3D { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => { // (.., second, ..) would be ambiguous and won't compile
            println!("Some numbers: {}, {}", first, last);
        }
    }

    // Match guard with additional conditional
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // Match guard with an outer variable
    let x = Some(10); // Matches the Some(n) case because 10 == y
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    // Using the match guard against a literal pattern
    let x = 4;
    let y = true;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }

    // Matching with the "@ local_var" operator
    enum HelloMessage {
        Hello { id: i32 },
    }

    let msg = HelloMessage::Hello { id: 11 };

    match msg {
        HelloMessage::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        HelloMessage::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        HelloMessage::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

fn print_msg(msg: Message) {
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!(
                "Move in the x direction {} and in the y direction {}",
                x, y
            );
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColorEnum(Color::Rgb(r, g, b)) => println!(
            "Change the color to red {}, green {}, and blue {}",
            r, g, b
        ),
        Message::ChangeColorEnum(Color::Hsv(h, s, v)) => println!(
            "Change the color to hue {}, saturation {}, and value {}",
            h, s, v
        ),
    }
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
    ChangeColorEnum(Color)
}

struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
