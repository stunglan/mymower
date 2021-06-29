mod authenticate;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let config = authenticate::ServerConfig::new().await;

    println!("resp {:}", config);

    println!("mowers {}",config.get_mowers().await);
}
