# Test

```rs
fn get_uppercase(word: &str) -> String {
  word.to_uppercase()
}

#[cfg(test)]
mod test {
  use crate::*;

  #[test] // 해당 함수가 테스트 코드라는 것을 컴파일러에게 알린다.
  pub fn check_uppercase() {
    let result = get_uppercase("hello");
    let expected = String::from("HELLO");
    assert_eq!(result, expected, "Should be all UPPERCASE.")
  }
}

fn main() {}
```

> $ cargo test

[모듈](./module.md)로 테스트 코드를 작성했기 때문에 `all_caps` 함수에 접근하기 위해 `use crate::*`를 선언해야 한다.

이후 `cargo test`를 실행하면 테스트 결과를 볼 수 있다.
