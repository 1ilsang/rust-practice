fn main() {
  struct Box {
    item1: i32,
  }
  // 반드시 Box 구조체의 값을 다 채워야 함
  let my_box = Box { item1: 1 };

  let item1 = my_box.item1;

  println!("{}", item1);
}
