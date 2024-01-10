# Crate

Crate는 패키지와 같다. 타인이 만든 코드를 불러와 생산성을 높일 수 있다.

## 설치

랜덤 변수를 쉽게 가져오고 싶어 랜덤 라이브러리를 사용한다고 생각해보자.

[https://crates.io](https://crates.io)에서 `rand` crate를 검색한다.

```toml
# Cargo.toml
# ...

[dependencies]
rand = "0.8.5"
```

`cargo add rand` 혹은 `Cargo.toml`에 직접 붙여 넣을 수 있다.

## 사용

```rs
use rand::random;

fn main() {
  let x: i32 = random();
  println!("{}", x); // 1223058313
}
```

`use` 키워드로 rand crate를 가져와서 사용하면 끝!

## 의존성 관리

`package-lock.json`과 같이 버전 의존성은 `Cargo.lock` 파일에서 관리된다.

```toml
# Cargo.lock
[[package]]
name = "rand"
version = "0.8.5"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "34af8d1a0e25924bc5b7c43c079c942339d8f0a8b57c39049bef581b46327404"
dependencies = [
 "libc",
 "rand_chacha",
 "rand_core",
]

[[package]]
name = "rand_chacha"
version = "0.3.1"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e6c10a63a0fa32252be49d21e7709d4d4baf8d231c2dbce1eaa8141b9b127d88"
dependencies = [
 "ppv-lite86",
 "rand_core",
]

# ...
```

`rand` crate를 설치하니 `libc`, `rand_chacha`, `rand_core`에 의존성이 있다고 그래프를 그린다.

의존하고 있는 각 crate 또한 또 다른 crate에 의존하고 있을 수 있으므로 이들의 관계를 나타내는 파일이라고 보면 된다.
