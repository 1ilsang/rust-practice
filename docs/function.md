# 함수

## 함수의 선언

```rs
fn add(a: i32, b: i32) -> i32 {
  a + b // 러스트는 마지막행에 return을 생략할 수 있다. 단 생략할 경우 세미콜론이 있으면 안된다.
}
```

함수는 `fn` 키워드로 시작된다. 함수의 이름은 숫자로 시작하지 않아야 하며 예약된 키워드가 아니어야 한다.

이어지는 괄호는 매개변수를 나타낸다. 매개변수에는 콜론을 통해 타입을 명시한다.

괄호 뒤에는 리턴 타입으로 화살표 다음에 리턴되는 타입을 지정한다.