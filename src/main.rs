use std::collections::HashMap;

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
  let mut cache = HashMap::new();
  cache.insert("blog", 2);

  match cache.get("blog") {
    Some(hit) => println!("blog HIT {:?} times.", hit),
    None => println!("Empty cache."),
  }

  for (url, hitCount) in cache.iter() {
    println!("{:?}: {:?}", url, hitCount); // blog: 2
  }
}
