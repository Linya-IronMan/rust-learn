use std::sync::Mutex;
use super::models::Course;

pub struct AppState {
    // 共享与所有线程 初始化之后这是不可变的state
    pub health_check_response: String,
    // 共享访问次数 Mutex 是Rust标准库提供的进程间安全通信的机制
    // 修改这个值之前，此线程需要获得这个数据的控制权，这件事就是通过 Mutex来保证的
    pub visit_count: Mutex<u32>,

    pub courses: Mutex<Vec<Course>>,
}
