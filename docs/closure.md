# Closure

```rs
fn add_fn(a: i32, b: i32) -> i32 {
  a + b
}

fn main() {
  let sum = add_fn(1, 1);

  // || 를 파이프라고 부른다.
  let add_closure = |a: i32, b: i32| -> i32 { a + b };
  let add_closure_2 = |a, b| a + b; // 더 단축
  println!("{:?}", add_closure(1, 1)); // 2
}
```

클로저는 항상 다른 함수의 내부에서 사용해야 한다. 함수 파라미터 등에 유리한 문법이다.

## with map

```rs
fn add_fn(a: i32, b: i32) -> Option<i32> {
  Some(a + b)
}

fn main() {
  let sum_double = add_fn(1, 1).map(|num| num * 2);
  println!("{:?}", sum_double); // Some(4)
}
```

클로저를 적절하게 사용할 좋은 예로 `map`이 있다. [Option](./option.md) 타입은 `map`을 내장하고 있는데, 해당 구문은 함수를 받아 인자를 콜백으로 처리 해준다.

## 함수 인자로 클로저 사용하기

```rs
// 함수가 어떤 사이즈로 들어올지 모르기 때문에 dyn(dynamic) 키워드를 사용해야 한다.
// 사이즈를 모르기 때문에 힙 메모리에 올려야 한다. 따라서 Box 키워드를 사용한다.
fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
  op(a, b)
}

fn main() {
  let name = "1ilsang";
  // Box::new 를 통해 힙 메모리에 올릴 함수를 생성한다.
  // move 키워드를 사용해야 소유권을 빌릴수 있다.
  let add = Box::new(move |a, b| {
    print!("{} ", name);
    a + b
  });
  println!("{}", math(2, 2, add)); // 1ilsang 4
  // 변수가 존재함을 확인
  println!("{}", name); // 1ilsang
}
```
