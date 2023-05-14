extern crate dotenv;
use dotenv::dotenv;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fmt;
use reqwest::header;
use reqwest::Client;
use reqwest::{Response, Error};


#[derive(Debug)]
pub struct ServerConfig {
  application_key: String,
  auth_uri: String,
  husqvarna_uri: String,
  bearer: String,
  login: bool,
  secret: String,
}

fn get_env(key: &str) -> String {
  match env::var(key) {
    Ok(val) => val,
    Err(e) => {
      panic!("couldn't interpret {} got {}", key, e)
    }
  }
}

fn set_from_env() -> ServerConfig {
  let application_key = get_env("APPLICATION_KEY");
  let auth_uri = get_env("AUTH_URI");
  let husqvarna_uri = get_env("HUSQVARNA_URI");
  let secret = get_env("APPLICATION_SECRET");

  ServerConfig {
    application_key: String::from(application_key),
    auth_uri: String::from(auth_uri),
    husqvarna_uri: String::from(husqvarna_uri),
    bearer: String::from(""),
    login: false,
    secret: String::from(secret),
  }
}

impl fmt::Display for ServerConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "ServerConfig application_key: {}, auth_uri: {}, app_uri: {}, bearer: {}, login {}, secret: {})",
      self.application_key,
      self.auth_uri,
      self.husqvarna_uri,
      self.bearer,
      self.login,
      self.secret,
    )
  }
}

impl ServerConfig {
  async fn login(&self) -> Value {
    let mut map = HashMap::new();
    map.insert("grant_type", "client_credentials");
    map.insert("client_id", &self.application_key);
    map.insert("client_secret", &self.secret);


    let client = reqwest::Client::new();
    let resp = client.post(&self.auth_uri).form(&map).send().await;
    match resp {
      Ok(resp) => {
        //let result = resp.text().await;
        let json_response = resp.json::<serde_json::Value>().await;
        match json_response {
          Ok(value) => {
            return value;
          }
          Err(err) => {
            panic!(
              "Error in extracting text from what returned from {:}\nErr; {:}",
              self.auth_uri, err
            )
          }
        }
      }
      Err(err) => {
        panic!(
          "Error in extracting text from  {:}\nErr; {:}",
          self.auth_uri, err
        )
      }
    }
  }

  pub async fn get_mowers(&self) -> Result<Response, Error> {
    let client = reqwest::Client::new();


    let resp = client.get(&self.husqvarna_uri)
    //let resp = client.get("https://postman-echo.com/get")
    .header("Authorization",format!("{} {}","Bearer",&self.bearer))
    .header("Authorization-Provider","husqvarna")
    .header("X-Api-Key",&self.application_key)
    .send().await?;

    Ok(resp)
    

  }


  pub async fn new() -> ServerConfig {
    dotenv().ok();

    let mut config = set_from_env();

    let resp = config.login();
    let resp = resp.await;

    //println!("Hello, res {:?}", &resp);
    // resp.whatisthis(); // no method named `whatisthis` found for enum `Value` in the current scope
    //let v: Value = serde_json::from_str(&resp).unwrap();
    //let json_response = resp.json::<serde_json::Value>().unwrap();

    //println!("v {:?}",v.as_str());
    //println!("json_response {:?}",json_response.as_str());
    
    config.bearer = resp["access_token"].as_str().unwrap().to_string();
    //println!("Hello, bearer {:?}", &config.bearer);
    config.login = true;
    config
  }
}
