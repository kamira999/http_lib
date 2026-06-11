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

    fn test_req() -> anyhow::Result<()>{
        let method = req::Method::try_from("GET")?;
        // println!("{:?}");
        assert_eq!(format!("{:?}", method), "GET");
        Ok(())
    }
}
