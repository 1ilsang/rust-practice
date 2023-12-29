struct My_struct {
  no: i32,
}
impl My_struct {
  fn show_no(&self) {
    println!("{:?}", self.no);
  }

  fn set_no() -> Self {
    Self { no: 321 }
  }
}
fn main() {
  let my_nums = vec![1, 2, 3];
  for n in &my_nums {
    println!("{:?}", n);
  }
  my_nums.len(); // 3
}
