use std::ops::Deref;

fn main() {
    let a = vec![1, 2, 3, 4];
    println!("a: {:?}", a);
    let b = a.deref();
    println!("b: {:?}", b);

    let c = &a;
}
