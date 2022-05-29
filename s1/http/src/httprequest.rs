// derive: 派生
#[derive(Debug, PartialEq)]
pub enum Method {
  Get,
  Post,
  Uninitialized,
}

impl From<&str> for Method {
  fn from(s: &str) -> Method {
    match s {
      "GET" => Method::Get,
      "POST" => Method::Post,
      _ => Method::Uninitialized,
    }
  }
}

// 条件编译，在测试的时候才会编译执行
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_method_info() {
    let m: Method = "GET".into();
    assert_eq!(m, Method::Get);
  }
}
