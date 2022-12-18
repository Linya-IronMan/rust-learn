macro_rules! expr_from {
    ($expr: expr) => {{
        let one = 1;
        $expr
        // 此处即使替换成了 one * 5 和上面的 one 也是不一样的，因为他们的上下文不一样。 args::one
        // 在AST上看可能一样，但是在编译器上来看，他们并不一样。
        // 问题：代码展开之后的编译流程是什么？
    }};
}

fn main() {
    // macro 代码的展开是携带上下文的展开
    println!("{} ==>>>", expr_from!(one * 5))
}
