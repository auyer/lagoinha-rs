pub mod services;
use services::Address;

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

pub async fn get_address(cep: &str) -> Address{
    let (tx, mut rx) = mpsc::channel::<services::Address>(1);

    futures::select!{
        () = viacep_requet(cep, tx.clone()).fuse() => "viacep", 
        () = cepla_requet(cep, tx.clone()).fuse() => "cepla", 
        () = correios_requet(cep, tx.clone()).fuse() => "correios",
        default => unreachable!()
    };

    let read = rx.try_next().unwrap().unwrap();
    read
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

        let recv_addr = super::get_address("70150903").await;
        assert_eq!(addr.city, recv_addr.city);
        assert_eq!(addr.state, recv_addr.state);
        assert_eq!(addr.neighborhood, recv_addr.neighborhood);
        // the other fields, like cep can come with different formating
    }

}