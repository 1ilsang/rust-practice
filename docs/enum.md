# Enum

```rs
enum Name {
  Foo,
  Bar
}

let my_name = Name::Foo;
match my_name {
  Name::Foo => println!("Foo"),
  Name::Bar => println!("Bar")
}
```

`enum`은 여러 가능한 값 중 하나인 데이터 조각이다. 이 뜻은 선언이 될 때 enum의 여러 값 중 하나가 된다는 뜻이다.

또한 enum은 관심사를 묶은 데이터 구조이다. 코드를 쉽게 읽을수 있게 된다.

<details>
  <summary>Rust의 enum과 TypeScript의 enum</summary>
  
```rs
// test.rs
enum Test {
  A, // 0
  B  // 1
}
let test = Test::A;
match test {
  Test::A => println!("A"),
  Test::B => println!("B"),
  0 => println!("0") // Mismatched type exception.
}
// 'A'
```

위의 코드에서 위화감을 느낀다면 타입스크립트와 같은 "구조적 타입 시스템"에 익숙한 개발자이다.

```ts
// test.ts
enum Test {
  A, // 0
}
const a = Test.A;
if (a === 0) console.log(0);
if (a === Test.A) console.log("A");
// 0, "A"
```

타입스트립트는 덕타입(\*1) 위의 구조적 타입 시스템이기 때문에 `Test.A`의 값 `0`이 될수만 있다면 비교 변수가 어떠한 형태든 신경쓰지 않는다. 따라서 `a === 0`이든 `a === Test.A`이든 상관하지 않고 통과된다.

> \*1: 어떤 새가 오리처럼 운다면 나는 그 새를 오리라고 하겠다.

하지만 러스트는 "명목적 타입 시스템" 언어이기 때문에 그 값 `0`이 중요한게 아닌 `Test::A`라는 형태 자체가 중요하다. 따라서 `a == 0`은 타입이 전혀 다르기 때문에 비교 대상이 될 수 없다. 오직 `a == Test::A`로 비교해야만 한다.

- [구조적 타입 시스템이 더 궁금하다면?](https://1ilsang.dev/posts/book/woowa-type#%ED%83%80-%EC%96%B8%EC%96%B4%EC%9D%98-%ED%83%80%EC%9E%85-%EC%8B%9C%EC%8A%A4%ED%85%9C%EA%B3%BC-%EB%B9%84%EA%B5%90)

</details>
