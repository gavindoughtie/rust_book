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

}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
