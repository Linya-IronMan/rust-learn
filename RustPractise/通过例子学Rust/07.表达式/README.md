
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


