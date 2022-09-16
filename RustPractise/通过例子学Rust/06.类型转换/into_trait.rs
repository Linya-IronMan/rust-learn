

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
    let i = 3222;
    let num: Number = i.into();

    println!("{:?}", num);
}
