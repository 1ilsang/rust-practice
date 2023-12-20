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
