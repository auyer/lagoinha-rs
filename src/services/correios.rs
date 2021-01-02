//! Correios service: http://www.buscacep.correios.com.br/sistemas/buscacep/BuscaCepEndereco.cfm

use isahc::{ReadResponseExt, Request, RequestExt};

use crate::error::Error;
use crate::error::Kind;
use crate::error::Source::Correios;

use serde::{Deserialize, Serialize};

/// request function runs the API call to correios service
pub async fn request(cep: &str) -> Result<Address, Error> {
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

    let req = Request::post(
        "https://apps.correios.com.br/SigepMasterJPA/AtendeClienteService/AtendeCliente?wsdl",
    )
    .header("content-type", "application/soap+xml;charset=utf-8")
    .header("cache-control", "no-cache")
    .body(payload)
    .or(Err(Error {
        kind: Kind::UnexpectedLibraryError,
        source: Correios,
    }))?;

    let mut response = req.send().or(Err(Error {
        kind: Kind::MissingBodyError,
        source: Correios,
    }))?;

    match response.status().as_u16() {
        200..=299 => (),
        400..=499 => {
            return Err(Error {
                kind: Kind::ClientError {
                    code: response.status().as_u16(),
                },
                source: Correios,
            });
        }
        500..=599 => {
            return Err(Error {
                kind: Kind::ServerError {
                    code: response.status().as_u16(),
                },
                source: Correios,
            });
        }
        _ => {
            return Err(Error {
                kind: Kind::UnknownServerError {
                    code: response.status().as_u16(),
                },
                source: Correios,
            });
        }
    }

    let body = response.body_mut();

    let correios_data: Result<BodyTag, serde_xml_rs::Error> = serde_xml_rs::from_reader(body);
    match correios_data {
        Ok(correios_data) => {
            return Ok(correios_data.body_tag.consult_tag.return_tag);
        }
        Err(e) => {
            let str_body = response.text();
            let str_body = match str_body {
                Ok(str_body) => str_body,
                Err(_) => "Failed to produce string body ".to_owned() + e.to_string().as_str(),
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
// only the Address struct is useful.
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
    #[test]
    fn valid_correios() {
        let resaddr = async_std::task::block_on(super::request("70150903")).unwrap();

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

    #[test]
    fn valid_correios_with_dash() {
        let resaddr = async_std::task::block_on(super::request("70150-903")).unwrap();

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
    #[test]
    fn invalid_input_correios() {
        let resaddr = async_std::task::block_on(super::request("123"));
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
