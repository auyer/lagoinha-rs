
#![crate_name = "lagoinha"]
//! Lagoinha is a library that retrieve Addresses from the Brazilian Postal Code (CEP) using multiple APIs asynchronously, and returns the result from the first one to respond.
//! It uses async/.await and the Futures library for its asyncronous features, and can be used with most runtimes.
//! 
//! # Services
//! 
//! Currenlty the services used are : correios, viacep and cepla
//! It is expected to support adding a custom service to the pool in the future, and the ability to disable the default ones.
//! 
//! While the default http library is Hyper, the CepLá service has an issue with its header implementation, and so the curl library was used. More information in the docs for this service.
//! 
//! # Example
//! ```
//!extern crate lagoinha;
//!extern crate tokio;
//!
//!#[tokio::main]
//!async fn main() {    
//!    let addr = lagoinha::get_address("CEP_GOES_HERE").await;
//!    println!("{:#?}", addr);
//!}
//!```
//!

pub mod services;
use services::Address;

use std::error::Error;
use futures::channel::mpsc;
use futures::{
    future::FutureExt,
    sink::SinkExt,
};

async fn viacep_requet(cep : &str, mut  tx: mpsc::Sender<Address>){
    return tx.send(services::viacep::request(cep).await.unwrap().to_address()).await.unwrap()
}

async fn cepla_requet(cep : &str, mut  tx: mpsc::Sender<Address>){
    return tx.send(services::cepla::request(cep).await.unwrap().to_address()).await.unwrap()
}

async fn correios_requet(cep : &str, mut  tx: mpsc::Sender<Address>){
    return tx.send(services::correios::request(cep).await.unwrap().to_address()).await.unwrap()
}

pub async fn get_address(cep: &str) -> Result<Address, Box<dyn std::error::Error>> {
    let (tx, mut rx) = mpsc::channel::<services::Address>(1);

    futures::select!{
        () = viacep_requet(cep, tx.clone()).fuse() => "viacep", 
        () = cepla_requet(cep, tx.clone()).fuse() => "cepla", 
        () = correios_requet(cep, tx.clone()).fuse() => "correios",
        default => unreachable!()
    };

    let read = rx.try_next().unwrap().unwrap();
    Ok(read)
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_channels() {

        let addr = super::services::Address {
            cep: "70150903".to_string(),
            address: "SPP".to_string(),
            details: "".to_string(),
            neighborhood: "Zona Cívico-Administrativa".to_string(),
            city: "Brasília".to_string(),
            state: "DF".to_string(),
        };

        let recv_addr = super::get_address("70150903").await.unwrap();
        assert_eq!(addr.city, recv_addr.city);
        assert_eq!(addr.state, recv_addr.state);
        assert_eq!(addr.neighborhood, recv_addr.neighborhood);
        // the other fields, like cep can come with different formating
    }

}