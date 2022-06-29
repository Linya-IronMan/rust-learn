#[derive(Debug, PartialEq)]
pub enum Method {
  Get,
  Post,
  Uninitialized,
}

// #[derive(Method)]
impl From<&str> for Method {
  fn from(s: &str) -> Method {
    match s {
      "GET" => Method::Get,
      "POST" => Method::Post,
      _ => Method::Uninitialized,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_method_into() {
    // 这里涉及到类型的转换
    // 我们在Method类型上实现了from这个trait用来做类型的转换
    // 此处调用的时候，会通过类型标注获取转换的目标类型。
    // 然后在目标类型上查找转换的方法 from 这个trait
    // 也可以写成.. 下面的写法，这种写法就不用显示标记目标类型了，因为可以自动推导而来
    // let m = Method::from("GET");
    let m: Method = "GET".into();
    assert_eq!(m, Method::Get);
  }
}
