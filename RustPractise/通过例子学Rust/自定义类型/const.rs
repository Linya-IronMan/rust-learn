static mut LANGUAGE: &'static str = "Rust";
#[allow(dead_code)]
const NUMBER: i32 = 10;

fn main() {
    unsafe {
        LANGUAGE = "JavaScript";
        println!("{}", LANGUAGE);
    }
}
