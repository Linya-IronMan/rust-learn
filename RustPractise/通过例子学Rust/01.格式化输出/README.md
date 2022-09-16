
https://rustwiki.org/zh-CN/rust-by-example/hello/print/print_display.html

# print
```rust

fn main() {
    // 通常情况下，`{}` 会被任意变量内容所替换。
    // 变量内容会转化成字符串。
    println!("{} days", 31);

    // 不加后缀的话，31 就自动成为 i32 类型。
    // 你可以添加后缀来改变 31 的类型（例如使用 31i64 声明 31 为 i64 类型）。

    // 用变量替换字符串有多种写法。
    // 比如可以使用位置参数。
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用命名参数。
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // 可以在 `:` 后面指定特殊的格式。
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 你可以按指定宽度来右对齐文本。
    // 下面语句输出 "     1"，5 个空格后面连着 1。
    println!("{number:>width$}", number=1, width=6);

    // 你可以在数字左边补 0。下面语句输出 "000001"。
    println!("{number:>0width$}", number=1, width=6);

    // println! 会检查使用到的参数数量是否正确。
    // println!("My name is {0}, {1} {0}", "Bond");
    // 改正 ^ 补上漏掉的参数："James"

    // 创建一个包含单个 `i32` 的结构体（structure）。命名为 `Structure`。
    // #[allow(dead_code)]
    #[derive(Debug)]
    struct Structure(i32);

    // 但是像结构体这样的自定义类型需要更复杂的方式来处理。
    // 下面语句无法运行。
    println!("This struct `{:#?}` won't print...", Structure(3));
    // 改正 ^ 注释掉此行。
}

```

打印操作由 std::fmt 里面所定义的一系列宏来处理，包括：

- format!：将格式化文本写到字符串。
- print!：与 format! 类似，但将文本输出到控制台（io::stdout）。
- println!: 与 print! 类似，但输出结果追加一个换行符。
- eprint!：与 print! 类似，但将文本输出到标准错误（io::stderr）。
- eprintln!：与 eprint! 类似，但输出结果追加一个换行符。
- 这些宏都以相同的做法解析文本。有个额外优点是格式化的正确性会在编译时检查。

std::fmt 包含多种 trait（特质）来控制文字显示，其中重要的两种 trait 的基本形式如下：

fmt::Debug：使用 {:?} 标记。格式化文本以供调试使用。
fmt::Display：使用 {} 标记。以更优雅和友好的风格来格式化文本。

**默认应该都是使用的 fmt::Display trait，想要使用 fmt::Debug 需要通过 derive 将对应的 trait 挂载**


# 结构体的打印

```rust
#[derive(Debug)]
struct Structure(i32);

println!("{:#?}", Structure(3));
```

需要注意的是，结构体的打印是需要添加`#[derive(Debug)]`进行配合的，并且这种标记语法需要加在 struct 定义的位置，而不是println打印的位置。


# 调试（Debug）


所有类型都能 derive `fmt::Debug` 这个 trait。但是 `fmt::Display` 就需要手动实现

```rust

#![allow(unused)]
fn main() {
// 这个结构体不能使用 `fmt::Display` 或 `fmt::Debug` 来进行打印。
struct UnPrintable(i32);

// `derive` 属性会自动创建所需的实现，使这个 `struct` 能使用 `fmt::Debug` 打印。
#[derive(Debug)]
struct DebugPrintable(i32);
}
```
所有 std 库类型都天生可以使用 {:?} 来打印：
需要注意的是 Debug trait 时，如何使用打印序号
```rust

// 推导 `Structure` 的 `fmt::Debug` 实现。
// `Structure` 是一个包含单个 `i32` 的结构体。
#[derive(Debug)]
struct Structure(i32);

// 将 `Structure` 放到结构体 `Deep` 中。然后使 `Deep` 也能够打印。
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // 使用 `{:?}` 打印和使用 `{}` 类似。
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `Structure` 也可以打印！
    println!("Now {:?} will print!", Structure(3));
    
    // 使用 `derive` 的一个问题是不能控制输出的形式。
    // 假如我只想展示一个 `7` 怎么办？
    println!("Now {:?} will print!", Deep(Structure(7)));
}
```
Debug 确实可以打印内容，但是牺牲了一些美感。Rust 通过 `{:#?}` 提供了美化打印的功能

# Display 

Display 一半用于自定义的输出外观。这需要手动实现 `fmt::Display` 来做到

```rust

use std::fmt;
struct Structure(i32);
impl fmt::Display for Structure {
    // 为什么 f 参数的类型是 fmt::Formatter 的引用？
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HEllo world: {}", self.0)
    }
}
fn main() {
    println!("{}", Structure(3));
}

```

问题：
- self.0 是什么写法
- 

# List

- write!()? 在后面加上一个问号可以用于处理错误。如果有错误，就将错误输出，如果没有错误，就继续执行后面的代码。
- Vec<i32> 用于设置 Vector 类型
- vec.iter().enumerate() 用于遍历 Vector
- for (count, value) 遍历的第一个参数可以认为是迭代次数


```rust
use std::fmt;
struct Structure(Vec<i32>);
impl fmt::Display for Structure {
    // 为什么 f 参数的类型是 fmt::Formatter 的引用？
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (i, v) in vec.iter().enumerate() {
            if i != 0 {write!(f, ", ")?;}
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}
fn main() {
    println!("{}", Structure(vec![1, 2, 3, 4]));
}

```

# 格式化 Format


