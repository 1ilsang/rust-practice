# 문자열

문자열에서 가장 기본은 `String`과 `&str` 타입이다.

## String

`String` 타입은 "소유권"을 가지고 있는 스트링을 의미한다.

```rs
struct MyStruct {
  name: String, // String 타입이므로 name 값을 "소유"한다.
}
fn main() {
  let owned_string = "owned string".to_owned(); // String
  let sliced_string = "Hello, world!"; // &str
  let my_struct = MyStruct { name: owned_string };
  // let my_struct = MyStruct { name: sliced_string }; // type error!
}
```

## &str

문자열 슬라이스인 `&str` 타입은 "대여"한 스트링을 의미한다([소유권](./ownerShip.md) 참고).

```rs
fn print(string: &str) {
  println!("{:?}", string);
}
fn main() {
  let sliced_string = "Hello, world!"; // &str
  print(sliced_string)
}
```
