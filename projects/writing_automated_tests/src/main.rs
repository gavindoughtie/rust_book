mod adder;
mod rect;

fn main() {
  println!("automated tests");
}

#[cfg(test)]
mod tests {

  #[test]
  fn exploration() {
    println!("tests in main.rs\n");
    println!("#######################\n");
    assert_eq!(2 + 2, 4);
  }
}
