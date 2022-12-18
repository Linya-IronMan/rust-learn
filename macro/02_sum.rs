macro_rules! sum {
  ($($num: expr), *) => {
    0 $(+$num)*
  };
}

fn main() {
    println!("{}", sum!());
    println!("{}", sum!(1));
    println!("{}", sum!(1, 2));
    println!("{}", sum!(1, 2, 3));
    println!("{}", sum!(1, 2, 3, 4));
}
