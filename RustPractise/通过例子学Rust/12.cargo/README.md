
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




