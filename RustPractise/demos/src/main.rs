fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let b = a.into_iter();
    let result: Vec<i32> = b.filter(|&x| x <= 3).collect();
}
