# Impl

```rs
struct My_struct {
  no: i32,
}
fn show_no(data: My_struct) {
  println!("{:?}", data.no);
}
fn main() {
  let my_struct = My_struct { no: 123 };
  show_no(my_struct);
}
```

[구조체](./structure.md)를 사용할때 위와 같이 사용할 수 있다. 하지만 구조체 자체의 응집도를 더 높이고 싶다면 `impl`을 사용하면 된다.

```rs
struct My_struct {
  no: i32,
}
impl My_struct {
  fn show_no(&self) {
    println!("{:?}", self.no);
  }
}
fn main() {
  let my_struct = My_struct { no: 123 };
  my_struct.show_no();
}
```

인자로 넘어오는 구조체는 `&self`를 통해 동일한 이름의 구조체 [소유권을 대여](./ownerShip.md)해 접근할 수 있다. `show_no` 함수가 `impl`안에 들어가면서 `My_struct` 구조체의 응집도가 훨씬 강해진 것을 볼 수 있다.

```rs
struct My_struct {
  no: i32,
}
impl My_struct {
  fn set_no() -> Self {
    Self { no: 321 }
  }
}
fn main() {
  let my_struct = My_struct::set_no();
  my_struct.show_no(); // 321
}
```

만약 구조체를 동적으로 선언하고 싶다면 `impl`에서 위와 같이 하면 된다. `Self` 키워드는 자기 자신을 반환하므로 `My_struct` 구조체를 의미하게 된다.

> NOTE: `my_struct.set_no()`는 되지 않는다. 구조체의 함수로 사용되고 싶다면 `&self` 파라미터가 반드시 있어야 한다.
