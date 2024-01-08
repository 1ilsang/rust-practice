fn fetch(url: &str) -> Result<i32, String> {
  if url == "1ilsang.dev" {
    return Ok(200);
  }
  Err("NOT FOUND".to_owned())
}

fn check(url: &str) -> Result<(), String> {
  let result = fetch(url)?;
  println!("{:?} OK! Hello!", result);
  Ok(())
}

fn main() {
  match check("1ilsang.d2ev") {
    Err(e) => println!("Error! {:?}", e),
    _ => (),
  }
}
