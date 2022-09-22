# 14 范型

https://rustwiki.org/zh-CN/rust-by-example/generics/gen\_fn.html

## 14.1 函数

*   需要注意，<i32> 这样是类型明确的，并不能说使用这样类型的函数是一个范型函数
*   除了函数，结构体也是可以使用范型的

```rust
struct A;          // 具体类型 `A`。
struct S(A);       // 具体类型 `S`。
struct SGen<T>(T); // 泛型类型 `SGen`。
// 定义一个函数 `gen_spec_i32`，接受一个 `SGen<i32>` 类型的参数 `_s`。
// `SGen<>` 显式地接受了类型参量 `i32`，而 `i32` 是一个具体类型。
// 由于 `i32` 不是一个泛型类型，所以这个函数也不是泛型的。
fn gen_spec_i32(_s: SGen<i32>) {}
```

一个范型函数需要显示定义范型类型，并且在参数或者返回值中接收这个范型

```rust
// 定义一个函数 `generic`，接受一个 `SGen<T>` 类型的参数 `_s`。
// 因为 `SGen<T>` 之前有 `<T>`，所以这个函数是关于 `T` 的泛型函数。
fn generic<T>(_s: SGen<T>) {}
```

```rust
// 定义一个函数 `gen_spec_t`，接受一个 `SGen<A>` 类型的参数 `_s`。
// `SGen<>` 显式地接受了类型参数 `A`，且在 `gen_spec_t` 中，`A` 没有被用作
// 泛型类型参数，所以函数不是泛型的
fn gen_spec_t(_s: SGen<A>) {}

```

## 14.2 实现

https://rustwiki.org/zh-CN/rust-by-example/generics/impl.html

## 14.3 trait

可以将实现trait的对象设置为范型

```rust
// 对泛型的调用者类型 `U` 和任何泛型类型 `T` 实现 `DoubleDrop<T>` 。
impl<T, U> DoubleDrop<T> for U {
    // 此方法获得两个传入参数的所有权，并释放它们。
    fn double_drop(self, _: T) {}
}
```

## 14.4 约束

使用范型的时候，类型参数会使用 trait 作为“约束”，来明确规定类型应该实现哪些功能。

```rust
// 定义一个函数 `printer`，接受一个类型为泛型 `T` 的参数，
// 其中 `T` 必须实现 `Display` trait。
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

```

**问题**

*   如何不使用范型进行约束？
*

### 14.4.1 测试实例：空约束

即使一个 trait 不包含任何功能，仍然可以用它作为约束。标准库中的Eq和Ord就是这样的trait

```rust
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

// 这些函数只对实现了相应的 trait 的类型有效。
// 事实上这些 trait 内部是空的，但这没有关系。
fn red<T: Red>(_: &T)   -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    // 由于约束，`red()` 不能作用于 blue_jay （蓝松鸟），反过来也一样。
    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    //println!("A turkey is {}", red(&_turkey));
    // ^ 试一试：去掉此行注释。
}
```

**问题**

*   这样的约束有什么作用？

## 14.5 多种约束 

多重约束使用 + 连接。

```rust
use std::fmt::{Debug, Display};
fn compare_prints<T: Debug + Display>(t: &T) {}
```

## 14.6 where 子句

- 当分别指定范型的类型和越是时会更加清晰。

```rust
impl <A: TraitB + TraitC, D: TraitE + TraitF> MyTrait<A, D> for YourType {}

// 使用 `where` 从句来表达约束
impl <A, D> MyTrait<A, D> for YourType where
    A: TraitB + TraitC,
    D: TraitE + TraitF {}
```

- 使用 where 会更有表现力，有些时候，某些类型不是用 where 从句，就无法直接表达。

- 除了直接对范型添加约束，也可以使用 Option 这样的类型，将范型包裹后对整体添加约束

```rust 
use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

// 这里需要一个 `where` 从句，否则就要表达成 `T: Debug`（这样意思就变了），
// 或者改用另一种间接的方法。
impl<T> PrintInOption for T where
    Option<T>: Debug {
    // 我们要将 `Option<T>: Debug` 作为约束，因为那是要打印的内容。
    // 否则我们会给出错误的约束。
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn main() {
    let vec = vec![1, 2, 3];

    vec.print_in_option();
}
```


## 14.7 new type 惯用法

?? 这一章在说什么 
https://rustwiki.org/zh-CN/rust-by-example/generics/new_types.html

## 14.8 关联项

指的是与多种类型的项有关的一组规则。他是 trait 范型的扩展，允许在trait内部定义新的项

一种将类型占位符与 trait 联系起来的做法，这样trait中的方法签名就可以使用这些占位符类型。
trait的实现会指定在该实现中那些占位符对应什么具体类型。

### 14.8.1 存在问题

trait 如果对实现了它的容器类型是范型的，则trait的使用者必须指出 trait 的全部范型类型。

https://rustwiki.org/zh-CN/rust-by-example/generics/assoc_items/the_problem.html

### 14.8.2 关联类型

将容器内部的类型放到 trait 中作为输出类型，使用 “关联类型”增加了代码的可读性。

```rust

#![allow(unused)]
fn main() {
// `A` 和 `B` 在 trait 里面通过 `type` 关键字来定义。
// （注意：此处的 `type` 不同于为类型取别名时的 `type`）。
trait Contains {
    type A;
    type B;

    // 这种语法能够泛型地表示这些新类型。
    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
}
}

```

使用了 Contains trait 的函数就不需要写出 A 或 B 了。

```rust
// 不使用关联类型
fn difference<A, B, C>(container: &C) -> i32 where
    C: Contains<A, B> { ... }

// 使用关联类型
fn difference<C: Contains>(container: &C) -> i32 { ... }
```

- 使用关联类型将一些范型内置在 trait 之后，对于其他类型进行约束时，就可以省略原本写在外面的范型
- trait 中的方法调用范型类型的时候 `&Self::A` 如此调用

```rust
struct Container(i32, i32);

// 这个 trait 检查给定的 2 个项是否储存于容器中
// 并且能够获得容器的第一个或最后一个值。
trait Contains {
    // 在这里定义可以被方法使用的泛型类型。
    type A;
    type B;

    fn contains(&self, _: &Self::A, _: &Self::B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains for Container {
    // 指出 `A` 和 `B` 是什么类型。如果 `input`（输入）类型
    // 为 `Container(i32, i32)`，那么 `output`（输出）类型
    // 会被确定为 `i32` 和 `i32`。
    type A = i32;
    type B = i32;

    // `&Self::A` 和 `&Self::B` 在这里也是合法的类型。
    fn contains(&self, number_1: &i32, number_2: &i32) -> bool {
        (&self.0 == number_1) && (&self.1 == number_2)
    }

    // 得到第一个数字。
    fn first(&self) -> i32 { self.0 }

    // 得到最后一个数字。
    fn last(&self) -> i32 { self.1 }
}

fn difference<C: Contains>(container: &C) -> i32 {
    container.last() - container.first()
}

fn main() {
    let number_1 = 3;
    let number_2 = 10;

    let container = Container(number_1, number_2);

    println!("Does container contain {} and {}: {}",
        &number_1, &number_2,
        container.contains(&number_1, &number_2));
    println!("First number: {}", container.first());
    println!("Last number: {}", container.last());
    
    println!("The difference is: {}", difference(&container));
}
```

## 14.9 虚类型参数

虚类型：phantom type 
这是一种在运行时不出现，而在编译时进行静态检查的类型参数。

**问题** 这有什么用？

https://rustwiki.org/zh-CN/rust-by-example/generics/phantom.html

不懂！

### 14.9.1 测试实例：单位说明

https://rustwiki.org/zh-CN/rust-by-example/generics/phantom/testcase_units.html


