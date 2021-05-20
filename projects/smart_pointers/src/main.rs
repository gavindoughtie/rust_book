use crate::List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl List<T> {
  fn toString() -> String {
    return self::Cons
  }

  fn get() -> T {
      return self::Cons[0]
  }
}

fn main() {
    let list = Cons("foo", Box::new(Cons("bar", Box::new(Cons("baz", Box::new(Nil))))));
    println!("list: {:?}", list);
}
