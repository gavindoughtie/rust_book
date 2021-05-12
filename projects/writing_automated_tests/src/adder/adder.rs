pub fn add_two(input: i32) -> i32 {
  return input + 2;
}

#[cfg(test)]
mod tests {
  #[test]
  fn test_add_two() {
    let result = super::add_two(5);
    assert_eq!(result, 7);
  }
}
