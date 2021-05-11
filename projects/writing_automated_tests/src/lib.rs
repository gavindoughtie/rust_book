mod adder;
mod rect;

#[cfg(test)]
mod tests {

  #[test]
  fn exploration() {
    println!("tests in main.rs\n");
    println!("#######################\n");
    assert_eq!(2 + 2, 4);
  }

  #[test]
  fn test_add_two() {
    let result = adder::add_two(5);
    assert_eq!(result, 7);
  }
}
