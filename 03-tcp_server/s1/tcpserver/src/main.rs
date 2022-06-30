use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    // 只会接收一次信息
    // let result = listener.accept().unwrap();

    // incoming 会返回一个迭代器，这个迭代器会舰艇listener上的连接
    // 每一个连接就代表一个字节流，这个字节流的类型是TcpStream，数据就能在TcpStream上传送和接收
    // 对 TCPStream的读写，是通过原始字节来完成的
    for stream in listener.incoming() {
        // stream的类型是Result，那么就使用unwrap简单处理下
        let mut stream = stream.unwrap();
        println!("Connection established!");
        let mut buffer = [0; 1024];

        stream.read(&mut buffer).unwrap();
        stream.write(&mut buffer).unwrap();
    }
}
