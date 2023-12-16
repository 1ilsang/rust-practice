# 반복

## Loop

러스트에서는 `loop` 키워드를 활용해 반복되는 작업을 할 수 있다.

```rs
let mut a = 0;
loop {
  // 조건문에 괄호가 없다
  if a == 5 {
    break;
  }
  println!("{:?}", a);
  a = a + 1;
}
println!("Exit!");
// 0, 1, 2, 3, 4, Exit!
```

## While

`loop` 문이 단순 반복이라면 `while`문은 조건에 따른 반복을 할 수 있다.

```rs
let mut a = 0;
while a != 5 {
  println!("{:?}", a);
  a = a + 1;
}
println!("Exit!");
// 0, 1, 2, 3, 4, Exit!
```
