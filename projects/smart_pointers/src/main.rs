use crate::List::{Cons, Nil};
use std::fmt;

#[derive(Debug)]
enum List<T: Default+Copy+std::fmt::Display> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T: Default+Copy+std::fmt::Display> List<T> {
  fn to_string(&self) -> String {
      match self {
          List::Cons(t, b) => format!("{} {}", t, b.to_string()),
          List::Nil => String::new()
      }
  }
}

impl<T: Default+Copy+std::fmt::Display> fmt::Display for List<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      // The `f` value implements the `Write` trait, which is what the
      // write! macro is expecting. Note that this formatting ignores the
      // various flags provided to format strings.
      write!(f, "{}", self.to_string())
  }
}

fn main() {
    let list = Cons("foo", Box::new(Cons("bar", Box::new(Cons("baz", Box::new(Nil))))));
    println!("list: {}", list);
}
