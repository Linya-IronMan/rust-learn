
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


