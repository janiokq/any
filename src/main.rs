mod http;
mod threadPool;

use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::fs;
use crate::http::{Request, HttpReqMethod};
use crate::threadPool::ThreadPool;

fn main() {
    let listener  = TcpListener::bind("127.0.0.1:9988").unwrap();
    let th_pool: ThreadPool = ThreadPool::new(20);
    println!("[service] listening for connections on port {}",9988);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        /// 使用线程池处理任务
        th_pool.execute(||{
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0; 2048];
    stream.read(&mut buffer).unwrap();
    let parameter = String::from_utf8_lossy(&buffer[..]);
    let p:Request =   Request::new(&parameter.parse().unwrap()).unwrap();
    if p.method == HttpReqMethod::GET  {
        let get = b"GET / HTTP/1.1\r\n";
        if buffer.starts_with(get) {
            let contents = fs::read_to_string("/Users/z01/dev/rust/any/src/hello.html").unwrap();
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }
    }else{
        let status_line = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        let response = format!("{}{}", status_line, "404");
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}


