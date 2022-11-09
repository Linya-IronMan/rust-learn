use rand::{thread_rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("猜数！");
    // 在本地线程空间获取随机数的种子, 包含 1 不包含 101

    // let mut secret_number = thread_rng();
    // secret_number = secret_number.gen_range(0, 10);
    let mut rng = thread_rng();
    let secret_number: u32 = rng.gen_range(1, 101);

    println!("生成的随机数 {}", secret_number);

    loop {
        println!("猜测一个数");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("无法读取行");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜测的数是：{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
