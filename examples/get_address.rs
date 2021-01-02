// examples/get_address.rs
//!Run `run --example get_address yourcep` to run this example
use lagoinha;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cep: &str = "20940040".as_ref();
    if args.len() >= 2 {
        cep = &args[1][..];
    }
    let addr = async_std::task::block_on(lagoinha::get_address(cep, None));
    println!("{:#?}", addr);
}
