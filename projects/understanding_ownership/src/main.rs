#![allow(unused)]

fn main() {
    let s = String::from("hello");
    let s1 = String::from("hello");
    if true {
        let s2 = s1;
        println!("{}", s2);
    }
    let foo = "foo";
    let bar = foo;
    println!("foo: {}", foo);
    let foo = String::from("foo");
    let bar = foo;
    // This is now a compile error
    // println!("foo: {}", foo);
    println!("bar: {}", bar);

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
    let sample = String::from("This is a long sentence");

    println!("first word: {} of {}", first_word(&sample), sample)
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
