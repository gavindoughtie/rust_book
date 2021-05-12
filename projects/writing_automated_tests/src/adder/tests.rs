use adder;

#[cfg(test)]
mod tests {
  #[test]
  fn test_add_two() {
    let result = add_two(5);
    assert_eq!(result, 7);
  }
}
