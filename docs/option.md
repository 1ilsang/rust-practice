# Option

데이터가 있을 수도 있고 없을 수도 있는 경우 어떻게 표현할까? 이때 바로= `Option`을 사용하면 된다.

```rs
enum Option<T> {
  Some(T),
  None
}
```

Option은 위와 같이 구현되어 있다.

```rs
let me = Developer {
  name: "1ilsang".to_owned(),
  age: Some(216),
};
let you = Developer {
  name: "you".to_owned(),
  age: None,
};
match me.age {
  Some(16) => println!("Hello, 16!"),
  Some(age) => println!("Age is {:?}", age),
  None => println!("Age is empty"),
}
```

Option의 `Some`과 `None`의 경우 앞을 생략해도 된다(`Option::Some(..)` 안해도 됨).
