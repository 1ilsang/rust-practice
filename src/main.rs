fn add(a: i32, b: i32) -> i32 {
  return a + b;
}

fn main() {
  let a = add(1, 2);
  let mut b = 123;
  b = 321;
  println!("{:?}", b)
}
