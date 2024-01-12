# Structure

러스트에서 구조체는 All or nothing을 가능하게 한다.

```rs
struct Box {
  item1: i32,
  item2: i32
}
// 반드시 Box 구조체의 필드를 다 채워야 함
let my_box = Box {
  item1: 1,
  item2: 123
}
```

`struct`로 선언된 구조체를 통해 변수의 형태를 강제할 수 있게 된다.

변수는 구조체의 모든 필드를 구현해야 하므로 명세와 같은 역할을 해준다.

참고로 구조체에 `fn`과 같은 함수는 선언할 수 없다. 함수도 같이 선언하려면 [impl](./impl.md)에서 구현해야 한다.

## function parameter

```rs
fn show(obj: My_struct) {
  obj.show_no();
}
fn main() {
  let my_struct = My_struct::new();
  show(my_struct); // 321
}
```

구조체 명을 타입으로 사용하면 구조체를 함수 인자로 넘길수 있다.

## Tuple structure

```rs
struct TupleStructure(i32, String, i32);
impl TupleStructure {
  pub fn new(a: i32, b: String, c: i32) -> Self {
    Self(a, b, c)
  }
}

fn main() {
  let a = TupleStructure::new(1, "2".to_owned(), 3);
  println!("{}, {}, {}", a.0, a.1, a.2);
}
```

구조체를 [튜플](./tuple.md)로 만들수 있다.
