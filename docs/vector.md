# Vector

```rs
fn main() {
  let nums = [1, 2, 3];
  nums.push(123); // Error. no method name `push` found for array
}
```

JS에서는 `Array` 객체가 `push`, `pop` 등의 메서드를 가지고 있어 편리하게 자료구조로 사용한다. 러스트에서 하려면 어떻게 해야할까?

러스트에서는 "벡터"를 사용해 해결할 수 있다.

```rs
// 벡터 또한 배열 내의 모든 자료형이 일치해야 한다.
let my_nums = vec![1, 2, 3]; // 매크로를 사용한 선언
let mut my_numbers = Vec::new(); // 함수를 사용한 선언
my_numbers.push(1);
my_numbers.pop();
my_numbers.len();
```

> push 메서드를 사용할 경우 배열의 값을 변경하므로 `mut` 키워드를 선언해 이뮤터블이 아님을 알려야 한다.

벡터는 항상 순서를 유지한다.
