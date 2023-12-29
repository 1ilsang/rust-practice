# Expression

러스트는 식에 기반한 언어이기 때문에 대부분의 것들이 "평가"되어 값을 "반환"한다.

```rs
let my_num = 3;
let is_5 = if my_num == 5 { true } else { false };
// let is_5 = my_num == 5;
let match_5 = match my_num {
  5 => true,
  _ => false
};
println!("{:?}", is_5);
```

`if`, `match` 모두 반환 값을 주고 있는 것을 확인할 수 있다.

이는 아래와 같이 조합할 수 있다.

```rs
enum Food {
  Burger,
};
let menu = Food::Burger;
let drink = "sprite";
let order = match menu {
  Food::Burger => {
    if drink == "sprite" {
      true
    } else {
      false
    }
  }
  _ => false,
};
println!("{:?}", order);
```
