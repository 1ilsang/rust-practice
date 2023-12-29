fn print(string: &str) {
  println!("{:?}", string);
}
fn main() {
  let sliced_string = "Hello, world!"; // &str
  print(sliced_string)
}
