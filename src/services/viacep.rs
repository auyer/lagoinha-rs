//! Viacep service: https://viacep.com.br/
//!
//! the call to this service uses Hyper as its HTTP library

extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;

use crate::error::Error;
use crate::error::Kind;
use crate::error::Source::Viacep;

use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::{Deserialize, Serialize};

/// request function runs the API call to Viacep service
pub async fn request(cep: &str) -> Result<Address, Error> {
    // This is where we will setup our HTTP client requests.
    // Still inside `async fn main`...
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Parse an `http::Uri`...
    let uri = format!("https://viacep.com.br/ws/{}/json/", cep).parse::<hyper::Uri>();

    let uri = match uri {
        Ok(uri) => uri,
        Err(_) => {
            return Err(Error {
                kind: Kind::UnexpectedLibraryError,
                source: Viacep,
            })
        }
    };
    // Await the response...
    let resp = client.get(uri).await;
    let resp = match resp {
        Err(_) => {
            return Err(Error {
                kind: Kind::UnexpectedLibraryError,
                source: Viacep,
            })
        }
        Ok(resp) => {
            let code = resp.status().as_u16();
            match code {
                200..=299 => resp,
                400..=499 => {
                    return Err(Error {
                        kind: Kind::ClientError { code: code as u16 },
                        source: Viacep,
                    });
                }
                500..=599 => {
                    return Err(Error {
                        kind: Kind::ServerError { code: code as u16 },
                        source: Viacep,
                    });
                }
                _ => {
                    return Err(Error {
                        kind: Kind::UnknownServerError { code: code as u16 },
                        source: Viacep,
                    });
                }
            }
        }
    };
    let data = hyper::body::to_bytes(resp).await;
    let data = match data {
        Ok(data) => data,
        Err(_) => {
            return Err(Error {
                kind: Kind::MissingBodyError,
                source: Viacep,
            });
        }
    };

    let address = serde_json::from_slice::<Address>(&data);
    match address {
        Ok(address) => return Ok(address),
        Err(e) => {
            let str_body = std::str::from_utf8(&data);
            let str_body = match str_body {
                Ok(str_body) => str_body,
                Err(_) => "Failed to produce string body ", //+  e.to_string().as_str()},
            };
            return Err(Error {
                kind: Kind::BodyParsingError {
                    error: e.to_string(),
                    body: str_body.to_string(),
                },
                source: Viacep,
            });
        }
    };
}

/// Address struct used to deserialize the results from the viacep API
#[derive(Deserialize, Serialize, Debug)]
pub struct Address {
    #[serde(rename = "cep", default = "String::new")]
    pub cep: String,
    #[serde(rename = "logradouro", default = "String::new")]
    pub address: String,
    #[serde(rename = "complemento", default = "String::new")]
    pub details: String,
    #[serde(rename = "bairro", default = "String::new")]
    pub neighborhood: String,
    #[serde(rename = "uf", default = "String::new")]
    pub state: String,
    #[serde(rename = "localidade", default = "String::new")]
    pub city: String,
    #[serde(rename = "unidade", default = "String::new")]
    pub unidade: String,
    #[serde(rename = "ibge", default = "String::new")]
    pub ibge: String,
    #[serde(rename = "gia", default = "String::new")]
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

    #[tokio::test]
    async fn valid_viacep_with_dash() {
        let resaddr = super::request("70150-903").await.unwrap();

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

    use crate::error::Error;
    use crate::error::Kind;
    use crate::error::Source;
    #[tokio::test]
    async fn invalid_input_viacep() {
        let resaddr = super::request("123").await;
        assert!(resaddr.is_err());
        resaddr
            .map_err(|err| {
                assert_eq!(
                    err,
                    Error {
                        source: Source::Viacep,
                        kind: Kind::ClientError { code: 400 }
                    }
                )
            })
            .ok();
    }
}
