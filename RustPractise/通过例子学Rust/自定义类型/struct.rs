#[derive(Debug)]
struct Rectangle(f32, f32);
struct Point {
    x: f32,
    y: f32,
}
fn main() {
    let rec = Rectangle(10f32, 10f32);

    fn rect_area(target: &Rectangle) {
        let Rectangle(width, height) = target;
        println!("{} {}", height, width);
    }
    
    fn square(point: &Point, size: f32) -> Rectangle {
        Rectangle(point.x + size, point.y + size)
    }

    rect_area(&rec);

    let point = Point{x: 1f32, y: 2f32,};
    
    println!("{:?}", square(&point, 10f32));
}
