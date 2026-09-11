pub mod req;
pub mod resp;

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    fn test_req() -> anyhow::Result<()>{

        use std::collections::HashMap;

        let mut method = req::Method::try_from("GET")?;
        
        assert_eq!(format!("{:?}", method), "GET");
        assert_eq!("GET", method.to_string());

        let http_version = "HTTP/1.1";

        let path = "/";

        let mut headers: HashMap<String, String> = HashMap::new();
        headers.insert("Host:".to_string(), "google.com".to_string());
        headers.insert("Accept:".to_string(), "*/*".to_string());

        let mut request = req::Request::new(method, Some(path.to_string()), http_version.to_string(), headers);
        let request = request.to_string();
        if request == "GET / HTTP/1.1\r\nHost: google.com\nAccept: */*\r\n".to_string() || request == "GET / HTTP/1.1\r\nAccept: */*\nHost: google.com\r\n".to_string(){
            return Ok(())
        }

        Err(anyhow::Error::msg("request does not match:\n
            bad:{request} \n\
            good: GET / HTTP/1.1\r\nHost: google.com\nAccept: */*\r\n \n
            good: GET / HTTP/1.1\r\nAccept: */*\nHost: google.com\r\n"
        ))
    }

    #[test]
    fn test_resp() -> anyhow::Result<()> {

        use std::net::TcpStream;

        let mut req = req::Request::default();

        let mut stream = TcpStream::connect("google.com:80")?;

        stream.write_all(req.to_string().as_bytes())?;

        stream.flush()?;

        let mut buf = Vec::new();

        stream.read_to_end(&mut buf)?;

        let read = String::from_utf8(buf)?;

        // println!("{}", read);

        let read = resp::Response::parse(read)?;

        println!("{:#?}", read);

        Ok(())
    }
}
