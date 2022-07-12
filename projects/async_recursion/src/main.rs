use async_recursion::async_recursion;

struct FibInput {
  val: u32
}

#[async_recursion]
async fn fib(n : FibInput) -> u64 {
  match n.val {
      0     => panic!("zero is not a valid argument to fib()!"),
      1 | 2 => 1,
      3     => 2,
      _ => fib(FibInput{val: n.val-1}).await + fib(FibInput{val: n.val-2}).await
  }
}

#[tokio::main]
async fn main() {
  let result = fib(FibInput{val: 9}).await;
  println!("fib(9) result: {}", result);
}
