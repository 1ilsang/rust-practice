fn add_fn(a: i32, b: i32) -> Option<i32> {
  Some(a + b)
}

fn main() {
  let sum_double = add_fn(1, 1).map(|num| num * 2);
  println!("{:?}", sum_double);
}
