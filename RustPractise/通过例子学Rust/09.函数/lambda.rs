fn main() {
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i+1;
    println!("{}, {}", closure_inferred(1), closure_annotated(5));
}
