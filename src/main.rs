mod math {
  pub fn add(a: i32, b: i32) -> i32 {
    a + b
  }
}

fn main() {
  // use math::*;
  // let sum = add(1, 2);
  let sum = math::add(1, 2);
}
