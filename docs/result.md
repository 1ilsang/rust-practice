# Result

러스트는 `Result` 타입이 존재한다. 해당 타입은 `성공`과 `실패`의 데이터를 가지고 있다.

```rs
enum Result<T, E> {
  Ok(T),
  Err(E)
}
```

[Option](./option.md) 타입과 같이 enum에 제너릭으로 값을 준다.

```rs
fn fetch(url: &str) -> Result<i32, String> {
  if url == "1ilsang.dev" {
    return Ok(200);
  }
  Err("NOT FOUND".to_owned())
}

fn main() {
  let result = fetch("1ilsang.dev");
  match result {
    Ok(200) => println!("200 OK"),
    Ok(other) => println!("Success {:?}", other),
    Err(e) => println!("Error! {:?}", e),
  }
}
```

에러를 `try-catch`가 아닌 결과 "값"으로 처리한다는 점이 흥미로웠다.

## with question operator

`Result`를 물음표 연산자와 함께 하면 시너지가 난다.

```rs
// ? 연산자를 사용하기 위해선 반드시 Result 타입을 반환해야 한다.
fn fetch(url: &str) -> Result<i32, String> {
  if url == "1ilsang.dev" {
    return Ok(200);
  }
  Err("NOT FOUND".to_owned())
}

fn check(url: &str) -> Result<(), String> {
  let result = fetch(url)?; // ? 연산자는 Error가 발생하면 아래 행으로 내려가지 않고 throw 한다.
  println!("{:?} OK! Hello!", result);
  Ok(()) // 빈 괄호는 "유닛" 타입이라 부르며 아무것도 반환하지 않는 것과 같다.
}

fn main() {
  match check("1ilsang.d2ev") {
    Err(e) => println!("Error! {:?}", e),
    _ => (),
  }
  // Error! "NOT FOUND"
}
```

매번 `Ok` 인지 확인하지 않고 물음표 연산자로 빠르게 함수에서 탈출할 수 있다.

단, 함수 뒤에 `?`를 사용하려면 반드시 `Result` 타입을 반환해야 한다.
