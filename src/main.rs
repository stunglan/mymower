mod authenticate;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let resp = authenticate::ServerConfig::new();

    println!("resp {:}", resp.await)
}
