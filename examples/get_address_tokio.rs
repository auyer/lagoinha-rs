// examples/get_address.rs
//!Run `run --example get_address_tokio yourcep` to run this example
use lagoinha;

use std::env;
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cep: &str = "20940040".as_ref();
    if args.len() >= 2 {
        cep = &args[1][..];
    }
    let addr = lagoinha::get_address(cep, None).await;
    println!("{:#?}", addr);
}
