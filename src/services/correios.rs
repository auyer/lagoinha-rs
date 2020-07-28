//! Correios service: http://www.buscacep.correios.com.br/sistemas/buscacep/BuscaCepEndereco.cfm
//!
//! the call to this service uses Hyper as its HTTP library

extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_xml_rs;

use crate::error::Error;
use crate::error::Kind;
use crate::error::Source::Correios;

use bytes::buf::BufExt;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// request function runs the API call to correios service
pub async fn request(cep: &str) -> Result<Address, Error> {
    // This is where we will setup our HTTP client requests.
    // Still inside `async fn main`...
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    // Parse an `http::Uri`...
    let uri = hyper::Uri::from_str(
        "https://apps.correios.com.br/SigepMasterJPA/AtendeClienteService/AtendeCliente?wsdl",
    );

    let uri = match uri {
        Ok(uri) => uri,
        Err(_) => {
            return Err(Error {
                kind: Kind::UnexpectedLibraryError,
                source: Correios,
            })
        }
    };

    let payload = format!(
        r#"
    <soapenv:Envelope xmlns:soapenv="http://schemas.xmlsoap.org/soap/envelope/" xmlns:cli="http://cliente.bean.master.sigep.bsb.correios.com.br/">
        <soapenv:Header/>
        <soapenv:Body>
            <cli:consultaCEP>
                <cep>{}</cep>
            </cli:consultaCEP>
        </soapenv:Body>
    </soapenv:Envelope>
    "#,
        cep
    );
    let request = hyper::Request::builder()
        .method("POST")
        .header("content-type", "application/soap+xml;charset=utf-8")
        .header("cache-control", "no-cache")
        .uri(uri)
        .body(hyper::Body::from(payload));
    let request = match request {
        Ok(request) => request,
        Err(_) => {
            return Err(Error {
                kind: Kind::UnexpectedLibraryError,
                source: Correios,
            })
        }
    };

    let resp = client.request(request).await;
    let resp = match resp {
        Err(_) => {
            return Err(Error {
                kind: Kind::UnexpectedLibraryError,
                source: Correios,
            })
        }
        Ok(resp) => {
            let code = resp.status().as_u16();
            match code {
                200..=299 => resp,
                400..=499 => {
                    return Err(Error {
                        kind: Kind::ClientError { code: code as u16 },
                        source: Correios,
                    });
                }
                500..=599 => {
                    return Err(Error {
                        kind: Kind::ServerError { code: code as u16 },
                        source: Correios,
                    });
                }
                _ => {
                    return Err(Error {
                        kind: Kind::UnknownServerError { code: code as u16 },
                        source: Correios,
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
                source: Correios,
            });
        }
    };
    // let datab = data.clone();

    // println!("{}", std::str::from_utf8(&data).unwrap());

    let correios_data: Result<BodyTag, serde_xml_rs::Error> =
        serde_xml_rs::from_reader(data.clone().reader()); // this clone prevents value droping to produce str_body error
    match correios_data {
        Ok(correios_data) => {
            return Ok(correios_data.body_tag.consult_tag.return_tag);
        }
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
                source: Correios,
            });
        }
    };
}

// these structs are used to define the entire path to the XML. There must be a better way to do this...
// only the Address struct is usefull.
#[derive(Deserialize, Serialize, Debug)]
struct BodyTag {
    #[serde(rename = "Body")]
    pub body_tag: ConsultTag,
}

#[derive(Deserialize, Serialize, Debug)]
struct ConsultTag {
    #[serde(rename = "consultaCEPResponse")]
    pub consult_tag: ReturnTag,
}

#[derive(Deserialize, Serialize, Debug)]
struct ReturnTag {
    #[serde(rename = "return")]
    pub return_tag: Address,
}

/// Address struct used to deserialize the results from the correios API
#[derive(Deserialize, Serialize, Debug)]
pub struct Address {
    #[serde(rename = "cep", default = "String::new")]
    pub cep: String,
    #[serde(rename = "uf", default = "String::new")]
    pub state: String,
    #[serde(rename = "cidade", default = "String::new")]
    pub city: String,
    #[serde(rename = "bairro", default = "String::new")]
    pub neighborhood: String,
    #[serde(rename = "end", default = "String::new")]
    pub address: String,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn valid_correios() {
        let resaddr = super::request("70150903").await.unwrap();

        let addr = super::Address {
            cep: "70150903".to_string(),
            state: "DF".to_string(),
            city: "Brasília".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            address: "SPP".to_string(),
        };

        assert_eq!(addr.cep, resaddr.cep);
        assert_eq!(addr.state, resaddr.state);
        assert_eq!(addr.city, resaddr.city);
        assert_eq!(addr.neighborhood, resaddr.neighborhood);
        assert_eq!(addr.address, resaddr.address);
    }

    use crate::error::Error;
    use crate::error::Kind;
    use crate::error::Source;
    #[tokio::test]
    async fn invalid_input_correios() {
        let resaddr = super::request("123").await;
        assert!(resaddr.is_err());
        resaddr
            .map_err(|err| {
                assert_eq!(
                    err,
                    Error {
                        source: Source::Correios,
                        kind: Kind::ServerError { code: 500 }
                    }
                )
            })
            .ok();
    }
}
