# Trait

Trait은 특정 함수의 존재를 명시하는 문법이다. 여러 타입에 걸쳐 기능을 공유할때 사용하면 용이하다.

```rs
// trait으로 Foo의 구현체는 get_no 함수가 있어야 함을 명시
trait Foo {
  fn get_no(&self) -> i32;
}

// impl TRAIT for STRUCT는 구조체 Bar의 구현을 Foo로 한다는 뜻이다.
struct Bar;
impl Foo for Bar {
  fn get_no(&self) -> i32 {
    123
  }
}

struct BarBar;
impl Foo for BarBar {
  fn get_no(&self) -> i32 {
    321
  }
}

fn main() {
  let bar = Bar {};
  let barbar = BarBar {};
  println!("{:?}", bar.get_no()); // 123
  println!("{:?}", barbar.get_no()); // 321
}
```

[impl](./impl.md)을 묶어 [구조체](./structure.md) 타입 강제를 해주기 좋다.

## function parameter

함수 인자에 trait 타입을 적용하려면 어떻게 해야할까? [구조체의 함수 파라미터](./structure.md#function-parameter)와 같이 가능할까?

```rs
// ... 위와 상동

// fn no(obj: Bar) -> i32 { // OK
// fn no(obj: Foo) -> i32 { // Error! trait objects must include the `dyn` keyword
fn no(obj: impl Foo) -> i32 { // OK
  obj.get_no()
}
```

위 주석을 보면 `Bar`의 구조체로 선언하거나 `impl Foo`로 trait에 impl 키워드를 작성해야만 한다.

이는 "구현된 Foo trait"만 와야 하기 때문이다.
