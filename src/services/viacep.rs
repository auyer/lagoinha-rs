extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate serde;

use hyper::Client;
use hyper_tls::HttpsConnector;

use serde::{Serialize, Deserialize};


pub async fn request(cep : String) -> Result<Address, hyper::Error>{
    // This is where we will setup our HTTP client requests.
    // Still inside `async fn main`...
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    // let client = Client::new();

    // Parse an `http::Uri`...
    let uri =  format!("https://viacep.com.br/ws/{}/json/",cep.as_str()).parse().unwrap();

    // Await the response...
    let resp = client.get(uri).await?;

    println!("Response: {}", resp.status());
    
    let data = hyper::body::to_bytes(resp).await?;

    println!("{}", std::str::from_utf8(&data).unwrap());

    let address = serde_json::from_slice::<Address>(&data).unwrap();

    return Ok(address)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Address {
    pub cep: String,
    pub logradouro: String,
    pub complemento: String,
    pub bairro: String,
    pub uf: String,
    pub localidade: String,
    pub unidade: String,
    pub ibge: String,
    pub gia: String,
}

#[cfg(test)]
mod tests {
    // use viacep;
    #[tokio::test]
    async fn valid_viacep() {
        let resaddr = super::request(String::from("70150903")).await.unwrap();
        
        let addr = super::Address {
            cep: "70150-903".to_string(),
            logradouro: "SPP".to_string(),
            complemento: "".to_string(),
            bairro: "Zona Cívico-Administrativa".to_string(),
            localidade: "Brasília".to_string(),
            uf: "DF".to_string(),
            unidade: "".to_string(),
            ibge: "5300108".to_string(),
            gia: "".to_string(),
        };

        assert_eq!(addr.logradouro, resaddr.logradouro);
        assert_eq!(addr.complemento, resaddr.complemento);
        assert_eq!(addr.bairro, resaddr.bairro);
        assert_eq!(addr.unidade, resaddr.unidade);
        assert_eq!(addr.ibge, resaddr.ibge);
        assert_eq!(addr.gia, resaddr.gia);
    }
}
