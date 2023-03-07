#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

fn main() {
    let int = Point{x: 1, y: 3};

    println!("{:#?}", int);
}
