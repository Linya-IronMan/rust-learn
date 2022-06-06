use std::error::Error;
// collect
use std::fs;

// Box<dyn Error> 表示一个实现了 Error trait 的类型，但是无需指明具体类型
// 如此，可以在不同的场景下返回不同的错误
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // ? 作用：在发生错误时不会出现恐慌，会将错误信息返回给函数的调用者
  let contents = fs::read_to_string(config.filename)?;
  println!("With text:\n{}", contents);
  Ok(())
}

pub struct Config {
  pub query: String,
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    Ok(Config { query, filename })
  }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  // vec![]
  let mut results = Vec::new();
  for line in content.lines() {
    if (line.contains(query)) {
      results.push(line)
    }
  }
  results
}
#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn one_result() {
    let query = "duct";
    let contents = "\
Rust:
Safe, fast, productive.
Pick three.";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents))
  }
}
