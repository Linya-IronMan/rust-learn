use std::error::Error;
// collect
use std::fs;
// 标准库中的环境变量
use std::env;

// Box<dyn Error> 表示一个实现了 Error trait 的类型，但是无需指明具体类型
// 如此，可以在不同的场景下返回不同的错误
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  // ? 作用：在发生错误时不会出现恐慌，会将错误信息返回给函数的调用者
  let contents = fs::read_to_string(config.filename)?;
  let results = if config.case_sensitive {
    search(&config.query, &contents)
  } else {
    search_case_insensitive(&config.query, &contents)
  };

  for line in results {
    println!("||: {}", line)
  }
  // println!("With text:\n{}", contents);
  Ok(())
}

pub struct Config {
  pub query: String,
  pub filename: String,
  pub case_sensitive: bool,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
    if args.len() < 3 {
      return Err("not enough arguments");
    }
    let query = args[1].clone();
    let filename = args[2].clone();
    // 环境变量只要出现就行
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    Ok(Config {
      query,
      filename,
      case_sensitive,
    })
  }
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
  // vec![]
  let mut results = Vec::new();
  for line in content.lines() {
    if line.contains(query) {
      results.push(line)
    }
  }
  results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();
  let query = query.to_lowercase();

  for line in contents.lines() {
    if line.to_lowercase().contains(&query) {
      results.push(line)
    }
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Duct tape.
";
    assert_eq!(vec!["Safe, fast, productive."], search(query, contents))
  }

  #[test]
  fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive. 
Pick therr
Trust me.
";

    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    )
  }
}
