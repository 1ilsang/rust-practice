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
