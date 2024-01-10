# 변수

## 변수의 선언

```rs
let ilsang = 1;
```

러스트의 변수는 `let` 키워드로 선언할 수 있다.

자바스크립트 개발자인 나로써는 변경 가능한 값의 선언이군! 이라고 이해했지만 러스트는 후술할 키워드를 작성하지 않으면 **기본적으로 "불변"이다**.

## 변수의 타입

러스트는 아래와 같은 기본 타입을 가지고 있다.

1. Boolean
2. Integer
3. Double / Float
4. Character
5. String

특이했던 점은 문자열을 쿼테이션으로 구분한다는 점이었다.

```rs
let number = 123; // Integer
let a = 'a'; // Character
let b = "abc"; // String
```

> 더 많은 타입은 [여기서 확인](https://rinthel.github.io/rust-lang-book-ko/ch03-02-data-types.html)할 수 있다.

## 변수의 변경 가능성(mutable)

기본적으로 러스트는 "불변성" 변수다. 변경가능한 변수를 만들기 위해선 `mut` 키워드를 추가해야한다.

```rs
let a = 123; // Immutable
let mut b = 123; // Mutable
```

## 상수의 선언

```rs
const MAX_COUNT: i32 = 10;

fn check_count(count: i32) -> bool {
  count < MAX_COUNT
}
```

`const`는 주로 상수의 선언으로 사용된다. 매직 넘버를 사용해야 할 경우 `const`로 상수화하는 것이 좋다.
