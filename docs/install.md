# Rust 초기 설정

## Install

[공식문서](https://www.rust-lang.org/tools/install)에 따르면 한줄이면 가능하다.

```sh
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

<img width="439" alt="image" src="https://github.com/1ilsang/dev/assets/23524849/030f92b2-94d0-4f33-8e86-86cc5c8dd299" />

cargo가 정상적으로 설치된 모습.

## Settings

```sh
$ cargo init [PATH]
```

cargo를 활용해 프로젝트를 간편하게 만들수 있다.

init 명령 이후에는 `src/main.rs`와 `Cargo.lock`, `Cargo.toml` 파일이 생기며 `main.rs`를 entry-point로 빌드한다.

## Build and Start

<img width="450" alt="image" src="https://github.com/1ilsang/dev/assets/23524849/b2b3d038-6a96-4fb9-9ab1-9019442b4393" />

```sh
$ cargo build
$ cargo run
```

빌드를 진행하면 `target/...` 폴더로 결과물이 생성되며 실행할 경우 빌드된 결과물을 실행시켜준다.
