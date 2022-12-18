// Inits
// ...
// 递推关系

macro_rules! recurrence {
    ($($inits: expr),* ; ... ; $seq: ident[$idx: ident] = $recur:expr) => {
        {

        }
    };
}

fn main() {
    let fib = {
        // 1. 定义结构体
        struct Recurrence {

        }
        // 2. 实现 Iterator trait
        impl Iterator fro Recurrence {
            todo!(); 
        }
        // 3. 返回一个实例给 fib
        Recurrence {
            todo!();
        }
    }

    let seq = recurrence!(1, 2; ... ; f[n] = f[n-1] + f[n-2]);
    for let i in seq.take(10) {
        println!("{}", i);
    }
}
