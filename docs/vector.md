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

## itor

```rs
let nums = vec![1, 2, 3, 4];

// Case 1.
let mut refined_nums = vec![];
for num in nums {
  refined_nums.push(num * num);
}

// Case 2.
// _ 를 사용하면 컴파일러가 타입을 추론하게 둔다.
let refined_nums: Vec<_> = nums.iter().map(|num| num * num).collect();
```

값을 순회하면서 다른 형태의 배열로 가져오고 싶다면 어떻게 해야할까? `for`와 `push`로 해결할 수도 있다.

하지만 이렇게 할 경우 코드에 "개입"할 여지가 생기고 "명령형" 코드가 된다.

선언적 코드를 작성하고 싶다면 `iter`과 `map`으로 할 수 있다.
