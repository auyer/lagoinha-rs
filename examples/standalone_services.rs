// examples/standalone_services.rs
//!Run `run --example standalone_services yourcep` to run this example
use lagoinha;
// optional trait for standard type conversion
use lagoinha::services::Addressable;

use std::env;
use tokio;
#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cep: &str = "20940040".as_ref();
    print!("{}", args.len());
    if args.len() >= 2 {
        cep = &args[1][..];
    }
    let addr = lagoinha::services::viacep::request(cep).await;
    println!("\nviacep");
    println!("{:#?}", addr);
    // optinal to_address from Addressable trait converts specific address to general address
    println!("\n--converted:");
    println!("{:#?}", addr.unwrap().to_address());

    let addr = lagoinha::services::correios::request(cep).await;
    println!("\ncorreios");
    println!("{:#?}", addr);
    // optinal to_address from Addressable trait converts specific address to general address
    println!("\n--converted:");
    println!("{:#?}", addr.unwrap().to_address());

    let addr = lagoinha::services::cepla::request(cep).await;
    println!("\ncepla");
    println!("{:#?}", addr);
    // optinal to_address from Addressable trait converts specific address to general address
    println!("\n--converted:");
    println!("{:#?}", addr.unwrap().to_address());
}
