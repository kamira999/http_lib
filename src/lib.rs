pub mod req;
pub mod resp;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }

    fn test_req(){
        let method = req::Method::try_from("GET");
        // let req = req::Request::new();
    }
}
