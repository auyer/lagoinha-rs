// examples/get_address.rs
extern crate lagoinha_rs;

extern crate tokio;
#[tokio::main]
async fn main() {    
    let addr = lagoinha_rs::get_address("04569901").await;
    println!("{:#?}", addr);
}