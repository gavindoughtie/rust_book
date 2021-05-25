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

fn main() {
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
}

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
