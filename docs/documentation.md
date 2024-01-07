# Documentation

러스트는 문서화를 자동으로 해준다. 일반 주석(`//`)이 아닌 `///`으로 남기면 문서에 추가된다.

```rs
/// 이후 서베이 확인에 필요한 개발자 구조체
struct Developer {
  /// 실명 뿐만 아니라 닉네임도 가능합니다.
  name: String,
  /// 나이를 작성하지 않으면 술을 제공하지 않습니다.
  age: Option<i32>,
}

/// 나이 검증 함수
fn checkAge(developer: Developer) -> bool {
  /// age가 비어있으면 false 입니다.
  if (developer.age == None) {
    return false;
  }
  /// 그 외에는 모두 true 입니다.
  true
}

fn main() {
  /// 집주인
  let me = Developer {
    name: "1ilsang".to_owned(),
    age: Some(125),
  };
  /// 집주인 나이 검사
  let valid = checkAge(me);
  println!("{:?}", valid);
}
```

![example](https://github.com/1ilsang/dev/assets/23524849/5f895d90-006a-4ad7-ade1-e42f14f10c9f)

![example (2)](https://github.com/1ilsang/dev/assets/23524849/0b30cbb6-ea94-4433-8afa-7ed49ce09138)
