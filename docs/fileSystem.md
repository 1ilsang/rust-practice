# File System

러스트의 파일 시스템은 독특했다.

JavaScript의 경우 `import/export`로 어느 위치에서든 함수를 가져오거나 내보낼수 있었다.

반면 러스트는 파일의 경로가 아닌 `use NAME`이 중요하다.

```sh
src
  |-- utils
  |     |-- helpers.rs
  |     |-- math.rs
  |-- main.rs
```

위와 같은 파일 구조에서 `main.rs`가 `utils` 하위의 함수에 접근하려면 어떻게 해야할까?

```toml
# Cargo.toml
[lib]
name = "helpers"
path = "./src/utils/helpers.rs"
```

우선 `Cargo.toml`에 `lib`으로 이름을 지정한다. 나는 `helpers.rs`를 진입 포인트로 `math.rs`를 가져오도록 할 예정이다.

```rs
// src/utils/math.rs
pub fn add(a: i32, b: i32) -> i32 {
  a + b
}
```

`math.rs` 파일은 `add` 함수를 `pub` 키워드로 가지고 있다(export 없음).

```rs
// src/utils/helpers.rs
pub mod math;

pub fn ping() -> bool {
  print!("pong ");
  true
}
```

같은 구조의 `helpers.rs`에서 `mod math`로 `math.rs` 파일을 불러오고 있으며 자신또한 `ping` 함수를 퍼블릭으로 가지고 있다.

> 러스트는 동일한 구조의 파일을 [mod](./module.md) 키워드로 가져올 수 있다.
>
> `pub mod math`이므로 `math` 모듈 또한 퍼블릭으로 설정 되었다.

```rs
// main.rs
use helpers;

fn main() {
  // ping true, 3
  println!("{}, {}", helpers::ping(), helpers::math::add(1, 2));
}
```

`use helpers`는 `Cargo.toml`에 설정된 `lib name`을 의미한다. 해당 name의 path는 `src/utils/helpers.rs`를 가리키고 있으므로 `helpers.rs` 파일을 가져온 것과 같다.

이때 `helpers.rs` 파일에서 `math.rs`를 `pub mod math`로 설정했기 때문에 `main.rs`에서 `helpers::math::add`로 `math.rs`에 접근할 수 있게 된다.
