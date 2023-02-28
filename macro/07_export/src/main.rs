// #[macro_use]
mod macros;
// #[macro_use] extern crate macro_lib;
use macro_lib::m; 
use macros::b::test;


fn main() {
    let lib = recurrence!(items:u64 => 0, 1, 3, 4; ... ; a[n] = a[n-3] + a[n-4]);
    m!();
    test{};
    for (index, val) in lib.take(1).enumerate() {
        println!("F[{}] = {}", index, val);
    }
}
 