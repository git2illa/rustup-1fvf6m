use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
   let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

   for stream in listener.incoming() {
       let stream = stream.unwrap();
       handle_connection(stream);
   }

}
// 定义一个处理连接的函数
fn handle_connection(mut stream: TcpStream){
    // 定义一个缓冲区，大小为512
    let mut buffer = [0; 512];
    // 从流中读取数据到缓冲区
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();
    let response = format!(
        "HTTP/1.1 200 OK\r\n\r\n{}",
        contents,
    );
    // 将响应写入流
    stream.write(response.as_bytes()).unwrap();
   // Flush the stream and unwrap any errors that may occur
    stream.flush().unwrap();
}
