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


