// examples/get_address.rs
extern crate lagoinha;
extern crate tokio;

#[tokio::main]
async fn main() { 
    let addr = lagoinha::get_address("70150903").await;
    println!("{:#?}", addr);
}