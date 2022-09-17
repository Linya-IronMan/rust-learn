# 8. 流程控制
## 8.1 if/else

Rust 中的 不二判断不必使用小括号包裹，并且每个条件后面都跟着一个代码块。

## 8.2 loop 循环

这是一种无限循环，可以使用 `continue` 跳过循环体的剩余部分并开始下一轮循环。

```rust 
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // 无限循环
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // 跳过这次迭代的剩下内容
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // 退出循环
            break;
        }
    }
}

```

### 8.2.1 嵌套循环和标签

- loop 循环可以多层嵌套
- loop 多层嵌套的时候，为了区分continue 或者 break 到底是中断的哪个循环，需要使用一些标签来说明

```rust
#![allow(unreachable_code)]

fn main() {
    'outer: loop {jj
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // 这只是中断内部的循环
            //break;

            // 这会中断外层循环
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}

```

### 8.2.2 从 loop 循环返回

- loop 循环后面跟着一个代码块，它是可以有返回值的，不过它的返回值要放在 break 后面

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}

```

## 8.3 while 循环

while 是条件满足时的循环

```rust 
fn main() {
    // 计数器变量
    let mut n = 1;

    // 当 `n` 小于 101 时循环
    while n < 101 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // 计数器值加 1
        n += 1;
    }
}
```


## 8.4 for 循环和 区间

### 8.4.1 for 与区间
- `for in` 结构可以遍历一个迭代器。
- `a..b` 使用区间标记，可以生成一个 从 a 到 b 步长为1 的一系列值。注意，这个区间会包含 a 但是不会包含 b
  for n in 1..101
- a..=b 则可以将两端都包含在内的范围

### 8.4.2 for 与 迭代器

- 如果没有特别的指定，for 循环会对给出的集合应用 `into_iter` 函数，将其转换成一个迭代器。
  这并不是将集合变成迭代器的唯一方法，其他的还有 `iter` 以及 `iter_mut` 
- iter: 在每次迭代中借用集合中的一个元素。集合本身不会被改变，循环之后仍可以使用
- into_iter: 会消耗集合。在每次迭代中，集合中的数据本身会被提供。一旦集合被消耗了，之后就无法再使用了，因为它已经在循环中被 move 了
- iter_mut: 可变地借用集合中的每个元素，从而允许集合被就地改变

## 8.5 match 匹配

- 类似 switch，要求所有可能值都必须被覆盖
- match 代码块能够以多种方式解构物件
  - 元组
  - 枚举
  - 指针
  - 结构体

### 8.5.1 解构

#### 8.5.1.1 元组


- `_` 表示不将值绑定到变量
- `..` 表示用来忽略元组的其他部分

```rust
fn main() {
    let triple = (0, -2, 3);
    // 试一试 ^ 将不同的值赋给 `triple`

    println!("Tell me about {:?}", triple);
    // match 可以解构一个元组
    match triple {
        // 解构出第二个和第三个元素
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..)  => println!("First is `1` and the rest doesn't matter"),
        // `..` 可用来忽略元组的其余部分
        _      => println!("It doesn't matter what they are"),
        // `_` 表示不将值绑定到变量
    }
}
```

#### 8.5.1.2 枚举


```rust 
// 需要 `allow` 来消除警告，因为只使用了枚举类型的一种取值。
#[allow(dead_code)]
enum Color {
    // 这三个取值仅由它们的名字（而非类型）来指定。
    Red,
    Blue,
    Green,
    // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    let color = Color::RGB(122, 17, 40);
    // 试一试 ^ 将不同的值赋给 `color`

    println!("What color is it?");
    // 可以使用 `match` 来解构 `enum`。
    match color {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}!", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
                c, m, y, k),
        // 不需要其它分支，因为所有的情形都已覆盖
    }
}

```


#### 8.5.1.3 指针和引用

- 通过 ref 创建引用: let ref a = 5
- let ref mut a = 5;
- ref 更改的是赋值行为，从而可以对具体值创建引用
- 传入的时候使用引用，但是仍可以通过解引用的方式来修改值

```rust
let mut mut_value = 6;
// 类似地使用 `ref mut`。
match mut_value {
    ref mut m => {
        // 已经获得了 `mut_value` 的引用，先要解引用，才能改变它的值。
        *m += 10;
        println!("We added 10. `mut_value`: {:?}", m);
    },
}
```

#### 8.5.1.4 结构体

- 结构体的解构：`let Foo {x: (a, b), y} = foo`

```rust 
fn main() {
    struct Foo { x: (u32, u32), y: u32 }

    // 解构结构体的成员
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;

    println!("a = {}, b = {},  y = {} ", a, b, y);

    // 可以解构结构体并重命名变量，成员顺序并不重要

    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);

    // 也可以忽略某些变量
    let Foo { y, .. } = foo;
    println!("y = {}", y);

    // 这将得到一个错误：模式中没有提及 `x` 字段
    // let Foo { y } = foo;
}

```

### 8.5.2 match 卫语句

这是一种过滤分支的方式。

```rust 
fn main() {
    let pair = (2, -2);
    // 试一试 ^ 将不同的值赋给 `pair`

    println!("Tell me about {:?}", pair);
    match pair {
        (x, y) if x == y => println!("These are twins"),
        // ^ `if` 条件部分是一个卫语句
        (x, y) if x + y == 0 => println!("Antimatter, kaboom!"),
        (x, _) if x % 2 == 1 => println!("The first one is odd"),
        _ => println!("No correlation..."),
    }
}
```

### 8.5.3 绑定

- 绑定的  @ 语法
- 1..=12 表示 数字在 1 到 12 这个范围内

match 中如果间接地访问一个变量，则不经过重新绑定就无法在分支中再使用它。

这里的 n 表示将 age() 的执行结果暂时缓存了起来，后面执行的时候，就不用再将age() 再执行一遍了

应该也可以提前将 age() 缓存起来，然后再通过match进行匹配 


```rust
// `age` 函数，返回一个 `u32` 值。
fn age() -> u32 {
    15
}

fn main() {
    println!("Tell me what type of person you are");

    match age() {
        0             => println!("I haven't celebrated my first birthday yet"),
        // 可以直接匹配（`match`） 1 ..= 12，但那样的话孩子会是几岁？
        // 相反，在 1 ..= 12 分支中绑定匹配值到 `n` 。现在年龄就可以读取了。
        n @ 1  ..= 12 => println!("I'm a child of age {:?}", n),
        n @ 13 ..= 19 => println!("I'm a teen of age {:?}", n),
        // 不符合上面的范围。返回结果。
        n             => println!("I'm an old person of age {:?}", n),
    }
}
```
也可以在解构的时候使用 @ 绑定

```rust 
fn some_number() -> Option<u32> {
    Some(42)
}

fn main() {
    match some_number() {
        // 得到 `Some` 可变类型，如果它的值（绑定到 `n` 上）等于 42，则匹配。
        Some(n @ 42) => println!("The Answer: {}!", n),
        // 匹配任意其他数字。
        Some(n)      => println!("Not interesting... {}", n),
        // 匹配任意其他值（`None` 可变类型）。
        _            => (),
    }
}

```
## 8.6 if let 

有的时候使用 match 并不优雅，因为 match 要求必须枚举所有情况

```rust

#![allow(unused)]
fn main() {
// 将 `optional` 定为 `Option<i32>` 类型
let optional = Some(7);

match optional {
    Some(i) => {
        println!("This is a really long string and `{:?}`", i);
        // ^ 行首需要 2 层缩进。这里从 optional 中解构出 `i`。
        // 译注：正确的缩进是好的，但并不是 “不缩进就不能运行” 这个意思。
    },
    _ => {},
    // ^ 必须有，因为 `match` 需要覆盖全部情况。不觉得这行很多余吗？
};

}

```

所以就有了 if let 
- if let 解构
- 可以匹配到 解构失败的情况
- 除了正常的解构，还可以补充添加一些条件语句


```rust
fn main() {
    // 全部都是 `Option<i32>` 类型
    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    // `if let` 结构读作：若 `let` 将 `number` 解构成 `Some(i)`，则执行
    // 语句块（`{}`）
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // 如果要指明失败情形，就使用 else：
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // 解构失败。切换到失败情形。
        println!("Didn't match a number. Let's go with a letter!");
    };

    // 提供另一种失败情况下的条件。
    let i_like_letters = false;

    if let Some(i) = emoticon {
        println!("Matched {:?}!", i);
    // 解构失败。使用 `else if` 来判断是否满足上面提供的条件。
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        // 条件的值为 false。于是以下是默认的分支：
        println!("I don't like letters. Let's go with an emoticon :)!");
    };
}

```

### 8.6 while let

while let 也是为了将 match 改写得更好看一些
不过，if let 有可选的 else/else if 分支，while 没有

```rust
fn main() {
    // 将 `optional` 设为 `Option<i32>` 类型
    let mut optional = Some(0);

    // 这读作：当 `let` 将 `optional` 解构成 `Some(i)` 时，就
    // 执行语句块（`{}`）。否则就 `break`。
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ 使用的缩进更少，并且不用显式地处理失败情况。
    }
    // ^ `if let` 有可选的 `else`/`else if` 分句，
    // 而 `while let` 没有。
}
```


