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
  let set_no = My_struct::set_no();
  set_no.show_no();
}
