use std::fmt;
struct Structure(Vec<i32>);
impl fmt::Display for Structure {
    // 为什么 f 参数的类型是 fmt::Formatter 的引用？
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;
        for (i, v) in vec.iter().enumerate() {
            if i != 0 {write!(f, ", ")?;}
            write!(f, "{}", v)?;
        }
        write!(f, "]")
    }
}
fn main() {
    println!("{}", Structure(vec![1, 2, 3, 4]));
}

