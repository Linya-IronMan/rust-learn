use std::mem;

#[derive(Debug)]
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String },
}

fn convert_a_to_b(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            // 没有重新分配内存
            name: mem::take(name),
        }
    }
}

fn convert_a_to_b2(e: &mut MyEnum) {
    if let MyEnum::A { name, x: 0 } = e {
        *e = MyEnum::B {
            // 会将原本的 name 使用String::new() 替换,并且返回原本的 name
            name: mem::replace(name, String::new()),
        }
    }
}

fn main() {
    println!("Hello, world!");
}
