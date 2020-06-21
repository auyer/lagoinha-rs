extern crate curl;
extern crate serde_json;
extern crate serde;

use curl::easy::{Easy, List};
// use hyper_tls::HttpsConnector;

use serde::{Serialize, Deserialize};


pub async fn request(cep : String) -> Result<Address, hyper::Error>{
    let mut requester = Easy::new();
    let uri = format!("http://cep.la/{}",cep.as_str());
    requester.url(&uri).unwrap();
    let mut list = List::new();
    list.append("Accept: application/json").unwrap();
    requester.http_headers(list).unwrap();
    let mut buf = Vec::new();
    {
        let mut transfer = requester.transfer();
        transfer.write_function(|new_data| {
            buf.extend_from_slice(new_data);
            Ok(new_data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    println!("{}", std::str::from_utf8(&buf).unwrap());
    let address = serde_json::from_slice::<Address>(&buf).unwrap();
    return Ok(address)
}

#[derive(Serialize, Deserialize)]
pub struct Address {
    #[serde(rename = "cep")]
    cep: String,

    #[serde(rename = "uf")]
    localidade: String,

    #[serde(rename = "cidade")]
    cidade: String,

    #[serde(rename = "bairro")]
    bairro: String,

    #[serde(rename = "logradouro")]
    logradouro: String,

    #[serde(rename = "aux")]
    aux: String,
}

#[cfg(test)]
mod tests {
    // use viacep;
    #[tokio::test]
    async fn valid_cepla() {
        let resaddr = super::request(String::from("70150903")).await.unwrap();
        
        // let addr = super::Address {
        //     cep: "04569-901".to_string(),
        //     logradouro: "Rua Guaraiúva 553".to_string(),
        //     bairro: "Cidade Monções".to_string(),
        //     loca
        //     cidade: "São Paulo".to_string(),
        //     aux: "".to_string(),
        // };
        let addr = super::Address {
            cep: "70150903".to_string(),
            localidade: "DF".to_string(),
            cidade: "Brasília".to_string(),
            bairro: "Zona Cívico-Administrativa".to_string(),
            logradouro: "SPP".to_string(),
            aux: "Palácio da Alvorada (Residência Oficial do Presidente da República)".to_string(),
        };

        assert_eq!(addr.logradouro, resaddr.logradouro);
        assert_eq!(addr.aux, resaddr.aux);
        assert_eq!(addr.localidade, resaddr.localidade);
        assert_eq!(addr.bairro, resaddr.bairro);
        assert_eq!(addr.cidade, resaddr.cidade);
        assert_eq!(addr.cep, resaddr.cep);
    }
}
