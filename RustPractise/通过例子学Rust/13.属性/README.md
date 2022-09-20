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

