mod http;
mod pool;
use std::net::{TcpListener, TcpStream};
use std::io::{Read,Write};
use std::fs;
use crate::http::{Request, HttpReqMethod};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::{Debug, Error};
use core::cmp;
use std::os::macos::raw::stat;
use percent_encoding::percent_decode;
use bytes::{BytesMut, BufMut};


fn main() {
    let listener  = TcpListener::bind("127.0.0.1:9988").unwrap();
    let th_pool: pool::ThreadPool = pool::ThreadPool::new(20);
    println!("[service] listening for connections on port {}",9988);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
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
        let f = Path::new(&p.path);
        if f.is_dir() {
            let contents =  directory(&p.path);
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                contents.len(),
                contents
            );
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        }else if f.is_file(){

            // 下载文件
            let fileinfo =   f.metadata().unwrap();
            let mut file = fs::File::open(&p.path).unwrap();
            let mut buf:Vec<u8> = vec![0; fileinfo.len() as usize];
            let n = file.read(&mut buf[..]).unwrap();
            let mut bufs = BytesMut::with_capacity(fileinfo.len() as usize);
            bufs.put(&buf[..]);

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-type: {}\r\nContent-Disposition: {}\r\n\r\n{:?}",
                fileinfo.len(),
                "application/octet-stream",
                format!("attachment; filename={}",f.file_name().unwrap().to_str().unwrap()),
                bufs
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

fn directory(pathinfo:&str)->String{
    let mut results = String::from("");
    let mut plist = pathinfo.split("/").collect::<Vec<&str>>();
    results.push_str("<style>a{text-decoration:none;}</style>");
    results.push_str("<meta http-equiv='Content-Type' content='text/html; charset=utf-8' />");
    results.push_str(&*generate_header(&mut plist));
    let dir = fs::read_dir(&pathinfo);
    match dir {
        Ok(dir) => {
            results.push_str("<table><tbody>");
            results.push_str("<tr>
            <th><a>Name</a></th>
            <th><a>Last modified</a></th>
            <th><a>Size</a></th>
            </tr>");
            results.push_str("<tr><td style='border-top:1px dashed #BBB;' colspan='5'></td></tr>");
            for mut f in dir {
                let p = f.unwrap();
                let metadata  =  p.metadata().unwrap();
                let mut size:String  = String::from("-");
                let mut filename:String  = String::from(p.file_name().to_str().unwrap());
                if metadata.is_file() {
                    size =  gen_file_size(metadata.len() as f64);
                }
                if metadata.is_dir() {
                    filename.push_str("/");
                }
                results.push_str(&*format!("<tr>
                    <td><a href='{}' style='display: block;' >{}<a></td>
                    <td><a  style='display: block;' >{}<a></td>
                    <td><a  style='display: block;' >{}<a></td>
                </tr>", p.path().to_str().unwrap(), filename, "file_modified",size ));
                results.push_str("</tr>");
            }
            results.push_str("</table></tbody>");
            results
        },
        Err(e)=> {
            println!("error info {}",e);
            results.push_str(&*generate_link("", &*e.to_string(),true));
            results
        }
    }
}
fn generate_link(url:&str , text:&str,block:bool)-> String {
    let mut dispaly = "block";
    if !block{
        dispaly = "inline";
    }
    format!("<a href='{}' style='display: {};' >{}<a>", url,dispaly,text, )
}
fn generate_header(plist:&mut Vec<&str>) -> String {
    let start  = "<div>";
    let end = "</div><hr/>";
    let mut  content = String::from("");
    content.push_str(&*generate_link("/", "[Root]",false));
    for item in 1..plist.len() {
        content.push_str(" / ");
        let url = &plist[0..item+1].join("/");
        let name  = &*plist[item].to_string();
        content.push_str(&*generate_link(url,name, false));
    }
    let header  = format!("{}{}{}",start,content,end);
    header
}
pub fn gen_file_size(num: f64) -> String {
    let negative = if num.is_sign_positive() { "" } else { "-" };
    let num = num.abs();
    let units = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    if num < 1_f64 {
        return format!("{}{} {}", negative, num, "B");
    }
    let delimiter = 1000_f64;
    let exponent = cmp::min((num.ln() / delimiter.ln()).floor() as i32, (units.len() - 1) as i32);
    let pretty_bytes = format!("{:.2}", num / delimiter.powi(exponent)).parse::<f64>().unwrap() * 1_f64;
    let unit = units[exponent as usize];
    format!("{}{} {}", negative, pretty_bytes, unit)
}