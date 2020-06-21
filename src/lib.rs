
// use super::services;
mod services;
pub use services::viacep;
pub async fn get_address(cep : &str) -> Result<viacep::Address, hyper::Error>{
    let addr = viacep::request(String::from(cep)).await?;
    return Ok(addr)
    // return Ok(
}


#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_works() {
        let resaddr = super::get_address("04569901").await.unwrap();
        
        let addr = super::viacep::Address {
            cep: "04569-901".to_string(),
            uf: "DF".to_string(),
            logradouro: "Rua Guaraiúva 553".to_string(),
            complemento: "".to_string(),
            bairro: "Cidade Monções".to_string(),
            localidade: "São Paulo".to_string(),
            unidade: "".to_string(),
            ibge: "3550308".to_string(),
            gia: "1004".to_string(),
        };

        // println!("{:#?}", addr);
        assert_eq!(addr.logradouro, resaddr.logradouro);
        assert_eq!(addr.complemento, resaddr.complemento);
        assert_eq!(addr.bairro, resaddr.bairro);
        assert_eq!(addr.unidade, resaddr.unidade);
        assert_eq!(addr.ibge, resaddr.ibge);
        assert_eq!(addr.gia, resaddr.gia);
        // assert_eq!("123", super::get_address("123").await.unwrap());
    }
}
