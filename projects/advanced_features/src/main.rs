use std::ops::Add;
use advanced_features::my_vec;
use macros::some_name;

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point; // For overloading add

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn count() -> u32 {
    let count: u32;
    unsafe {
        count = COUNTER
    }
    return count;
}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}


some_name!();

fn main() {
    let i = answer();
    println!("answer() {}", i);

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let m = Millimeters(3);
    let mm = m + Meters(1);
    assert_eq!(1003, mm.0);

    println!("{}, {:?}", HELLO_WORLD, count());
    let mut num = 5u32;

    let r1 = &num as *const u32;
    let r2 = &mut num as *mut u32;
    let r3: u32;
    add_to_count(1);
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        *r2 = 0xdeadbeef;
        r3 = *r1;
    }

    println!("r3 is {}, 0x{:x}", r3, r3);
    unsafe {
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    add_to_count(1);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    println!("a {:?} b {:?}", a, b);
    println!("{}, counter: {:?}", HELLO_WORLD, count());

    let (c, d) = split_at_mut(r, 3);
    println!("c {:?}, d {:?}", c, d);

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    let my_union = MyUnion{f1: 5};
    unsafe {
        println!("my_union f1 {}", my_union.f1);
    }

    let bar = Bar::new();
    for b in bar {
        println!("b: {}", b)
    }

    let gavin = Human{};
    println!("Gavin fly!");
    gavin.fly();
    Wizard::fly(&gavin);
    Pilot::fly(&gavin);

    println!("my dog is called a {}", Dog::baby_name());
    println!("A baby dog is called {}", <Dog as Animal>::baby_name());
    let s = "barfoo".to_string(); // String::from("foobar");
    s.outline_print();
    let to_wrap = vec!["thing1".to_string(), "thing2".to_string()];
    println!("wrapped: {}", Wrapper(to_wrap));

    // Advanced types

    // This is a type alias
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);
    let to_wrap = vec!["printable1".to_string(), "printable2".to_string()];
    printable(Wrapper(to_wrap));

    type Thunk = Box<dyn Fn() + Send + 'static>;

    let f: Thunk = Box::new(|| println!("hi"));

    f();

    let fnptr_result = do_twice(add_one, 5);
    println!("fnptr_result {}", fnptr_result);

    let list_of_numbers = vec![1, 2, 3, 99];
    let list_of_strings: Vec<String> =
        list_of_numbers.iter().map(ToString::to_string).collect();
    println!("{}", Wrapper(list_of_strings));

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("{:?},{:?}", list_of_statuses, Status::Stop);

    let from_closure = returns_closure();
    println!("from_closure: {}", from_closure(5));
    
    let my_vector = my_vec![7, 8, 9];
    println!("my_vector {:?}", my_vector);
}

// Does not compile:
// fn returns_closure() -> dyn Fn(i32) -> i32 {
//     |x| x + 1
// }

// But you have to box the type
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}


#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}

// Function pointer
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

// trait aliases are experimental:
// trait DSized = ?Sized + fmt::Display;

// Sized
pub fn generic<T: ?Sized + fmt::Display>(t: &T) {
    println!("{}", t);
}

// Type alias
type Printable = Wrapper;

fn printable(p: Printable) {
    println!("printable: {}", p);
}

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "-->[{}]", self.0.join(", "))
    }
}

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl OutlinePrint for String {}

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

extern "C" {
    fn abs(input: i32) -> i32;
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();
    use std::slice;
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

unsafe fn dangerous() {
    println!("but not serious");
    let mut num = 8u32;
    let r1 = &num as *const u32;
    let r2 = &mut num as *mut u32;
    println!("r1 is: {}", *r1);
    println!("r2 is: {}", *r2);
}

unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}

#[repr(C)]
union MyUnion {
    f1: u32,
    f2: f32,
}

pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}

struct Bar {
    things: Vec<String>,
    index: usize
}

impl Bar {
    fn new() -> Bar {
        Bar {
            things: vec![String::from("Foo"), String::from("Bar"), String::from("Baz")],
            index: 0
        }
    }
}

impl std::iter::Iterator for Bar {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let result;
        if self.index < self.things.len() {
            result = Some(self.things[self.index].clone());
            self.index = self.index + 1;
        } else {
            result = None;
        }
        result
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}