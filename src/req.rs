// use tokio::net::TcpStream;

use anyhow;

use std::collections::{self, HashMap};



pub fn parse_request(data: String) -> anyhow::Result<Request>{
   

    // let mut buf = vec![0u8; 1024];

    // socket.try_read(&mut buf)?;

    

    let mut parts = data.split_whitespace();


    let method: Method = parts
        .next()
        .ok_or(anyhow::anyhow!("No fucking method in request??? req:{}", data))?
        .try_into()?;

    let path: String = parts
        .next()
        .ok_or(anyhow::anyhow!("THERWS NO FUHCKING PATH IN THE REQUEST:{}", data))?
        .try_into()?;

    let http_version = parts
        .next()
        .ok_or(anyhow::anyhow!("THERS NO FUCKING HTTP VERSION req:{}", data))?;

    
    let mut parts = data.split("\r\n");
    parts.next(); //Throw away first part bc its already been parsed

    let mut headers: HashMap<String, String> = HashMap::new();

    loop {
        let (k, v): (String, String) = match parts.next(){
            Some(str) => {
                let mut str = str.split(':');
                (str.next().unwrap().to_string(), str.next().unwrap().to_string())
            },
            None => break,
        };

        headers.insert(k, v);
    }

    let req = Request::new(method, Some(path), http_version.to_string(), headers);
    Ok(req)
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Method{
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

impl TryFrom<&str> for Method{
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.trim();

        match value {
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "DELETE" => Ok(Method::DELETE),
            "CONNECT" => Ok(Method::CONNECT),
            "OPTIONS" => Ok(Method::OPTIONS),
            "TRACE" => Ok(Method::TRACE),
            "PATCH" => Ok(Method::PATCH),
            _ => Err(anyhow::anyhow!("method does not fucking match any fucking thing")),
        } 
    }
}

impl Method {
    pub fn to_string(&mut self) -> String{
        return format!("{:?}", self)
    }
}

pub struct Request{
    pub method: Method,
    pub path: Option<String>,
    pub http_version: String,
    pub headers: HashMap<String, String> 
}

impl Request{
    pub fn new(method: Method, path: Option<String>, http_version: String, headers: HashMap<String, String>) -> Self{
        return Request{
            method,
            path,
            http_version,
            headers
        }
    }

    pub fn to_string(&mut self) -> String{
        format!("{:?} {} {}", self.method, self.path.clone().unwrap_or("".to_string()), self.http_version)
    }
}