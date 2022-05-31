use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);

    // let query = &args[1];
    // let filename = &args[2];

    // unwrap 表示打开包装，这种错误处理在需要从返回结果中提取某些值得时候使用
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
