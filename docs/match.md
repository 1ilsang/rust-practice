# 조건

## If

```rs
let a = 123;
if a == 123 {
  println!("Hit!");
} else {
  printlnt!("None");
}
```

러스트에서는 조건문에 괄호가 존재하지 않는다.

## Match

```rs
let my_bool = false;
// 만약 false를 지정하지 않으면 에러가 발생한다.
// missing match arm: `false` not covered
match my_bool {
  true => println!("true"),
  // false => println!("false"),
}
```

match는 *조건에 해당하는 모든 경우를 나열*해야 한다([Exhaustiveness checking](https://1ilsang.dev/posts/book/woowa-type#%EC%8B%A4%EC%9A%A9%EC%A0%81%EC%9D%B8-%EC%98%88%EC%A0%9C)). 또한 각 행은 세미콜론이 아닌 컴마로 끝난다.

`match`는 **문이 아닌 표현식에 해당**한다.

```rs
let my_int = 3;
match my_int {
  1 => println!("1"),
  2 => println!("2"),
  _ => println!("something else..."),
}
```

정수와 같이 모든 경우를 나열할 수 없을 경우 나머지 조건을 `_` 언더바로 표현한다.

`if`와의 차이는 아래와 같다.

|      | Match                                                          | If-else                                                     |
| ---- | -------------------------------------------------------------- | ----------------------------------------------------------- |
| 조건 | 모든 가능성을 나열해야 한다. 그렇지 않으면 컴파일 되지 않는다. | 모든 가능성을 나열하지 않아도 된다. 필요한 조건만 나열한다. |
