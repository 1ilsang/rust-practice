# HashMap

해시맵은 key-value로 이루어진 컬렉션이다. O(1)만에 값을 찾을 수 있는 유용한 자료구조다.

```rs
use std::collections::HashMap; // 해시맵을 사용하기 위해선 상단에 선언해야 한다.

let mut cache = HashMap::new();
cache.insert("blog", 2);

match cache.get("blog") {
  Some(hit) => println!("blog HIT {:?} times.", hit),
  None => println!("Empty cache."),
}
```

값을 변경할 수 있어야 하므로 `mut` 키워드를 항상 넣어주어야 한다.

## Loop

[반복문](./loop.md)에서 해시맵을 편하게 사용할 수 있다.

```rs
for (url, hitCount) in cache.iter() {
  println!("{:?}: {:?}", url, hitCount); // blog: 2
}
```

이 외에도 `.keys()`, `.values()` 메서드를 활용해 키와 값을 편리하게 가져올 수 있다.

[Vector](./vector.md)는 "순서가 보장"되지만 해시맵은 "순서가 보장되지 않는다".
