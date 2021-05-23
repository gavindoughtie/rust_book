use crate::List::{Cons, Nil};
use std::fmt;
use std::ops::Deref;

#[derive(Debug)]
enum List<T: Default + Copy + std::fmt::Display> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: Default + Copy + std::fmt::Display> List<T> {
    fn to_string(&self) -> String {
        match self {
            List::Cons(t, b) => format!("{} {}", t, b.to_string()),
            List::Nil => String::new(),
        }
    }
}

impl<T: Default + Copy + std::fmt::Display> fmt::Display for List<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // The `f` value implements the `Write` trait, which is what the
        // write! macro is expecting. Note that this formatting ignores the
        // various flags provided to format strings.
        write!(f, "{}", self.to_string())
    }
}

fn main() {
    let list = Cons(
        "foo",
        Box::new(Cons("bar", Box::new(Cons("baz", Box::new(Nil))))),
    );
    println!("list: {}", list);

    // deref examples:
    let x = 5;
    let y = &x;
    let boxy = Box::new(x);
    let myboxy = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *boxy);
    assert_eq!(5, *myboxy);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
    hello(&"gavin"[1..5]);

    println!("outside of smart pointer scope");
    {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };
        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        let e = CustomSmartPointer { data: String::from("e pointer")};
        println!("custom smart pointers {} and {}, {}", c.data, d.data, e.data);
        drop(d);
    }
    println!("after CustomSmartPointers scope.");
}

#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
