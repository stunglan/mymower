mod authenticate;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let resp = authenticate::ServerConfig::new().await;
    

    println!("resp {:}", resp)
}
