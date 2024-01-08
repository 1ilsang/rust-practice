fn add_fn(a: i32, b: i32) -> i32 {
  a + b
}

fn main() {
  let sum = add_fn(1, 1);
  // let add_closure = |a: i32, b: i32| -> i32 { a + b };
  let add_closure = |a, b| a + b;
  println!("{:?}", add_closure(1, 1));
}
