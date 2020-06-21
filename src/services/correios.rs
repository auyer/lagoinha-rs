extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_xml_rs;

use bytes::buf::BufExt;
use hyper::Client;
use hyper_tls::HttpsConnector;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

pub async fn request(cep: String) -> Result<Address, hyper::Error> {
    // This is where we will setup our HTTP client requests.
    // Still inside `async fn main`...
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    // Parse an `http::Uri`...
    let uri : hyper::Uri = hyper::Uri::from_str(
        "https://apps.correios.com.br/SigepMasterJPA/AtendeClienteService/AtendeCliente?wsdl",
    )
    .unwrap();
    // format!cep.as_str())
    // Await the response...
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
        cep.as_str()
    );
    let request = hyper::Request::builder()
        .method("POST")
        .header("content-type", "application/soap+xml;charset=utf-8")
        .header("cache-control", "no-cache")
        .uri(uri)
        .body(hyper::Body::from(payload))
        .unwrap();

    let resp = client.request(request).await?;
    // match resp {
    //     Ok(resp) => println!("Response: {}", resp.status),
    //     return(Err(e))
    // }
    let data = hyper::body::to_bytes(resp).await?;
    println!("{}", std::str::from_utf8(&data).unwrap());
    // std::io::BufReader::new(data)
    // let address = match serde_xml_rs::from_reader::<_,AddressResponse>(data.reader()){
    //     Ok(address)  => Ok(address.body),
    //     Err(e) => return Err(e),
    // };

    let correios_data: BodyTag = serde_xml_rs::from_reader(data.reader()).unwrap();
    return Ok(correios_data.body_tag.consult_tag.return_tag);
}

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

#[derive(Deserialize, Serialize, Debug)]
pub struct Address {
    #[serde(rename = "cep")]
    pub cep: String,
    #[serde(rename = "uf")]
    pub state: String,
    #[serde(rename = "cidade")]
    pub city: String,
    #[serde(rename = "bairro")]
    pub neighborhood: String,
    #[serde(rename = "end")]
    pub street: String,
}

// Example XML response
// <?xml version="1.0" encoding="UTF-8"?>
// <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
//    <soap:Body>
//       <ns2:consultaCEPResponse xmlns:ns2="http://cliente.bean.master.sigep.bsb.correios.com.br/">
//          <return>
//             <bairro>Zona Cívico-Administrativa</bairro>
//             <cep>70150903</cep>
//             <cidade>Brasília</cidade>
//             <complemento2 />
//             <end>SPP</end>
//             <uf>DF</uf>
//          </return>
//       </ns2:consultaCEPResponse>
//    </soap:Body>
// </soap:Envelope>

#[cfg(test)]
mod tests {
    // use viacep;
    #[tokio::test]
    async fn valid_correios() {
        let resaddr = super::request(String::from("70150903")).await.unwrap();

        let addr = super::Address {
            cep: "70150903".to_string(),
            state: "DF".to_string(),
            city: "Brasília".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            street: "SPP".to_string(),
        };

        assert_eq!(addr.cep, resaddr.cep);
        assert_eq!(addr.state, resaddr.state);
        assert_eq!(addr.neighborhood, resaddr.neighborhood);
        assert_eq!(addr.street, resaddr.street);
    }
}
