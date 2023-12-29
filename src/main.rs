fn main() {
  enum Food {
    Burger,
  };
  let menu = Food::Burger;
  let drink = "sprite";
  let order = match menu {
    Food::Burger => {
      if drink == "sprite" {
        true
      } else {
        false
      }
    }
    _ => false,
  };
  println!("{:?}", order);
}
