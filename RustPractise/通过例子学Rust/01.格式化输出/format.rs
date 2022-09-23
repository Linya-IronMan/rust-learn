use std::fmt::{self, Formatter, Display };

struct City {
    name: &'static str, 
    lat: f32,
    lon: f32,
}
impl Display for City{
    // 为什么 f 参数的类型是 fmt::Formatter 的引用？
    fn fmt (&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >=0.0 { 'E' } else { 'W' };

        write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)

    }
}

#[derive(Debug)]
struct Inch(i32);
fn main() {
    // for city in [
    //     City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
    //     City { name: "Oslo", lat: 59.95, lon: 10.75 },
    //     City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    // ].iter() {
    //     println!("{}", *city);
    // }
    println!("结构体打印：{:?}", Inch(32));
}

