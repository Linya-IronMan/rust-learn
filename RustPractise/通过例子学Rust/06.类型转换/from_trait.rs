
#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(value: i32) -> Number {
        Number{value}
    }
}

fn main() {
    let num = Number::from(2222);
    println!("{:?}", num);
}
