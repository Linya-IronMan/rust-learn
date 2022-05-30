

# 接收命令行参数

## Vector

- 由标准库提供
- 可存储多个值
- 只能存储相同类型的数据
- 值在内存中连续存放

**Vector 创建**
- Vect::new
- vec! 宏

## `std::env::args().collect()`

`std::env::args()` 获取一个迭代器, 包含用户的输出信息。但是无法获取非法的字符。想要获取非法字符可以使用 `env::OsString`

获取到的第一个元素是二进制文件的名称，后面的才是传给程序的真实的参数

```rust
let query = &args[1];
let filename = &args[2];
```

`std::env::args().collect()` 将迭代器转成一个集合

# 读取文件

`std::fs::read_to_string(filepath)`

```rust
let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
```

返回值是 `Result<String>` 所以可以使用 `expect`。如果结果是 Ok，那么就会获取到文件内容，如果出错，就会爆发 `panic`

# 重构

**问题：**
- main 函数现在既负责参数解析，又负责读取文件
- main query filename 变量离散，在之后的扩展中程序复杂，变量越来越多，这样关联性较强的变量最好是存放在一个结构体中。
- 文件读取失败时，需要完善错误处理信息提示
- 如果没有指定参数，用户只能获取 rust 内部的错误信息，用户无法判断错误。需要将错误信息放在一处处理，方便管理，并且放在一起能够确保为用户打印的信息是易于理解的。

```rust
use std::env; // collect
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    let query = &args[1];
    let filename = &args[2];

    println!("Search for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```



## 二进制程序关注点分离的指导性原则

- 将程序拆分为 main.rs 和 lib.rs，将业务逻辑放在 `lib.rs`
- 当命令行解析逻辑较少时，将它放在 `main.rs` 也行
- 当命令行 解析逻辑变复杂时，需要将它从 `main.rs` 提取到 `lib.rs`

留在 main 的功能有：
- 使用参数值调用命令行解析逻辑
- 进行其他配置
- 调用 `lib.rs` 中的 run 函数
- 处理 run 函数可能出现的错误

## 错误处理

程序是要供人使用的，错误提示只有程序员自己看懂还不行，需要让用户也同样能够看懂。

程序的错误分为两类
- 一类程序内部的错误，只用于程序员调试所用
- 一类与用户的操作有关，当用户的操作不在程序的预期之内，需要做出提示

`panic!` 做出错误提示的时候会包含很多无用信息，需要换种方式处理 =》 Result


使用 Result 包裹返回结果，在 Result 的结果之上做出错误处理的判断。

```rust
impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();
        Ok(Config { query, filename })
    }
}

```

值得注意的是，函数的返回值是 Result 的时候，返回结果要么使用 Ok，要么使用 Err 进行包裹。

```rust
use std::process;
let config = Config::new(&args).unwrap_or_else(|err| {
    println!("Problem parsing arguments: {}", err);
    process::exit(1);
});
```

这里的 `unwrap_or_else` 是 Result 上的方法。如果是 Ok，就会返回 Ok 结果；如果结果是 Err，就会执行回调函数。

参数就是一个回调函数

```rust
|err| {
    println!("Problem parsing arguments: {}", err);
        process::exit(1);
}
```
这里是一种管道语法，将 err 注入到花括号中使用






