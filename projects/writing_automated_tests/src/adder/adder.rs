pub fn add_two(input: i32) -> i32 {
  return input + 2;
}


#[test]
fn test_add_two() {
  let result = add_two(5);
  assert_eq!(result, 7);
}
