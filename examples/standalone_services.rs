// examples/standalone_services.rs
extern crate lagoinha;

extern crate tokio;
#[tokio::main]
async fn main() {    
    let addr = lagoinha::services::viacep::request("70150903").await;
    println!("\nviacep");
    println!("{:#?}", addr);

    let addr = lagoinha::services::correios::request("70150903").await;
    println!("\ncorreios");
    println!("{:#?}", addr);

    let addr = lagoinha::services::cepla::request("70150903").await;
    println!("\ncepla");
    println!("{:#?}", addr);
}