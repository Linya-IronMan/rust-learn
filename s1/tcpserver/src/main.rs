use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    // 建立监听
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    println!("Running on port 3000 ...");

    // 读取监听到的数据
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established!");
        // 新建长度为 1024 的数组，默认填充 0
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
