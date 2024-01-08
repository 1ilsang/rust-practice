# Module

```rs
mod math {
  // 외부에서 사용하려면 pub 키워드로 정의한다.
  pub fn add(a: i32, b: i32) -> i32 {
    a + b
  }
}

fn main() {
  // use math::*; // *로 모듈에 있는 값을 가져올 수 있음
  // let sum = add(1, 2);
  let sum = math::add(1, 2);
}
```

`mod` 모듈은 별개의 스코프로 처리된다. 따라서 `add`로 바로 접근할 수 없다.

모듈로 구분해서 파일을 나눈것처럼 사용할 수 있다.
