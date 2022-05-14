fn main() {
    let mut s = String::from("Hello world");
    let mut wordIndex = first_world(&mut s);

    s.clear();

    println!("{}", wordIndex);
}

fn first_world(s: &mut String) -> &str {
    let bytes = s.as_bytes();
    // 问题：函数内部定义了一个 mut 可变变量并返回，函数外部接收的参数是否可变？
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
