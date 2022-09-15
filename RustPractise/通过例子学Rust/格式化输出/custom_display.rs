use std::fmt;
struct Structure(i32);
impl fmt::Display for Structure {
    // 为什么 f 参数的类型是 fmt::Formatter 的引用？
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HEllo world: {}", self.0)
    }
}
fn main() {
    println!("{}", Structure(3123123));
}

