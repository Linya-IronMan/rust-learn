# Web Server 

- Server 
  监听进来的TCP字节流
- Router
  接受HTTP请求，并决定调用哪个Handler
- Handler
  处理HTTP请求，构建HTTP相应
- HTTP Library
  - 解释字节流，把它转化为 HTTP 请求
  - 把HTTP相应转化回字节流

服务端的 `TcpListener::bind("...")` 能够接收到 client 传过来的字节流。通过 HTTP Library 转换成HTTP请求之后，交给Router路由分配对应的处理方法。

经过 handler 处理过之后，再将处理结果交由HTTP Library 转成字节流并流转到Server向客户端传递消息。

# 构建步骤

- 解析HTTP请求消息
- 构建HTTP响应消息
- 路由与Handler
- 测试 Web Server

## 数据结构指定


| 数据结构名称 | 数据类型 | 描述 |
| -------- | ----- |-- |
|HttpRequest | struct | 表示HTTP请求 |
| Method | enum | 指定所允许的HTTP方法 |
| Version | enum | 指定所允许的HTTP版本 |


**需要实现的Trait**

| Trait | 描述 |
| --- | --- |
| From<&str> | 用于将传进来的字符串切片转化为 HttpRequest |
| Debug | 打印调试信息  |
| PartialEq | 用于解析和自动化测试脚本里做比较 |

# 解析 HTTP 请求

就是从HTTP消息文本中按照一定的规则解析出 method，version，resource以及headers等字段

`GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n`

这就是要解析的HTTP消息。

最后要得到的效果如下，我们可以直接通过into方法直接解析字符串为 HttpRequest 类型

```Rust
let s = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.71.1\r\nAccept: */*\r\n\r\n");
// HttpRequest 是必写的
let req: HttpRequest = s.into();

```

需要注意的是，req的类型 `HttpRequest` 是必写的，s.into 会调用 HttpRequest 的From trait

除了 into trait 方法能够调用 HttpRequest::From 也可以显式调用 From trait

---

终止于 P4 05；40

