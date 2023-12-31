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
