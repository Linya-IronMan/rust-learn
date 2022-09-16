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


