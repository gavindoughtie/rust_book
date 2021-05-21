use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
  fn toString() -> String {
    String::from("???")
  }

  fn get(&self) -> T {
    match self {
      List::Cons(t, b) => T
    }
  }
}

fn main() {
    let list = Cons("foo", Box::new(Cons("bar", Box::new(Cons("baz", Box::new(Nil))))));
    println!("list: {:?}", list);
}
