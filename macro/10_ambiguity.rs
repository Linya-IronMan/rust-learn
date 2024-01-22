macro_rules! amb {
    ($($i:expr),*) => {
        println!("last item: {}", $(i )*);
    }; // ($($i:expr),* $j:expr) => {
       //     println!("last item: {}", $j);
       // };
}

fn main() {
    amb!(123, 222, 3423, 2222);
}
