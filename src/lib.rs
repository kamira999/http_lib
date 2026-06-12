pub mod req;
pub mod resp;

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    fn test_req() -> anyhow::Result<()>{
        let mut method = req::Method::try_from("GET")?;
        
        assert_eq!(format!("{:?}", method), "GET");
        assert_eq!("GET", method.to_string());

        let http_version = "HTTP/1.1";

        let path = "/";

        let mut request = req::Request::new(method, Some(path.to_string()), http_version.to_string(), HashMap::new());
        assert_eq!("GET / HTTP/1.1".to_string(), request.to_string());
        Ok(())
    }
}
