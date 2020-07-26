// examples/get_address.rs
//!Run `run --example get_address yourcep` to run this example
extern crate lagoinha;
extern crate tokio;

use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cep: &str = "20940040".as_ref();
    print!("{}", args.len());
    if args.len() >= 2 {
        cep = &args[1][..];
    }
    let addr = lagoinha::get_address(cep).await;
    println!("{:#?}", addr);
}
