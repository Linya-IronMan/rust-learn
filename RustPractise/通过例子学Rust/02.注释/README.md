


要运行测试，首先将代码构建为库，然后告诉 rustdoc 在哪里找到库，这样它就可以 使每个文档中的程序链接到库：

```rust
$ rustc doc.rs --crate-type lib
$ rustdoc --test --extern doc="libdoc.rlib" doc.rs
```

文档注释的效果应当是在引用lib的时候才有效，暂时就不去做测试了。留待以后进行。


