use tungstenite::{connect_with_config, protocol::WebSocketConfig,Message};
use tungstenite::http::header::{AUTHORIZATION, HeaderValue};
use tungstenite::accept_with_config;
use std::net::TcpStream;
use url::Url;
use serde_json;



pub fn get_data( url_str: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {    // Connect to the WS server locally

    let url = Url::parse(url).unwrap();
    let (mut socket, _response) = connect(Url::parse(url_str).unwrap()).expect("Can't connect");    // Write a message containing "Hello, Test!" to the server
    socket.write_message(Message::Text("Hello, Test!".into())).unwrap();
    
    let stream = TcpStream::connect(url.socket_addrs(|| None)?[0])?;

    let mut config = WebSocketConfig::new();
    config.additional_headers = Some(vec![
        (AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", token))?),
    ].into_iter().collect());


    let socket = accept_with_config(url_str, stream, Some(config))?;
    // Loop forever, handling parsing each message
    loop {
        let msg = socket.read_message().expect("Error reading message");
        let msg = match msg {
            tungstenite::Message::Text(s) => { s }
            _ => { panic!() }
        };
        let parsed: serde_json::Value = serde_json::from_str(&msg).expect("Can't parse to JSON");
        println!("{:?}", parsed["result"]);
    }
}