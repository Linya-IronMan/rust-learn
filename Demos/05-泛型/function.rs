fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut larger = list[0];

    for &item in list.iter() {
        if item > larger {
            larger = item;
        }
    }
    larger
}

fn main() {
    let number_list = vec![1, 2, 23, 24, 8, 100, 88];
    let char_list = vec!['c', 'a', 'z' ,'e', '3', 't'];
    let max_number = largest(&number_list);
    println!("largest number: {}", max_number);

    let max_char = largest(&char_list);
    println!("largest char: {}", max_char);
}
