macro_rules! recurrence {
    (@count ) => { 0_usize };
    (@count $_: expr $(, $tail: expr)*) => {
        1 + recurrence!(@count $($tail), *)
    }
}

fn main() {
    println!("Hello, world!");
}
