mod authenticate;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let resp = authenticate::test1();

    println!("resp {:?}",resp.await)
    

}
