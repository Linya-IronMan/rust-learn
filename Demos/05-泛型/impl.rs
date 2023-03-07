#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn get_x(&self) -> &T {
        &self.x
    }

    fn get_y(&self) -> &T {
        &self.y
    }
}

fn main() {
    let p = Point{x: 1, y: 2};
    println!("x = {}", p.get_x());
    println!("y = {}", p.get_y());

    let p1 = Point{x: 1.1, y: 2.1};
    println!("x1 = {}", p1.get_x());
    println!("y1 = {}", p1.get_y());
}
