use crate::List::{Cons, Nil};
use crate::RcList::{RcCons, RcNil};
use crate::RcCList::{RcCCons, RcCNil};
use crate::CycleList::{CycleCons, CycleNil};

use std::fmt;
use std::ops::Deref;
use std::rc::Rc;

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

impl RcList {
    fn to_string(&self) -> String {
        match self {
            RcList::RcCons(t, b) => format!("{} {}", t, b),
            RcList::RcNil => String::new(),
        }
    }
}

impl fmt::Display for RcList {
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
        let e = CustomSmartPointer {
            data: String::from("e pointer"),
        };
        println!(
            "custom smart pointers {} and {}, {}",
            c.data, d.data, e.data
        );
        drop(d);
    }
    println!("after CustomSmartPointers scope.");

    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    {
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = RcCons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        println!("a: {}, b: {}, c: {}", a, b, c)
    }
    println!("count after exiting scope: {}", Rc::strong_count(&a));

    // RefCell examples:
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RcCCons(Rc::clone(&value), Rc::new(RcCNil)));

    let b = RcCCons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RcCCons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    // Memory leak with cycles:
    println!("Cycles that create a memory leak....");
    let a = Rc::new(CycleCons(5, RefCell::new(Rc::new(CycleNil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    println!("a rc count after calling .tail() = {}", Rc::strong_count(&a));

    let b = Rc::new(CycleCons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle;
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());
}

use std::cell::RefCell;

#[derive(Debug)]
enum RcCList {
    RcCCons(Rc<RefCell<i32>>, Rc<RcCList>),
    RcCNil,
}

#[derive(Debug)]
enum CycleList {
    CycleCons(i32, RefCell<Rc<CycleList>>),
    CycleNil,
}

impl CycleList {
    fn tail(&self) -> Option<&RefCell<Rc<CycleList>>> {
        match self {
            CycleCons(_, item) => Some(item),
            CycleNil => None,
        }
    }
}

#[derive(Debug)]
enum RcList {
    RcCons(i32, Rc<RcList>),
    RcNil,
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
