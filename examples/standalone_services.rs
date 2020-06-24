// examples/standalone_services.rs
extern crate lagoinha_rs;

extern crate tokio;
#[tokio::main]
async fn main() {    
    let addr = lagoinha_rs::services::viacep::request("04569901").await;
    println!("\nviacep");
    println!("{:#?}", addr);

    let addr = lagoinha_rs::services::correios::request("04569901").await;
    println!("\ncorreios");
    println!("{:#?}", addr);

    let addr = lagoinha_rs::services::cepla::request("04569901").await;
    println!("\ncepla");
    println!("{:#?}", addr);
}