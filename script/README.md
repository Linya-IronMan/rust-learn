[toc]

# print

https://rustwiki.org/zh-CN/rust-by-example/hello/print/print_display.html

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

- 什么叫做将文本输出到标准错误?


# 结构体的打印

```rust
#[derive(Debug)]
struct Structure(i32);

println!("{:#?}", Structure(3));
```

- 需要注意的是，结构体的打印是需要添加`#[derive(Debug)]`进行配合的，并且这种标记语法需要加在 struct 定义的位置，而不是println打印的位置。


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





要运行测试，首先将代码构建为库，然后告诉 rustdoc 在哪里找到库，这样它就可以 使每个文档中的程序链接到库：

```rust
$ rustc doc.rs --crate-type lib
$ rustdoc --test --extern doc="libdoc.rlib" doc.rs
```

文档注释的效果应当是在引用lib的时候才有效，暂时就不去做测试了。留待以后进行。


# 原生类型

- 标量类型
- 复合类型

## 标量类型

- 有符号整数（signed integers）：i8、i16、i32、i64、i128 和 isize（指针宽度）
- 无符号整数（unsigned integers）： u8、u16、u32、u64、u128 和 usize（指针宽度）
- 浮点数（floating point）： f32、f64
- char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
- bool（布尔型）：只能是 true 或 false
- 单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组

尽管单元类型的值是个元组，它却并不被认为是复合类型，因为并不包含多个值。

## 复合类型

- 数组（array）：如 [1, 2, 3]
- 元组（tuple）：如 (1, true)

变量都能够显式地给出类型说明（type annotation）。数字还可以通过后缀（suffix）或默认方式来声明类型。整型默认为 i32 类型，浮点型默认为 f64类型。

Rust 还可以根据上下文来推断（infer）类型（译注：比如一个未声明类型整数和 i64 的整数相加，则该整数会自动推断为 i64 类型。仅当根据环境无法推断时，才按默认方式取整型数值为 i32，浮点数值为 f64）

```rust
fn main() {
    // 变量可以给出类型说明。
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 常规说明
    let an_integer   = 5i32; // 后缀说明

    // 否则会按默认方式决定类型。
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    inferred_type = 4294967296i64;

    // 可变的（mutable）变量，其值可以改变。
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // 报错！变量的类型并不能改变。
    mutable = true;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;
}
```

通过加前缀 0x、0o、0b，数字可以用十六进制、八进制或二进制记法表示。

为了改善可读性，可以在数值字面量中插入下划线，比如：1_000 等同于 1000，0.000_001 等同于 0.000001。

```rust 
 // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
    // 试一试 ^ 尝试将 `1i32` 改为 `1u32`，体会为什么类型声明这么重要
```

# 元组 

元组是一个可以包含各种类型值的组合。元组使用括号 () 来构造（construct），而每个元组自身又是一个类型标记为 (T1, T2, ...) 的值，其中 T1、T2 是每个元素的类型。函数可以使用元组来返回多个值，因为元组可以拥有任意多个值。


- 元组中各项的类型不一定一样
- 访问元组中的某一项使用 .  语法 
- 元组可以在 Debug 中打印
- 过长的元组无法打印 —— 为什么
- 创建单元素元组的时候需要加上一个逗号 let tumple = (1,)  let i = (1) 这样创建的是一个整数类型

```rust 
// 元组可以充当函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用 `let` 把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;

    (boolean, integer)
}

// 在 “动手试一试” 的练习中要用到下面这个结构体。
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 通过元组的下标来访问具体的值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组无法打印
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)

}

```

## 数组和切片

**数组**
- 数组中存储的类型是相同的
- 数组在内存中是连续存储的
- 数组使用中括号创建 `[]` 在创建后大小是固定的
- 数组是在栈中进行分配的
- 数组通过 [] 访问越界会引发 panic 致命错误


**切片**
- 与数组类似，但是大小在创建的时候并不确定，创建之后切片是可变的
- 切片是一个双字对象。第一个字是指向数据的指针，第二个字是切片的长度。
- 这里字的宽度和 usize 大小相同，由处理器架构决定
- slice 可以借用来数组的一部分，slice 的标记类型是 &[T]
- 切片和数组一样是通过中括号进行访问

**切片的获取**
- ..y 等价于 0..y
- x.. 等价于位置 x 到数据结束
- .. 等价于位置 0 到结束
- 切片获取只有两个 `.`  

切片不止可以从数组中获取，也可以从字符串获取

```rust
let str = String::from("Hello world");
let slice = &str[..]
```

**问题**
- 如何通过trait动态获取数组切片
- 数组的更安全的访问方法, get

```rust 
use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 定长数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 下标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 越界的下标会引发致命错误（panic）
    println!("{}", xs[5]);
}

```

- std::mem::size_of_val(&slice) 可以用来获取数据占用内存大小 byte



# 自定义类型

- struct: 定义一个结构体
- enum：定义一个枚举类型

常量可以通过 const 和 static 关键字创建

## 结构体

**结构体的类型**

- 元组结构体
- C 语言风格结构体
- 单元结构体，不带字段，在范型中很有用

---

**结构体操作**

- 更新结构体（做部分更新）`let bottom_right = Point { x: 5.2, ..point };`
- 元组风格的结构体访问内容
  
```rust
// 元组风格结构体
struct Pair(i32, f32)
 // 实例化一个元组结构体
let pair = Pair(1, 0.1);
// 访问元组结构体的字段
println!("pair contains {:?} and {:?}", pair.0, pair.1);
// 解构一个元组结构体
let Pair(integer, decimal) = pair;

// C 语言风格
struct Point {
  x: f32,
  y: f32,
}

// 单元结构体
struct Unit;

// 使用结构体更新语法创建新的 point，
// 这样可以用到之前的 point 的字段
let bottom_right = Point { x: 5.2, ..point };

// 实例化一个单元结构体
let _unit = Unit;

```

## Enum 枚举

- 隐藏对未使用代码的警告 `#[allow(dead_code)]`
- match 中对 enum 的使用是在执行析构操作
- 枚举是通过关键字以及类型来确定一个"类型"

enum 关键字允许创建一个从数个不同取值中选其一的枚举类型（enumeration）。任何一个在 struct 中合法的取值在 enum 中也合法。

```rust
// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]

// 创建一个 `enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了 `enum`
// 取值的种类：`PageLoad` 不等于 `PageUnload`，`KeyPress(char)` 不等于
// `Paste(String)`。各个取值不同，互相独立。
enum WebEvent {
    // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
    PageLoad,
    PageUnload,
    // 或者一个元组结构体，
    KeyPress(char),
    Paste(String),
    // 或者一个普通的结构体。
    Click { x: i64, y: i64 }
}

// 此函数将一个 `WebEvent` enum 作为参数，无返回值。
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 从 `enum` 里解构出 `c`。
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把 `Click` 解构给 `x` and `y`。
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

```

## 类型别名

若使用类型别名，则可以通过其别名引用每个枚举变量。当枚举的名称太长或者太一般化，且你想要对其重命名，那么这对你会有所帮助。

- impl 中的类型别名 self
- use 语法与类型别名
  - use Status::{Poor, Rich}
  - use Work::*;


```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}

```

- 最常见的别名实际上就是 impl 块中使用的 Self 别名

```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
```

使用 use 声明，可以不用写出完整的路径

```rust
// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 显式地 `use` 各个名称使他们直接可用，而不需要指定它们来自 `Status`。
    use Status::{Poor, Rich};
    // 自动地 `use` `Work` 内部的各个名称。
    use Work::*;

    // `Poor` 等价于 `Status::Poor`。
    let status = Poor;
    // `Civilian` 等价于 `Work::Civilian`。
    let work = Civilian;

    match status {
        // 注意这里没有用完整路径，因为上面显式地使用了 `use`。
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 再次注意到没有用完整路径。
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}

```

## C 风格用法

实际上和 TypeScript 的使用很像。

```rust
// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]

// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enum` 可以转成整型。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

```

## Linked List 链表

- enum 和 struct 一样都可以通过 impl 挂载trait
- enum 的链表实现
- ref: 将会使用借用而不是移动
- match 匹配的时候，最好使用具体的类型T，而不是引用类型 &T
- match 使用的时候可能会发生所有权的转移，如果不想转移所有权，可以使用 ref 关键字标记
- 当self用作函数的第一个参数时，它等价于self: Self。&self参数等价于self: &Self。&mut self参数等价于self: &mut Self。
- 一般写 trait 的时候，self 建议写成 &self, 这样不会发生move
- impl List<i32> {} 这个i32，并不是每个trait都要写

** 问题 **
- impl List<i32> {} 类型在什么时候需要写？
- format! 这个宏是用来干什么的


```rust

use List::*;

enum List {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil：末结点，表明链表结束
    Nil,
}

// 可以为 enum 定义方法
impl List {
    // 创建一个空的 List 实例
    fn new() -> List {
        // `Nil` 为 `List` 类型（译注：因 `Nil` 的完整名称是 `List::Nil`）
        Nil
    }

    // 处理一个 List，在其头部插入新元素，并返回该 List
    fn prepend(self, elem: u32) -> List {
        // `Cons` 同样为 List 类型
        Cons(elem, Box::new(self))
    }

    // 返回 List 的长度
    fn len(&self) -> u32 {
        // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的
        // 取值种类。
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        // 类型要好过匹配引用 `&T`。
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 因此使用一个对 tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // （递归的）基准情形（base case）：一个长度为 0 的空列表
            Nil => 0
        }
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                // 而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 创建一个空链表
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
```


## 常量

- const : 不可改变的值（通常使用这一种）
- static：具有 `'static` 生命周期的，可以是可变的变量 (`static mut ` 关键字声明)
- static mut LANG: &'static str = "JavaScript"
- static 被认为是线程不安全的，所以在使用的时候需要使用 unsafe 字段

```rust

// 全局变量是在所有其他作用域之外声明的。
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}
```



```rust
static mut LANGUAGE: &'static str = "Rust";
#[allow(dead_code)]
const NUMBER: i32 = 10;

fn main() {
    unsafe {
        LANGUAGE = "JavaScript";
        println!("{}", LANGUAGE);
    }
}
```


# 变量绑定

https://rustwiki.org/zh-CN/rust-by-example/variable_bindings.html

- 变量名如果没有被使用，并且是故意如此。可以在变量名前加上 `_` 消除警告
- println 中 {:?} 并不是必须在 Debug 下使用

```rust
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // 改正 ^ 在变量名前加上下划线以消除警告
}

```

## 变量冻结

- 一个可变的变量，可以通过 冻结(freeze) 操作来取消变量的 "可修改性"
- 冻结操作也是有作用范围的

```rust
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // 被不可变的 `_mutable_integer` 遮蔽
        let _mutable_integer = _mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        _mutable_integer = 50;
        // 改正 ^ 注释掉上面这行

        // `_mutable_integer` 离开作用域
    }

    // 正常运行！ `_mutable_integer` 在这个作用域没有冻结
    _mutable_integer = 3;
}


```


# 5.0 类型系统

- 原生类型的类型转换（cast）。
- 指定字面量的类型。
- 使用类型推断（type inference）。
- 给类型取别名（alias）。


## 5.1 类型转换 

Rust 不提供原生类型之间的隐式类型转换（coercion），但可以使用 as 关键字进行显 式类型转换（casting）。

- #![allow(overflowing_literals)]: 不显示类型转换产生的溢出警告
- rust 没有任何隐式的类型转换
- 使用 as 可以进行显式的类型转换

```rust
// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // 错误！不提供隐式转换
    let integer: u8 = decimal;
    // 改正 ^ 注释掉这一行

    // 可以显式转换
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当把任何类型转换为无符号类型 T 时，会不断加上或减去 (std::T::MAX + 1)
    // 直到值位于新类型 T 的范围内。

    // 1000 已经在 u16 的范围内
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 事实上的处理方式是：从最低有效位（LSB，least significant bits）开始保留
    // 8 位，然后剩余位置，直到最高有效位（MSB，most significant bit）都被抛弃。
    // 译注：MSB 就是二进制的最高位，LSB 就是二进制的最低位，按日常书写习惯就是
    // 最左边一位和最右边一位。
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 对正数，这就和取模一样。
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换到有符号类型时，（位操作的）结果就和 “先转换到对应的无符号类型，
    // 如果 MSB 是 1，则该值为负” 是一样的。

    // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 重复之前的例子
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 的二进制补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}
```

## 5.2 字面量

- 数字可以通过后缀的方式标记类型
- 无后缀的数字字面量，其类型如果没有限制，编译器会对整数使用`i32`, 对浮点数使用 `f64`
- std::mem::size_of_val(&x); 返回一个变量所占字节数

```rust
fn main() {
    // 带后缀的字面量，其类型在初始化时已经知道了。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，其类型取决于如何使用它们。
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回一个变量所占的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

```


## 5.4 别名

- 使用`type`语句给已有的类型取一个新的名字
- 类型的名字必须遵循驼峰命名法(CamelCase)，否则编译器会给出警告






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



# 7 表达式

- Rust 中代码块也可以是一个表达式
- 如果代码块中没有返回值，那么会自动返回一个 ()

代码块中 z 就是一个 （）

```rust 
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将此表达式赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给 `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
```


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


# 9 函数

## 9.1 方法

- 方法如果不对实例有所依赖或者干扰，一般叫做静态方法
- 函数内部通过 self.name 的方式访问绑定目标的结构体实例属性
- &self 是 self: &self 的语法糖


```rust
struct Point {
    x: f64,
    y: f64,
}

// 实现的代码块，`Point` 的所有方法都在这里给出
impl Point {
    // 这是一个静态方法（static method）
    // 静态方法不需要被实例调用
    // 这类方法一般用作构造器（constructor）
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 另外一个静态方法，需要两个参数：
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 这是一个实例方法（instance method）
    // `&self` 是 `self: &Self` 的语法糖（sugar），其中 `Self` 是方法调用者的
    // 类型。在这个例子中 `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` 通过点运算符来访问结构体字段
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` 是一个 `f64` 类型的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 这个方法要求调用者是可变的
    // `&mut self` 为 `self: &mut Self` 的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` 拥有资源：两个堆分配的整型
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 这个方法会 “消耗” 调用者的资源
    // `self` 为 `self: Self` 的语法糖
    fn destroy(self) {
        // 解构 `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` 和 `second` 离开作用域后释放
    }
}

fn main() {
    let rectangle = Rectangle {
        // 静态方法使用双冒号调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 实例方法通过点运算符来调用
    // 注意第一个参数 `&self` 是隐式传递的，亦即：
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 报错！ `rectangle` 是不可变的，但这方法需要一个可变对象
    //rectangle.translate(1.0, 0.0);
    // 试一试 ^ 去掉此行的注释

    // 正常运行！可变对象可以调用可变方法
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // 报错！前面的 `destroy` 调用 “消耗了” `pair`
    //pair.destroy();
    // 试一试 ^ 将此行注释去掉
}
```

## 9.2 闭包

Rust 中的闭包也叫做 lambda表达式，能够捕获周围作用域中变量的函数。

- 声明时使用 `||` 替代 `()` 包裹输入参数
- 函数定界符（{}）在单个表达式的时候可选，其他情况必须加上
- 有能力捕获外部环境的变量
- 闭包是一种匿名函数
- 普通函数中无法访问外界的环境变量
- 闭包的语法可能会非常简洁：`let a = || 3;`
- 闭包不必书写返回的类型以及参数类型，它会自动推导


### 9.2.1 捕获










# print

https://rustwiki.org/zh-CN/rust-by-example/hello/print/print_display.html

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

- 什么叫做将文本输出到标准错误?


# 结构体的打印

```rust
#[derive(Debug)]
struct Structure(i32);

println!("{:#?}", Structure(3));
```

- 需要注意的是，结构体的打印是需要添加`#[derive(Debug)]`进行配合的，并且这种标记语法需要加在 struct 定义的位置，而不是println打印的位置。


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





要运行测试，首先将代码构建为库，然后告诉 rustdoc 在哪里找到库，这样它就可以 使每个文档中的程序链接到库：

```rust
$ rustc doc.rs --crate-type lib
$ rustdoc --test --extern doc="libdoc.rlib" doc.rs
```

文档注释的效果应当是在引用lib的时候才有效，暂时就不去做测试了。留待以后进行。


# 原生类型

- 标量类型
- 复合类型

## 标量类型

- 有符号整数（signed integers）：i8、i16、i32、i64、i128 和 isize（指针宽度）
- 无符号整数（unsigned integers）： u8、u16、u32、u64、u128 和 usize（指针宽度）
- 浮点数（floating point）： f32、f64
- char（字符）：单个 Unicode 字符，如 'a'，'α' 和 '∞'（每个都是 4 字节）
- bool（布尔型）：只能是 true 或 false
- 单元类型（unit type）：()。其唯一可能的值就是 () 这个空元组

尽管单元类型的值是个元组，它却并不被认为是复合类型，因为并不包含多个值。

## 复合类型

- 数组（array）：如 [1, 2, 3]
- 元组（tuple）：如 (1, true)

变量都能够显式地给出类型说明（type annotation）。数字还可以通过后缀（suffix）或默认方式来声明类型。整型默认为 i32 类型，浮点型默认为 f64类型。

Rust 还可以根据上下文来推断（infer）类型（译注：比如一个未声明类型整数和 i64 的整数相加，则该整数会自动推断为 i64 类型。仅当根据环境无法推断时，才按默认方式取整型数值为 i32，浮点数值为 f64）

```rust
fn main() {
    // 变量可以给出类型说明。
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 常规说明
    let an_integer   = 5i32; // 后缀说明

    // 否则会按默认方式决定类型。
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // 类型也可根据上下文自动推断。
    let mut inferred_type = 12; // 根据下一行的赋值推断为 i64 类型
    inferred_type = 4294967296i64;

    // 可变的（mutable）变量，其值可以改变。
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // 报错！变量的类型并不能改变。
    mutable = true;

    // 但可以用遮蔽（shadow）来覆盖前面的变量。
    let mutable = true;
}
```

通过加前缀 0x、0o、0b，数字可以用十六进制、八进制或二进制记法表示。

为了改善可读性，可以在数值字面量中插入下划线，比如：1_000 等同于 1000，0.000_001 等同于 0.000001。

```rust 
 // 整数相加
    println!("1 + 2 = {}", 1u32 + 2);

    // 整数相减
    println!("1 - 2 = {}", 1i32 - 2);
    // 试一试 ^ 尝试将 `1i32` 改为 `1u32`，体会为什么类型声明这么重要
```

# 元组 

元组是一个可以包含各种类型值的组合。元组使用括号 () 来构造（construct），而每个元组自身又是一个类型标记为 (T1, T2, ...) 的值，其中 T1、T2 是每个元素的类型。函数可以使用元组来返回多个值，因为元组可以拥有任意多个值。


- 元组中各项的类型不一定一样
- 访问元组中的某一项使用 .  语法 
- 元组可以在 Debug 中打印
- 过长的元组无法打印 —— 为什么
- 创建单元素元组的时候需要加上一个逗号 let tumple = (1,)  let i = (1) 这样创建的是一个整数类型

```rust 
// 元组可以充当函数的参数和返回值
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // 可以使用 `let` 把一个元组的成员绑定到一些变量
    let (integer, boolean) = pair;

    (boolean, integer)
}

// 在 “动手试一试” 的练习中要用到下面这个结构体。
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 包含各种不同类型的元组
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 通过元组的下标来访问具体的值
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple second value: {}", long_tuple.1);

    // 元组也可以充当元组的元素
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 元组可以打印
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 但很长的元组无法打印
    // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    // println!("too long tuple: {:?}", too_long_tuple);
    // 试一试 ^ 取消上面两行的注释，阅读编译器给出的错误信息。

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", reverse(pair));

    // 创建单元素元组需要一个额外的逗号，这是为了和被括号包含的字面量作区分。
    println!("one element tuple: {:?}", (5u32,));
    println!("just an integer: {:?}", (5u32));

    // 元组可以被解构（deconstruct），从而将值绑定给变量
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix)

}

```

## 数组和切片

**数组**
- 数组中存储的类型是相同的
- 数组在内存中是连续存储的
- 数组使用中括号创建 `[]` 在创建后大小是固定的
- 数组是在栈中进行分配的
- 数组通过 [] 访问越界会引发 panic 致命错误


**切片**
- 与数组类似，但是大小在创建的时候并不确定，创建之后切片是可变的
- 切片是一个双字对象。第一个字是指向数据的指针，第二个字是切片的长度。
- 这里字的宽度和 usize 大小相同，由处理器架构决定
- slice 可以借用来数组的一部分，slice 的标记类型是 &[T]
- 切片和数组一样是通过中括号进行访问

**切片的获取**
- ..y 等价于 0..y
- x.. 等价于位置 x 到数据结束
- .. 等价于位置 0 到结束
- 切片获取只有两个 `.`  

切片不止可以从数组中获取，也可以从字符串获取

```rust
let str = String::from("Hello world");
let slice = &str[..]
```

**问题**
- 如何通过trait动态获取数组切片
- 数组的更安全的访问方法, get

```rust 
use std::mem;

// 此函数借用一个 slice
fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    // 定长数组（类型标记是多余的）
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [0; 500];

    // 下标从 0 开始
    println!("first element of the array: {}", xs[0]);
    println!("second element of the array: {}", xs[1]);

    // `len` 返回数组的大小
    println!("array size: {}", xs.len());

    // 数组是在栈中分配的
    println!("array occupies {} bytes", mem::size_of_val(&xs));

    // 数组可以自动被借用成为 slice
    println!("borrow the whole array as a slice");
    analyze_slice(&xs);

    // slice 可以指向数组的一部分
    println!("borrow a section of the array as a slice");
    analyze_slice(&ys[1 .. 4]);

    // 越界的下标会引发致命错误（panic）
    println!("{}", xs[5]);
}

```

- std::mem::size_of_val(&slice) 可以用来获取数据占用内存大小 byte



# 自定义类型

- struct: 定义一个结构体
- enum：定义一个枚举类型

常量可以通过 const 和 static 关键字创建

## 结构体

**结构体的类型**

- 元组结构体
- C 语言风格结构体
- 单元结构体，不带字段，在范型中很有用

---

**结构体操作**

- 更新结构体（做部分更新）`let bottom_right = Point { x: 5.2, ..point };`
- 元组风格的结构体访问内容
  
```rust
// 元组风格结构体
struct Pair(i32, f32)
 // 实例化一个元组结构体
let pair = Pair(1, 0.1);
// 访问元组结构体的字段
println!("pair contains {:?} and {:?}", pair.0, pair.1);
// 解构一个元组结构体
let Pair(integer, decimal) = pair;

// C 语言风格
struct Point {
  x: f32,
  y: f32,
}

// 单元结构体
struct Unit;

// 使用结构体更新语法创建新的 point，
// 这样可以用到之前的 point 的字段
let bottom_right = Point { x: 5.2, ..point };

// 实例化一个单元结构体
let _unit = Unit;

```

## Enum 枚举

- 隐藏对未使用代码的警告 `#[allow(dead_code)]`
- match 中对 enum 的使用是在执行析构操作
- 枚举是通过关键字以及类型来确定一个"类型"

enum 关键字允许创建一个从数个不同取值中选其一的枚举类型（enumeration）。任何一个在 struct 中合法的取值在 enum 中也合法。

```rust
// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]

// 创建一个 `enum`（枚举）来对 web 事件分类。注意变量名和类型共同指定了 `enum`
// 取值的种类：`PageLoad` 不等于 `PageUnload`，`KeyPress(char)` 不等于
// `Paste(String)`。各个取值不同，互相独立。
enum WebEvent {
    // 一个 `enum` 可以是单元结构体（称为 `unit-like` 或 `unit`），
    PageLoad,
    PageUnload,
    // 或者一个元组结构体，
    KeyPress(char),
    Paste(String),
    // 或者一个普通的结构体。
    Click { x: i64, y: i64 }
}

// 此函数将一个 `WebEvent` enum 作为参数，无返回值。
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // 从 `enum` 里解构出 `c`。
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把 `Click` 解构给 `x` and `y`。
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从一个字符串切片中创建一个具有所有权的 `String`。
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

```

## 类型别名

若使用类型别名，则可以通过其别名引用每个枚举变量。当枚举的名称太长或者太一般化，且你想要对其重命名，那么这对你会有所帮助。

- impl 中的类型别名 self
- use 语法与类型别名
  - use Status::{Poor, Rich}
  - use Work::*;


```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

// Creates a type alias
type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
    // We can refer to each variant via its alias, not its long and inconvenient
    // name.
    let x = Operations::Add;
}

```

- 最常见的别名实际上就是 impl 块中使用的 Self 别名

```rust
enum VeryVerboseEnumOfThingsToDoWithNumbers {
    Add,
    Subtract,
}

impl VeryVerboseEnumOfThingsToDoWithNumbers {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}
```

使用 use 声明，可以不用写出完整的路径

```rust
// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // 显式地 `use` 各个名称使他们直接可用，而不需要指定它们来自 `Status`。
    use Status::{Poor, Rich};
    // 自动地 `use` `Work` 内部的各个名称。
    use Work::*;

    // `Poor` 等价于 `Status::Poor`。
    let status = Poor;
    // `Civilian` 等价于 `Work::Civilian`。
    let work = Civilian;

    match status {
        // 注意这里没有用完整路径，因为上面显式地使用了 `use`。
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // 再次注意到没有用完整路径。
        Civilian => println!("Civilians work!"),
        Soldier  => println!("Soldiers fight!"),
    }
}

```

## C 风格用法

实际上和 TypeScript 的使用很像。

```rust
// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]

// 拥有隐式辨别值（implicit discriminator，从 0 开始）的 enum
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    // `enum` 可以转成整型。
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}

```

## Linked List 链表

- enum 和 struct 一样都可以通过 impl 挂载trait
- enum 的链表实现
- ref: 将会使用借用而不是移动
- match 匹配的时候，最好使用具体的类型T，而不是引用类型 &T
- match 使用的时候可能会发生所有权的转移，如果不想转移所有权，可以使用 ref 关键字标记
- 当self用作函数的第一个参数时，它等价于self: Self。&self参数等价于self: &Self。&mut self参数等价于self: &mut Self。
- 一般写 trait 的时候，self 建议写成 &self, 这样不会发生move
- impl List<i32> {} 这个i32，并不是每个trait都要写

** 问题 **
- impl List<i32> {} 类型在什么时候需要写？
- format! 这个宏是用来干什么的


```rust

use List::*;

enum List {
    // Cons：元组结构体，包含链表的一个元素和一个指向下一节点的指针
    Cons(u32, Box<List>),
    // Nil：末结点，表明链表结束
    Nil,
}

// 可以为 enum 定义方法
impl List {
    // 创建一个空的 List 实例
    fn new() -> List {
        // `Nil` 为 `List` 类型（译注：因 `Nil` 的完整名称是 `List::Nil`）
        Nil
    }

    // 处理一个 List，在其头部插入新元素，并返回该 List
    fn prepend(self, elem: u32) -> List {
        // `Cons` 同样为 List 类型
        Cons(elem, Box::new(self))
    }

    // 返回 List 的长度
    fn len(&self) -> u32 {
        // 必须对 `self` 进行匹配（match），因为这个方法的行为取决于 `self` 的
        // 取值种类。
        // `self` 为 `&List` 类型，`*self` 为 `List` 类型，匹配一个具体的 `T`
        // 类型要好过匹配引用 `&T`。
        match *self {
            // 不能得到 tail 的所有权，因为 `self` 是借用的；
            // 因此使用一个对 tail 的引用
            Cons(_, ref tail) => 1 + tail.len(),
            // （递归的）基准情形（base case）：一个长度为 0 的空列表
            Nil => 0
        }
    }

    // 返回列表的字符串表示（该字符串是堆分配的）
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，
                // 而不是打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    // 创建一个空链表
    let mut list = List::new();

    // 追加一些元素
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // 显示链表的最后状态
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
```


## 常量

- const : 不可改变的值（通常使用这一种）
- static：具有 `'static` 生命周期的，可以是可变的变量 (`static mut ` 关键字声明)
- static mut LANG: &'static str = "JavaScript"
- static 被认为是线程不安全的，所以在使用的时候需要使用 unsafe 字段

```rust

// 全局变量是在所有其他作用域之外声明的。
static LANGUAGE: &'static str = "Rust";
const  THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // 在一般函数中访问常量
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // 在 main 函数（主函数）中访问常量
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // 报错！不能修改一个 `const` 常量。
    THRESHOLD = 5;
    // 改正 ^ 注释掉此行
}
```



```rust
static mut LANGUAGE: &'static str = "Rust";
#[allow(dead_code)]
const NUMBER: i32 = 10;

fn main() {
    unsafe {
        LANGUAGE = "JavaScript";
        println!("{}", LANGUAGE);
    }
}
```


# 变量绑定

https://rustwiki.org/zh-CN/rust-by-example/variable_bindings.html

- 变量名如果没有被使用，并且是故意如此。可以在变量名前加上 `_` 消除警告
- println 中 {:?} 并不是必须在 Debug 下使用

```rust
fn main() {
    let an_integer = 1u32;
    let a_boolean = true;
    let unit = ();

    // 将 `an_integer` 复制到 `copied_integer`
    let copied_integer = an_integer;

    println!("An integer: {:?}", copied_integer);
    println!("A boolean: {:?}", a_boolean);
    println!("Meet the unit value: {:?}", unit);

    // 编译器会对未使用的变量绑定产生警告；可以给变量名加上下划线前缀来消除警告。
    let _unused_variable = 3u32;

    let noisy_unused_variable = 2u32;
    // 改正 ^ 在变量名前加上下划线以消除警告
}

```

## 变量冻结

- 一个可变的变量，可以通过 冻结(freeze) 操作来取消变量的 "可修改性"
- 冻结操作也是有作用范围的

```rust
fn main() {
    let mut _mutable_integer = 7i32;

    {
        // 被不可变的 `_mutable_integer` 遮蔽
        let _mutable_integer = _mutable_integer;

        // 报错！`_mutable_integer` 在本作用域被冻结
        _mutable_integer = 50;
        // 改正 ^ 注释掉上面这行

        // `_mutable_integer` 离开作用域
    }

    // 正常运行！ `_mutable_integer` 在这个作用域没有冻结
    _mutable_integer = 3;
}


```


# 5.0 类型系统

- 原生类型的类型转换（cast）。
- 指定字面量的类型。
- 使用类型推断（type inference）。
- 给类型取别名（alias）。


## 5.1 类型转换 

Rust 不提供原生类型之间的隐式类型转换（coercion），但可以使用 as 关键字进行显 式类型转换（casting）。

- #![allow(overflowing_literals)]: 不显示类型转换产生的溢出警告
- rust 没有任何隐式的类型转换
- 使用 as 可以进行显式的类型转换

```rust
// 不显示类型转换产生的溢出警告。
#![allow(overflowing_literals)]

fn main() {
    let decimal = 65.4321_f32;

    // 错误！不提供隐式转换
    let integer: u8 = decimal;
    // 改正 ^ 注释掉这一行

    // 可以显式转换
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // 当把任何类型转换为无符号类型 T 时，会不断加上或减去 (std::T::MAX + 1)
    // 直到值位于新类型 T 的范围内。

    // 1000 已经在 u16 的范围内
    println!("1000 as a u16 is: {}", 1000 as u16);

    // 1000 - 256 - 256 - 256 = 232
    // 事实上的处理方式是：从最低有效位（LSB，least significant bits）开始保留
    // 8 位，然后剩余位置，直到最高有效位（MSB，most significant bit）都被抛弃。
    // 译注：MSB 就是二进制的最高位，LSB 就是二进制的最低位，按日常书写习惯就是
    // 最左边一位和最右边一位。
    println!("1000 as a u8 is : {}", 1000 as u8);
    // -1 + 256 = 255
    println!("  -1 as a u8 is : {}", (-1i8) as u8);

    // 对正数，这就和取模一样。
    println!("1000 mod 256 is : {}", 1000 % 256);

    // 当转换到有符号类型时，（位操作的）结果就和 “先转换到对应的无符号类型，
    // 如果 MSB 是 1，则该值为负” 是一样的。

    // 当然如果数值已经在目标类型的范围内，就直接把它放进去。
    println!(" 128 as a i16 is: {}", 128 as i16);
    // 128 转成 u8 还是 128，但转到 i8 相当于给 128 取八位的二进制补码，其值是：
    println!(" 128 as a i8 is : {}", 128 as i8);

    // 重复之前的例子
    // 1000 as u8 -> 232
    println!("1000 as a u8 is : {}", 1000 as u8);
    // 232 的二进制补码是 -24
    println!(" 232 as a i8 is : {}", 232 as i8);
}
```

## 5.2 字面量

- 数字可以通过后缀的方式标记类型
- 无后缀的数字字面量，其类型如果没有限制，编译器会对整数使用`i32`, 对浮点数使用 `f64`
- std::mem::size_of_val(&x); 返回一个变量所占字节数

```rust
fn main() {
    // 带后缀的字面量，其类型在初始化时已经知道了。
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // 无后缀的字面量，其类型取决于如何使用它们。
    let i = 1;
    let f = 1.0;

    // `size_of_val` 返回一个变量所占的字节数
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));
}

```


## 5.4 别名

- 使用`type`语句给已有的类型取一个新的名字
- 类型的名字必须遵循驼峰命名法(CamelCase)，否则编译器会给出警告






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



# 7 表达式

- Rust 中代码块也可以是一个表达式
- 如果代码块中没有返回值，那么会自动返回一个 ()

代码块中 z 就是一个 （）

```rust 
fn main() {
    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // 将此表达式赋给 `y`
        x_cube + x_squared + x
    };

    let z = {
        // 分号结束了这个表达式，于是将 `()` 赋给 `z`
        2 * x;
    };

    println!("x is {:?}", x);
    println!("y is {:?}", y);
    println!("z is {:?}", z);
}
```


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


# 9. 函数

## 9.1 方法

*   方法如果不对实例有所依赖或者干扰，一般叫做静态方法
*   函数内部通过 self.name 的方式访问绑定目标的结构体实例属性
*   \&self 是 self: \&self 的语法糖

```rust
struct Point {
    x: f64,
    y: f64,
}

// 实现的代码块，`Point` 的所有方法都在这里给出
impl Point {
    // 这是一个静态方法（static method）
    // 静态方法不需要被实例调用
    // 这类方法一般用作构造器（constructor）
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // 另外一个静态方法，需要两个参数：
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // 这是一个实例方法（instance method）
    // `&self` 是 `self: &Self` 的语法糖（sugar），其中 `Self` 是方法调用者的
    // 类型。在这个例子中 `Self` = `Rectangle`
    fn area(&self) -> f64 {
        // `self` 通过点运算符来访问结构体字段
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` 是一个 `f64` 类型的方法，返回调用者的绝对值
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // 这个方法要求调用者是可变的
    // `&mut self` 为 `self: &mut Self` 的语法糖
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` 拥有资源：两个堆分配的整型
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // 这个方法会 “消耗” 调用者的资源
    // `self` 为 `self: Self` 的语法糖
    fn destroy(self) {
        // 解构 `self`
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` 和 `second` 离开作用域后释放
    }
}

fn main() {
    let rectangle = Rectangle {
        // 静态方法使用双冒号调用
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // 实例方法通过点运算符来调用
    // 注意第一个参数 `&self` 是隐式传递的，亦即：
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // 报错！ `rectangle` 是不可变的，但这方法需要一个可变对象
    //rectangle.translate(1.0, 0.0);
    // 试一试 ^ 去掉此行的注释

    // 正常运行！可变对象可以调用可变方法
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // 报错！前面的 `destroy` 调用 “消耗了” `pair`
    //pair.destroy();
    // 试一试 ^ 将此行注释去掉
}
```

## 9.2 闭包

Rust 中的闭包也叫做 lambda表达式，能够捕获周围作用域中变量的函数。

*   声明时使用 `||` 替代 `()` 包裹输入参数
*   函数定界符（{}）在单个表达式的时候可选，其他情况必须加上
*   有能力捕获外部环境的变量
*   闭包是一种匿名函数
*   普通函数中无法访问外界的环境变量
*   闭包的语法可能会非常简洁：`let a = || 3;`
*   闭包不必书写返回的类型以及参数类型，它会自动推导
*   闭包会自动推导参数以及返回值类型。但是如果对同一个闭包多次调用的时候传递的参数不同，就会报错。
*   每一个闭包都有自己独有的匿名类型。即使两个闭包有着相同的签名，它们的类型仍然可以被认为是不同的

### 9.2.1 捕获

*   闭包优先(默认)通过引用来捕获变量，并且仅在需要时使用其他方式。

<!---->

*   一般情况下，传入闭包中的变量，虽然写法好像是使用的变量的本体，但是实际上是变量的引用

```rust

    let color = String::from("green");

    // 这个闭包打印 `color`。它会立即借用（通过引用，`&`）`color` 并将该借用和
    // 闭包本身存储到 `print` 变量中。`color`  会一直保持被借用状态直到
    // `print` 离开作用域。
    //
    // `println!` 只需传引用就能使用，而这个闭包捕获的也是变量的引用，因此无需
    // 进一步处理就可以使用 `println!`。
    let print = || println!("`color`: {}", color);

    // 使用借用来调用闭包 `color`。
    print();

    // `color` 可再次被不可变借用，因为闭包只持有一个指向 `color` 的不可变引用。
    let _reborrow = &color;

    print();

    // 在最后使用 `print` 之后，移动或重新借用都是允许的。
    let _color_moved = color;
```

*   闭包中如果发生了数据的修改，那么闭包需要加上一个 mut

<!---->

*   这里需要闭包执行完最后一次调用之后，才能重新借用，否则会报错。

问题：如果是一个普通的函数，也会这样么？为什么会有这样的规则

```rust
    let mut count = 0;
    // 这个闭包使 `count` 值增加。要做到这点，它需要得到 `&mut count` 或者
    // `count` 本身，但 `&mut count` 的要求没那么严格，所以我们采取这种方式。
    // 该闭包立即借用 `count`。
    //
    // `inc` 前面需要加上 `mut`，因为闭包里存储着一个 `&mut` 变量。调用闭包时，
    // 该变量的变化就意味着闭包内部发生了变化。因此闭包需要是可变的。
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };

    // 使用可变借用调用闭包
    inc();

    // 因为之后调用闭包，所以仍然可变借用 `count`
    // 试图重新借用将导致错误
    // let _reborrow = &count;
    // ^ 试一试：将此行注释去掉。
    inc();

    // 闭包不再借用 `&mut count`，因此可以正确地重新借用
    let _count_reborrowed = &mut count;
```

```rust
fn main() {
    use std::mem;
    // 不可复制类型（non-copy type）。
    let movable = Box::new(3);

    // `mem::drop` 要求 `T` 类型本身，所以闭包将会捕获变量的值。这种情况下，
    // 可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动
    // （move）到闭包中，因而 `movable` 变量在这里立即移动到了闭包中。
    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    // `consume` 消耗了该变量，所以该闭包只能调用一次。
    consume();
    //consume();
    // ^ 试一试：将此行注释去掉。
}
```

***

#### 9.2.2.2 闭包的捕获原理

闭包可以通过三种方式捕获其环境，它们直接对应函数的三种获取参数的方式：获取所有权，可变借用和不可变借用。这三种捕获值的方式被编码为如下三个 `Fn` trait

*   `FnOnce` 消费从周围作用域捕获的变量，闭包周围的作用域被成为其环境。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包。其名称的 `Once` 部分代表了闭包不能多次获取相同变量的所有权的事实。所以，它只能被调用一次。
*   `FnMut` 获取可变的借用值，所以可以改变其环境
*   `Fn` 从其环境获取不可变的借用值

创建一个闭包时，Rust 根据其如何使用环境中的变量来推断我们希望如何引用环境。
由于所有闭包都可以被调用至少一次，所以所有闭包都实现了 `FnOnce` 。
那些并没有移动被捕获变量所有权到闭包内的闭包也实现了`FnMut`，而不需要对捕获的变量进行可变访问的闭包也实现了`Fn`。

如果希望强制闭包获取其使用的环境值的所有权，可以在参数列表前使用 `move` 关键字。

闭包所实现的trait 是由闭包所捕获了什么值而不是如何捕获所决定的。而 move 仅代表后者

***

#### 9.2.2.3 作为输入参数

闭包作为函数参数的时候，要求闭包是范型的，闭包的定义方式决定了这是必要的。

```rust
#![allow(unused)]
fn main() {
// `F` 必须是泛型的。
  fn apply<F>(f: F) where
      F: FnOnce() {
      f();
  }
}
```

闭包被定义的时候，编译器会隐式创建一个匿名类型的结构体，用以存储闭包捕获的变量，同时为这个未知类型的结构体实现函数功能，
通过 `Fn`, `FnMut`, `FnOnce` 三种 trait 中的一种。

如果使用闭包作为函数参数，由于这个结构体的类型未知，任何的用法都要求是范型的。然而， 使用未限定类型的参数<T>过于不明确，并且是不被允许的。
事实上，指明为该结构体实现的是 `Fn` , `FnMut` 或 `FnOnce` 中的哪种trait，对于约束改结构体的类型而言就已经足够了。

```rust
// `F` 必须为一个没有输入参数和返回值的闭包实现 `Fn`，这和对 `print` 的
// 要求恰好一样。
fn apply<F>(f: F) where
    F: Fn() {
    f();
}

fn main() {
    let x = 7;

    // 捕获 `x` 到匿名类型中，并为它实现 `Fn`。
    // 将闭包存储到 `print` 中。
    let print = || println!("{}", x);

    apply(print);
}

```

*   函数也可以作为参数
*   函数作为参数的时候同样需要满足trait需求

既然闭包可以作为参数，你很可能想知道函数是否也可以呢。确实可以！如果你声明一个接受闭包作为参数的函数，那么任何满足该闭包的 trait 约束的函数都可以作为其参数。

```rust
// 定义一个函数，可以接受一个由 `Fn` 限定的泛型 `F` 参数并调用它。
fn call_me<F: Fn()>(f: F) {
    f()
}

// 定义一个满足 `Fn` 约束的封装函数（wrapper function）。
fn function() {
    println!("I'm a function!");
}

fn main() {
    // 定义一个满足 `Fn` 约束的闭包。
    let closure = || println!("I'm a closure!");
    
    call_me(closure);
    call_me(function);
}
```

#### 9.2.2.4 闭包作为函数返回值

https://rustwiki.org/zh-CN/rust-by-example/fn/closures/output\_parameters.html

## 9.3 高阶函数

https://rustwiki.org/zh-CN/rust-by-example/fn/hof.html

## 9.4 发散函数

https://rustwiki.org/zh-CN/rust-by-example/fn/diverging.html

# 10. 模块

模块是 “项(item)” 的集合，项可以是：函数，结构体，trait，impl 块，甚至是其他模块

## 10.1 可见性

- 默认情况下，模块中的项具有私有的可见性
- pub 修饰语可以修改项的默认可见性
- 模块中只有 public 项可以从模块外的作用域访问。
- 模块使用 mod 修饰语进行修饰定义
- 同一模块中，项可以访问其他项，即使它是私有的
- 模块也可以嵌套
- `pub(in path)` 只能在指定的模块中访问
- `pub(self)`
- `pub(super)`
- 模块的私有部分不能随意访问，即使它嵌套在公有模块内

- 模块机制消除了相同名字的项之间的歧义
- 公有项，包括嵌套模块内的，都可以在父模块外部访问
- `pub(crate)` 项可以在同一个 crate 中的任何地方访问
- 

**问题**
- pub(in path) pub(self) pub(super) 各自的可访问性
- 

```rust
// 一个名为 `my_mod` 的模块
mod my_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // 使用 `pub` 修饰语来改变默认可见性。
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 在同一模块中，项可以访问其它项，即使它是私有的。
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 模块也可以嵌套
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my_mod::nested::private_function()`");
        }

        // 使用 `pub(in path)` 语法定义的函数只在给定的路径中可见。
        // `path` 必须是父模块（parent module）或祖先模块（ancestor module）
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            print!("called `my_mod::nested::public_function_in_my_mod()`, that\n > ");
            public_function_in_nested()
        }

        // 使用 `pub(self)` 语法定义的函数则只在当前模块中可见。
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested");
        }

        // 使用 `pub(super)` 语法定义的函数只在父模块中可见。
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        print!("called `my_mod::call_public_funcion_in_my_mod()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // `pub(crate)` 使得函数只在当前 crate 中可见
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()");
    }

    // 嵌套模块的可见性遵循相同的规则
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my_mod::private_nested::function()`");
        }
    }
}

fn function() {
    println!("called `function()`");
}

fn main() {
    // 模块机制消除了相同名字的项之间的歧义。
    function();
    my_mod::function();

    // 公有项，包括嵌套模块内的，都可以在父模块外部访问。
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个 crate 中的任何地方访问
    my_mod::public_function_in_crate();

    // pub(in path) 项只能在指定的模块中访问
    // 报错！函数 `public_function_in_my_mod` 是私有的
    //my_mod::nested::public_function_in_my_mod();
    // 试一试 ^ 取消该行的注释

    // 模块的私有项不能直接访问，即便它是嵌套在公有模块内部的

    // 报错！`private_function` 是私有的
    //my_mod::private_function();
    // 试一试 ^ 取消此行注释

    // 报错！`private_function` 是私有的
    //my_mod::nested::private_function();
    // 试一试 ^ 取消此行的注释

    // Error! `private_nested` is a private module
    //my_mod::private_nested::function();
    // 试一试 ^ 取消此行的注释
}
```


## 10.2 结构体的可见性

- 解构体与结构体中的字段也可以被 pub 语句修饰

https://rustwiki.org/zh-CN/rust-by-example/mod/struct_visibility.html

## 10.3 use 声明

- 将一个完整的路径绑定你个到一个新的名字，更加容易访问

https://rustwiki.org/zh-CN/rust-by-example/mod/use.html#use-%E5%A3%B0%E6%98%8E


```rust
use deeply::nested::function as other_function;
```

## 10.4 super 和 self

https://rustwiki.org/zh-CN/rust-by-example/mod/super.html

## 10.5 文件分层

就是以合适的文件目录结构组织各个模块 

- mode my 用于声明一个模块。此声明会查找名为 my.rs 或者 my/mod.rs 的文件，并将该文件放到
  名为 my 的模块中 


```rust
// 此声明将会查找名为 `my.rs` 或 `my/mod.rs` 的文件，并将该文件的内容放到
// 此作用域中一个名为 `my` 的模块里面。
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();

    function();

    my::indirect_access();

    my::nested::function();
}

```



# 11. crate 

crate 是 Rust 的编译单元。当调用`rustc some_file.rs` 时，some_file.rs 被当作 crate 文件。如果 some_file.rs 里面
含有mod声明，那么模块文件的内容将会在编译之前被插入crate 文件的相应声明处。

模块不会被单独编译，只有crate才会被编译。

crate 可以编译成二进制可执行文件（binary）或库文件（library）。

默认情况下，rustc 将从crate产生二进制可执行文件。这种行为可以通过 rustc 的选项 --crate-type 重载


## 11.1 库

```shell
rustc --crate-type=lib rary.rs
ls lib*
library.rlib
```

执行后会生成一个 rlib 文件。默认情况下库会使用 crate 文件的名字，前面加上 'lib' 的前缀。
但是这个默认名称可以通过 `create_name` 属性覆盖

在使用 cargo 的时候，这两种类型都不会起作用。

```rust
// 这个 crate 是一个库文件
#![crate_type = "lib"]
// 库的名称为 “rary”
#![crate_name = "rary"]

pub fn public_function() {
    println!("called rary's `public_function()`");
}

fn private_function() {
    println!("called rary's `private_function()`");
}

pub fn indirect_access() {
    print!("called rary's `indirect_access()`, that\n> ");

    private_function();
}

```

当使用 `crate_type` 属性时，就不用再给 `rustc` 命令加上 `--crate-type`标记。

```shell
$ rustc lib.rs
$ ls lib*
library.rlib
```

## 11.2 使用库

- 使用 `rustc --extern` 选项，将一个crate链接到一个库

```rust
// executable.rs
// extern crate rary; // 在 Rust 2015 版或更早版本需要这个导入语句

fn main() {
    rary::public_function();

    // 报错！ `private_function` 是私有的
    //rary::private_function();

    rary::indirect_access();
}

```

```shell
# library.rlib 是已编译好的库的路径，这里假设它在同一目录下：
$ rustc executable.rs --extern rary=library.rlib --edition=2018 && ./executable 
called rary's `public_function()`
called rary's `indirect_access()`, that
> called rary's `private_function()`

```

执行了 --extern 之后，再次 rustc executable.rs 仍会报错，找不到对应的模块
但是执行 ./executable 文件，会正常执行

--extern 应该是只会影响编译结果



# 12. cargo

- 依赖管理与 crates.io 继承
- 方便的单元测试
- 方便的基准测试

## 12.1 依赖

- cargo new foo 创建二进制可执行文件
- cargo new --lib foo 创建库
- cargo build 
- cargo run 也会构建下载的 crate

依赖添加的几种方式

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # 来自 crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # 来自网上的仓库
bar = { path = "../bar" } # 来自本地文件系统的路径
```

构建项目的时候可以在项目目录的任何位置执行 `cargo build`

也可以执行 cargo run 来构建和运行。  

这些命令将处理所有依赖，在需要的时候下载crate，并构建所有内容，包括crate。

## 12.2 约定规范

- 如果有多个二进制可执行文件，需要放在 `src/bin/` 目录下。
- 如果想执行指定的二进制可执行文件： cargo run --bin my_other_bin

cargo 创建的项目，默认二进制名称是main，但是可以通过将文件放在 `bin/` 目录中来添加其他二进制可执行文件

```shell
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs

```

## 12.3 测试

[Rust圣经：编写自动化测试](https://rustwiki.org/zh-CN/book/ch11-00-testing.html)

- 目录组织结构，将单元测试放在需要测试的模块中，将集成测试放在 `tests/` 目录中

```shell
foo
├── Cargo.toml
├── src
│   └── main.rs
└── tests
    ├── my_test.rs
    └── my_other_test.rs

```

- tests 目录下每个文件都是一个单独的集成测试
- cargo test 运行所有测试 
- cargo test test_foo 名称匹配一个模式
- cargo 可能会同时进行多项测试，因此需要确认它们不会相互竞争。例如，如果它们都输出到文件，
  则应该将它们写入不同的文件
- 如果有另外的二进制可执行文件想要执行 test，可以执行 cargo test --bin my_other_bin

**问题**
- 如何执行工程中的所有模块的测试？
- 如何执行文档测试？

## 12.4 构建脚本

为了补充 cargo build 的正常构建而查u你感觉爱你。也许crate在cargo成功编译之前需要一些先决条件,
比如代码生成或者需要编译的一些本地代码。

```toml
[package]
...
build = "build.rs"
```

这里的cargo会先在项目目录中优先查找 `build.rs` 文件。


### 12.4.1 使用构建脚本

cargo 通过[此处指定](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts)
的可以使用的环境变量为脚本提供输入。

此脚本通过 stdout 提供 输出。打印的所有行都写入到 `/target/debug/build/<pkg>/output` .

以`cargo:`为前缀的行将由 cargo 直接解析，因此可用于定义包编译的参数




# 13. 属性

Rust 中的属性是应用于某些模块、crate或项的元数据。

- 条件编译代码
- 设置 crate 名称、版本和类型（二进制文件或库）
- 禁用 lint （警告）
- 启用编译器的特性（宏、全局导入（glob import）等）
- 链接到一个非 Rust 语言的库
- 标记函数作为单元测试
- 标记函数作为基准测试的某个部分

属性对于不同的作用对象，语法上有一些差别

当属性作用于整个 crate 时，它们的语法为 #![crate_attribute]，
当它们用于模块 或项时，语法为 #[item_attribute]（注意少了感叹号 !）。

属性可以接收参数，有不同的语法形式

- #[attribute = "value"]
- #[attribute(key = "value")]
- #[attribute(value)]

属性可以有多个值，也可以分散到多行中。

```rust
#[attribute(value, value2)]

#[attribute(value, value2, value3,
            value4, value5)]

```

## 13.1 dead_code

- #[allow(dead_code)]

https://rustwiki.org/zh-CN/rust-by-example/attribute/unused.html

## 13.2 crate

crate_type 
crate_name 

https://rustwiki.org/zh-CN/rust-by-example/attribute/crate.html

## 13.3 cfg

这个是用来进行条件编译的

cfg 可能通过两种不同的操作符实现

- cfg 属性：在属性位置使用 #[cfg(...)]
- cfg! 宏：在布尔表达式位置使用 cfg!(...)
- #[cfg(target_os = "linux")]: 当目标系统是Linux的时候才会编译
- #[cfg(not(target_os = "linux"))]: 目标系统不是 Linux 的时候才会编译
- 是否为Linux的编译判断应用对象可以是相同的方法名



```rust 
// 这个函数仅当目标系统是 Linux 的时候才会编译
#[cfg(target_os = "linux")]
fn are_you_on_linux() {
    println!("You are running linux!")
}

// 而这个函数仅当目标系统 **不是** Linux 时才会编译
#[cfg(not(target_os = "linux"))]
fn are_you_on_linux() {
    println!("You are *not* running linux!")
}

fn main() {
    are_you_on_linux();
    
    println!("Are you sure?");
    if cfg!(target_os = "linux") {
        println!("Yes. It's definitely linux!");
    } else {
        println!("Yes. It's definitely *not* linux!");
    }
}

```
**问题**
- 除了 target_os 还有其他的语法么

### 13.3.1 自定义条件

```rust
#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}

```

```shell
不使用自定义的 cfg
$ rustc custom.rs && ./custom
No such file or directory (os error 2)

$ rustc --cfg some_condition custom.rs && ./custom
condition met!

```

不是用 cfg 标记的时候，找不到标记的函数

- 如何做到像 target_os 一样传递参数
- 需要注意的是，参数左右的引号可能需要使用反斜杠包裹

```rust
#[cfg(some_condition = "add")]
fn conditional_function() {
    println!("condition met!")
}

fn main() {
    conditional_function();
}

```

```shell
rustc --cfg some_condition=\"add\" custom_condition.rs
```

