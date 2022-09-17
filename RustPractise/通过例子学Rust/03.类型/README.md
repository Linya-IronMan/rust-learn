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


