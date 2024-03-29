use std::fmt;
struct Circle {
    radius: i32,
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "circie of radius {}", self.radius)
    }
}

fn main() {
    let circle = Circle { radius: 6 };
    let a = circle.to_string();
    println!("a: {}", a);
    println!("circle: {}", circle);
}
