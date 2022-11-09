fn main() {
    let orig = Box::new(5);
    println!("{}", *orig);
    let stolen = orig; 
    println!("{}, {}", stolen, orig); 
}
