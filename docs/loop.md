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

## For-in

[벡터](./vector.md)와 같은 자료형을 사용한다면 `for-in` 키워드로 순회할 수 있다.

```rs
let nums = [1, 2, 3];
for n in nums {
  println!("{:?}", n);
}
let my_nums = vec![1,2,3];
for n in my_nums {
  println!("{:?}", n);
}
```

`for-in` 함수 또한 블록 스코프에 해당하므로 [소유권](./ownerShip.md)을 가지게 된다.

```rs
let my_nums = vec![1,2,3];
for n in my_nums {
  println!("{:?}", n);
}
my_nums.len(); // Error: borrow of moved value
```

`for-in`이 실행되면서 `my_nums`의 소유권이 넘어오게 되고, 함수가 종료되면서 메모리를 해제하게 되므로 하위 라인에서 접근하면 에러가 발생하게 된다. 따라서 대여를 통한 반복을 해주어야 한다.

```rs
let my_nums = vec![1,2,3];
for n in &my_nums {
  println!("{:?}", n);
}
my_nums.len(); // 3
```
