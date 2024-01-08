use std::io;

// io::Result 타입은 Error를 따로 지정하지 않아도 됨
fn input() -> io::Result<String> {
  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer)?;
  Ok(buffer.trim().to_owned())
}

fn main() {
  let mut input_count = 0;
  while input_count < 3 {
    match input() {
      Ok(words) => {
        println!("{:?}", words);
        input_count += 1;
      }
      Err(e) => println!("Error, {:?}", e),
    }
  }
}
