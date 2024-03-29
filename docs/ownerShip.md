# OwnerShip

Rust는 '소유권' 모델을 통해 메모리 안정성을 높힌 언어이다.

소유권은 **메모리의 소유자가 메모리를 정리하게 됨**을 의미하며 소유자란 함수이다. 메모리는 소유자로부터 이동하거나 대여할 수 있다.

```rs
enum Name {
  Chul,
}

fn my_fn(data: Name) {
  match data {
    Name::Chul => println!("Hi"),
    _ => println!("Exit"),
  }
}
fn main() {
  let my_data = Name::Chul;
  my_fn(my_data);
  my_fn(my_data); // Error: use of moved value
}
```

위의 코드는 알반적으로 전혀 문제가 없어 보이지만 `my_fn` 함수를 두 번째 호출 했을 때에는 에러가 발생한다.

이유는 `main` 함수의 `my_data`의 소유권이 `my_fn`으로 넘어갔고 "메모리의 소유자가 메모리를 정리해야 하는" 원칙으로 첫 번째 `my_fn` **함수가 종료 될 때** `my_data` 변수가 메모리에서 지워져 버렸기 때문이다.

따라서 두 번째 `my_fn` 함수를 호출하면 사용된 변수를 호출했다고 에러가 발생한다. 상당히 흥미로운 부분이다.

```rs
fn my_fn(data: &Name) {
  match data {
    Name::Chul => println!("Hi"),
    _ => println!("Exit"),
  }
}
fn main() {
  let my_data = Name::Chul;
  my_fn(&my_data);
  my_fn(&my_data);
  // Hi Hi
}
```

"메모리는 소유자로부터 이동하거나 대여할 수 있다"고 했다. 이동을 하게 되면 다음 소유권자가 삭제해야 하지만 대여를 하게 되면 정리하지 않아도 된다.

따라서 `&` 표시를 앞에 붙여 해당 데이터를 대여(참조)한다고 표시해 위의 문제를 해결할 수 있다.

> 또한 메모리의 소유권이 넘어갈 때 데이터의 값을 복사해서 가져가므로 비효율적이다. 대여를 사용하면 데이터가 한곳에 머물러 있기 때문에 더 효율적인 동작을 하게 된다.

```rs
fn sub() {
  let my_sub_data = 123;
}
let main() {
  let my_main_data = 123;
}
```

**메모리의 소유는 "선언"되었을 때 정해진다.** 따라서 `my_sub_data`는 `sub` 함수가 소유하고 있으며 `my_main_data`는 `main` 함수가 소유하고 있다.

## with filter

```rs
let a: Option<i32> = Some(1);
let a_filtered = a.filter(|num| num == 1); // Error! can't compare `&i32` with `{integer}`
let a_filtered = a.filter(|num| num == &1); // OK
```

[클로저](./closure.md)에서 값을 비교할 때도 소유권을 신경 써야 한다. 위 예에서는 filter 함수를 사용할 때 원소 `num`을 빌려와서 쓰므로 타입을 맞춰야 한다.

## Lifetime

```rs
#[derive(Debug)]
enum Answer {
  Yes,
  No,
}

#[derive(Debug)]
struct Form {
  question: &Answer, // Error! missing lifetime specifier.
}

fn main() {
  let form;
  {
    let answer = Answer::Yes;
    form = Form { question: &answer };
  }
  println!("{:?}", form);
}
```

위 코드는 정상적으로 실행되지 않는다. 중괄호가 끝나면서 `answer`은 삭제되지만 `form` 변수에서 `answer` 값을 계속 빌려야 하기 때문이다.

따라서 이러한 경우를 체크하기 위해 수명 주석을 추가해야 한다.

```rs
#[derive(Debug)]
enum Answer {
  Yes,
  No,
}

// 수명이 필요한 주석은 'abc... 등으로 작성한다.
#[derive(Debug)]
struct Form<'a> {
  question: &'a Answer,
}

fn main() {
  let form;
  {
    let answer = Answer::Yes;
    form = Form { question: &answer }; // Error! answer does not live long enough.
  }
  println!("{:?}", form);
}
```

이제 정상적으로 에러가 노출된다. `answer`를 중괄호 밖으로 가져가야 한다.
