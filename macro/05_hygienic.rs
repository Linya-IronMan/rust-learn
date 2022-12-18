macro_rules! expr_from_1 {
    ($expr: expr) => {{
        let one = 1;
        $expr
    }};
}

fn main() {
    println!("{}", expr_from_1!(one * 3));
}
