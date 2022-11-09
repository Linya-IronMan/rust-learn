fn main() {
    let numbers = [1, 2, 3, 4, 5];

    for i in &numbers {
        println!("&: {}", i);
    }


    for i in 0..numbers.len(){
        println!("len: {}", numbers[i]);
    }

    for i in numbers.iter() {
        println!("iter: {}", i);
    }
}
