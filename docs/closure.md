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
