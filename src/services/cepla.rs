//! CepLá service: http://cep.la/
//!
//! This service has an out os [spec](https://tools.ietf.org/html/rfc2616#section-4.2) header implementation,
//! and does not comply with the [RFC2616](https://tools.ietf.org/html/rfc2616#section-4.2).
//! This causes an issue when using it with libraries, like Hyper, because they parse all headers to lower case.
//! To solve this issue, the title_case_headers(true) option was used.

use crate::error::Error;
use crate::error::Kind;
use crate::error::Source::Cepla;

use serde::{Deserialize, Serialize};

use isahc::{config::Configurable, ReadResponseExt, Request, RequestExt};

/// request function runs the API call to cepla service
pub async fn request(cep: &str) -> Result<Address, Error> {
    let uri = format!("http://cep.la/{}", cep);
    let req = Request::get(uri)
        .title_case_headers(true)
        .header("Accept", "application/json")
        .body(())
        .or(Err(Error {
            kind: Kind::UnexpectedLibraryError,
            source: Cepla,
        }))?;

    let mut response = req.send().or(Err(Error {
        kind: Kind::MissingBodyError,
        source: Cepla,
    }))?;

    match response.status().as_u16() {
        200..=299 => (),
        400..=499 => {
            return Err(Error {
                kind: Kind::ClientError {
                    code: response.status().as_u16(),
                },
                source: Cepla,
            });
        }
        500..=599 => {
            return Err(Error {
                kind: Kind::ServerError {
                    code: response.status().as_u16(),
                },
                source: Cepla,
            });
        }
        _ => {
            return Err(Error {
                kind: Kind::UnknownServerError {
                    code: response.status().as_u16(),
                },
                source: Cepla,
            });
        }
    }
    let body = response.body_mut();

    let address = serde_json::from_reader(body);
    match address {
        Ok(address) => return Ok(address),
        Err(e) => {
            let str_body = response.text();
            let str_body = match str_body {
                Ok(str_body) => str_body,
                Err(e) => "Failed to produce string body ".to_owned() + e.to_string().as_str(),
            };
            return Err(Error {
                kind: Kind::BodyParsingError {
                    error: e.to_string(),
                    body: str_body.to_string(),
                },
                source: Cepla,
            });
        }
    };
}

/// Address struct used to deserialize the results from the cepla API
#[derive(Serialize, Deserialize, Debug)]
pub struct Address {
    #[serde(rename = "cep", default = "String::new")]
    pub cep: String,
    #[serde(rename = "uf", default = "String::new")]
    pub state: String,
    #[serde(rename = "cidade", default = "String::new")]
    pub city: String,
    #[serde(rename = "bairro", default = "String::new")]
    pub neighborhood: String,
    #[serde(rename = "logradouro", default = "String::new")]
    pub address: String,
    #[serde(rename = "aux", default = "String::new")]
    pub details: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn valid_cepla() {
        let resaddr = async_std::task::block_on(super::request("70150903")).unwrap();

        let addr = super::Address {
            cep: "70150903".to_string(),
            state: "DF".to_string(),
            city: "Brasília".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            address: "SPP".to_string(),
            details: "Palácio da Alvorada (Residência Oficial do Presidente da República)"
                .to_string(),
        };

        assert_eq!(addr.address, resaddr.address);
        assert_eq!(addr.state, resaddr.state);
        assert_eq!(addr.neighborhood, resaddr.neighborhood);
        assert_eq!(addr.city, resaddr.city);
        assert_eq!(addr.cep, resaddr.cep);
        assert_eq!(addr.details, resaddr.details);
    }

    #[test]
    fn valid_cepla_with_dash() {
        let resaddr = async_std::task::block_on(super::request("70150-903")).unwrap();

        let addr = super::Address {
            cep: "70150903".to_string(),
            state: "DF".to_string(),
            city: "Brasília".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            address: "SPP".to_string(),
            details: "Palácio da Alvorada (Residência Oficial do Presidente da República)"
                .to_string(),
        };

        assert_eq!(addr.address, resaddr.address);
        assert_eq!(addr.state, resaddr.state);
        assert_eq!(addr.neighborhood, resaddr.neighborhood);
        assert_eq!(addr.city, resaddr.city);
        assert_eq!(addr.cep, resaddr.cep);
        assert_eq!(addr.details, resaddr.details);
    }

    use crate::error::Kind;
    use crate::error::Source;
    #[test]
    fn invalid_input_viacep() {
        let resaddr = async_std::task::block_on(super::request("123"));
        assert!(resaddr.is_err());
        resaddr
            .map_err(|err| {
                assert_eq!(err.source, Source::Cepla);
                assert_eq!(
                    std::mem::discriminant(&err.kind),
                    std::mem::discriminant(&Kind::BodyParsingError {
                        error: "".to_owned(),
                        body: "".to_owned(),
                    })
                );
            })
            .ok();
    }
}
