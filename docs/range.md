# Range

```rs
let range = 1..=3;
let range2 = 1..3;

for target in range {
  print!("{:?}", target); // 1, 2, 3
}
for target in range2 {
  print!("{:?}", target); // 1, 2
}
```

특정 숫자부터 특정 숫자까지 범위 리스트를 생성하려면 어떻게 해야 할까?

`A..B` 와 같이 범위 타입을 지정하면 된다. 이때 마지막 값에 `=`이 있다면 해당 값까지를 범위로 포함한다.

```rs
let a: Vec<_> = (1..=3).into_iter().map(|num| num * 2).collect();
println!("{:?}", a); // [2, 4, 6]

let b: Vec<_> = ('a'..='i').into_iter().map(|chr| chr.to_ascii_uppercase()).collect();
println!("{:?}", b); // ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I']
```

[itor](./vector.md#itor) 용법과 같이 사용하면 순회가 필요할 때 편리하게 사용할 수 있다.
