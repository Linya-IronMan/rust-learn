fn main() {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for i in 0..5 {
        map.insert(i, i)
    }
    println!("{}", map);
}
