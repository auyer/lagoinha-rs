// examples/get_address.rs
extern crate lagoinha;
extern crate tokio;

#[tokio::main]
async fn main() {    
    let addr = lagoinha::get_address("04569901").await;
    println!("{:#?}", addr);
}