//! Viacep service: https://viacep.com.br/
//! 
//! the call to this service uses Hyper as its HTTP library

extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate serde;

use hyper::Client;
use hyper_tls::HttpsConnector;

use serde::{Serialize, Deserialize};

/// request function runs the API call to Viacep service
pub async fn request(cep : &str) -> Result<Address, hyper::Error>{
    // This is where we will setup our HTTP client requests.
    // Still inside `async fn main`...
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Parse an `http::Uri`...
    let uri =  format!("https://viacep.com.br/ws/{}/json/",cep).parse().unwrap();

    // Await the response...
    let resp = client.get(uri).await?;
    
    let data = hyper::body::to_bytes(resp).await?;

    let address = serde_json::from_slice::<Address>(&data).unwrap();

    return Ok(address)
}

/// Address struct used to deserialize the results from the viacep API
#[derive(Deserialize, Serialize, Debug)]
pub struct Address {
    #[serde(rename = "cep")]
    pub cep: String,
    #[serde(rename = "logradouro")]
    pub address: String,
    #[serde(rename = "complemento")]
    pub details: String,
    #[serde(rename = "bairro")]
    pub neighborhood: String,
    #[serde(rename = "uf")]
    pub state: String,
    #[serde(rename = "localidade")]
    pub city: String,
    #[serde(rename = "unidade")]
    pub unidade: String,
    #[serde(rename = "ibge")]
    pub ibge: String,
    #[serde(rename = "gia")]
    pub gia: String,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn valid_viacep() {
        let resaddr = super::request("70150903").await.unwrap();
        
        let addr = super::Address {
            cep: "70150-903".to_string(),
            address: "SPP".to_string(),
            details: "".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            city: "Brasília".to_string(),
            state: "DF".to_string(),
            unidade: "".to_string(),
            ibge: "5300108".to_string(),
            gia: "".to_string(),
        };

        assert_eq!(addr.cep, resaddr.cep);
        assert_eq!(addr.address, resaddr.address);
        assert_eq!(addr.details, resaddr.details);
        assert_eq!(addr.neighborhood, resaddr.neighborhood);
        assert_eq!(addr.city, resaddr.city);
        assert_eq!(addr.state, resaddr.state);
        assert_eq!(addr.unidade, resaddr.unidade);
        assert_eq!(addr.ibge, resaddr.ibge);
        assert_eq!(addr.gia, resaddr.gia);
    }
}
