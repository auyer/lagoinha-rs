// examples/standalone_services.rs
//!Run `run --example standalone_services yourcep` to run this example
extern crate lagoinha;

use std::env;

extern crate tokio;
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

    let addr = lagoinha::services::correios::request(cep).await;
    println!("\ncorreios");
    println!("{:#?}", addr);

    let addr = lagoinha::services::cepla::request(cep).await;
    println!("\ncepla");
    println!("{:#?}", addr);
}
