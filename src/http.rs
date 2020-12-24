use percent_encoding::percent_decode;

pub enum HttpReqMethod {
    GET,
    POST,
    OPTIONS,
    PUT,
    PATCH,
    DELETE,
    TRACE,
    CONNECT,
    UNKNOWN
}
impl HttpReqMethod {
    fn new(parameter:&String) -> HttpReqMethod {
        match parameter.as_ref() {
            "GET"=> HttpReqMethod::GET,
            "POST"=> HttpReqMethod::POST,
            "OPTIONS"=> HttpReqMethod::OPTIONS,
            "PUT"=> HttpReqMethod::PUT,
            "PATCH"=> HttpReqMethod::PATCH,
            "DELETE"=> HttpReqMethod::DELETE,
            "TRACE"=> HttpReqMethod::TRACE,
            "CONNECT"=> HttpReqMethod::CONNECT,
            _=>HttpReqMethod::UNKNOWN
        }
    }
}
impl  PartialEq for HttpReqMethod {
    fn eq(&self, other: &Self) -> bool{
        match (self,other) {
            (HttpReqMethod::GET,HttpReqMethod::GET)=>true,
            (HttpReqMethod::POST,HttpReqMethod::POST)=>true,
            (HttpReqMethod::OPTIONS,HttpReqMethod::OPTIONS)=>true,
            (HttpReqMethod::PUT,HttpReqMethod::PUT)=>true,
            (HttpReqMethod::PATCH,HttpReqMethod::PATCH)=>true,
            (HttpReqMethod::DELETE,HttpReqMethod::DELETE)=>true,
            (HttpReqMethod::TRACE,HttpReqMethod::TRACE)=>true,
            (HttpReqMethod::CONNECT,HttpReqMethod::CONNECT)=>true,
            (HttpReqMethod::UNKNOWN,HttpReqMethod::UNKNOWN)=>true,
            _=>false
        }
    }
}



pub struct Request {
   pub method:HttpReqMethod,
   pub path:String,
   pub data:String,
}
impl Request {
    pub fn new(parameter:&String) -> Option<Request> {
        let plist  =   parameter.split(" ").collect::<Vec<&str>>();
        if plist.len() >= 1 {
            return  Some(Request{
                method:HttpReqMethod::new(&plist[0].to_string()),
                path: percent_decode(plist[1].as_bytes()).decode_utf8().unwrap().parse().unwrap(),
                data:String::from(""),
            })
        }
        None
    }
}