# 6. 类型转换

Rust 使用 trait 解决类型之间的转换问题。最一般的转换会用到 From 和 Into 两个 trait。不过，也有可能会遇到特别的trait，尤其是从 String 转换到别的类型，以及把别的类型转换到String类型时。

## 6.1 From 和 Into

`From` `Into` 两个 trait 内部是相关联的

### 6.2 From 

`From` trait 允许一种类型定义“怎么根据另外一种类型生成自己”，提供了一种类型转换的简单机制。

```rust 
let string = String::from("Hello ");
```

也可以自定义类型转换机制：

```rust 
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Number {
        Number{value}
    }
}

fn main() {
    let num = Number::from(2222);
    println!("{:?}", num);
}
```

- 对于任何定义的函数都要注意是否有标注返回类型
- 对于自定义的类型，打印的时候不能直接使用 {} , 要使用{:?}


### 6.3 Into 

实际上就是将 From 倒过来。如果为你的类型实现了 From，那么同时也就免费获得了Into

使用 Into trait 需要指明需要转换到的类型，因为编译器大多时候都不能准确判断它。

- 定义 From trait 的时候，也就自动获得了反向的类型转换 Into trait
- 使用 into 作为类型转换的时候，需要手动指明目标类型 let num: Number = i.into();
- From trait 访问路径是 std::convert::From

```rust
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Number {
        Number{value}
    }
}

fn main() {
    let i = 3222;
    let num: Number = i.into();

    println!("{:?}", num);
}
```


### 6.4 TryFrom and TryInto

这两个 trait 类似 From 和 Into。不过这二者用于易出错的转换，其返回值是 Result 类型

- From TryFrom 都是来自 std::convert 这个包

**问题** 
- partialEq 是用来干什么的
- type Error = ()  这是一种叫做 Placeholder Types 的用法
- derive PartialEq 是为了能够使用 assert_eq 来比较 EventNumber 这个挂载目标
- Result 作为返回值类型的时候注意书写 `Result<T, E>` 表示可以有两个结果。描述的是可能的错误


```rust 
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));
}

```

### 6.5 ToString 和 FromStr

- 要把任何类型转换成 String，只需要实现那个类型的 `ToString` trait。
  但是不要直接这么做，应该实现 `fmt::Display` trait, 它会自动提供 `ToString`,并且还可以用来打印类型。
- fmt::Display 实现的时候会自动提供 ToString Trait 
 
```rust
use std::fmt;

struct Circle {
    radius: i32
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

```

除了通过实现 fmt::Display 来使用 ToString trait，还可以直接实现 ToString trait。只是这样就没了Display带来的可以直接打印类型的好处

```rust
use std::string::ToString;

struct Circle {
    radius: i32
}

impl ToString for Circle {
    fn to_string(&self) -> String {
        format!("Circle of radius {:?}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());
}

```


