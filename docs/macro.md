# Macro

러스트에는 매크로라는 개념이 있다.

```rs
let a = 123;
println!("Hello, World!");
println!("{:?}", a); // 123 >> :?은 디버그 모드를 뜻한다.
```

매크로는 느낌표를 이름의 마지막에 추가해 함수가 아닌 매크로임을 알린다.

[함수](./functions.md)와 비교하면, 매크로는 단순히 추가 러스트 코드를 만드는 반면 함수는 작업을 수행하고 **"평가"**한다.

|          | Macro                              | Function                                  |
| -------- | ---------------------------------- | ----------------------------------------- |
| 표현     | 느낌표를 추가해 표시               |                                           |
| 매개변수 | 개수가 유동적임                    | 개수를 반드시 지정해야 함                 |
|          | 타입이 유동적임                    | 타입을 반드시 지정해야 함                 |
| 사용     | 호출 전 정의되어 있어야 함         | 어디서든 정의하고 어디서든 호출할 수 있음 |
| 컴파일   | 러스트 코드를 작성하는 러스트 코드 | 단순 러스트 코드                          |
| 구현     | 복잡함                             | 단순함                                    |

## Derive

러스트에는 파생 매크로가 존재한다.

```rs
#[derive(Debug)] // 파생 매크로
enum Direction {
  Left,
  Right,
}

// 만약 구조체에 파생 매크로를 지정했다면 내부의 enum, struct에도 지정해야 한다.
// 여기서는 direction에 해당.
#[derive(Debug)]
struct My {
  direction: Direction,
  name: String,
}

fn main() {
  let direction = Direction::Left;
  println!("{:?}", direction); // 만약 파생 매크로를 지정하지 않으면 이렇게 출력할 수 없다.
  let me = My {
    direction,
    name: "chul".to_string(),
  };
  println!("{:?}", me); // My { direction: Left, name: "chul" }
}
```

`derive`는 `enum`과 `struct`에서 사용 가능하다.
