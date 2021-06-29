use dotenv::dotenv;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fmt;
use reqwest::header;

#[derive(Debug)]
pub struct ServerConfig {
  user_name: String,
  user_password: String,
  application_key: String,
  auth_uri: String,
  husqvarna_uri: String,
  bearer: String,
  login: bool,
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
  let user_name = get_env("USER_NAME");
  let user_password = get_env("USER_PASSWORD");
  let application_key = get_env("APPLICATION_KEY");
  let auth_uri = get_env("AUTH_URI");
  let husqvarna_uri = get_env("HUSQVARNA_URI");

  ServerConfig {
    user_name: String::from(user_name),
    user_password: String::from(user_password),
    application_key: String::from(application_key),
    auth_uri: String::from(auth_uri),
    husqvarna_uri: String::from(husqvarna_uri),
    bearer: String::from(""),
    login: false,
  }
}

impl fmt::Display for ServerConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "(user_name: {},  application_key: {}, auth_uri: {}, app_uri: {}, bearer: {}, login {})",
      self.user_name,
      self.application_key,
      self.auth_uri,
      self.husqvarna_uri,
      self.bearer,
      self.login
    )
  }
}

impl ServerConfig {
  async fn login(&self) -> String {
    let mut map = HashMap::new();
    map.insert("grant_type", "password");
    map.insert("client_id", &self.application_key);
    map.insert("username", &self.user_name);
    map.insert("password", &self.user_password);

    let client = reqwest::Client::new();
    let resp = client.post(&self.auth_uri).form(&map).send().await;
    match resp {
      Ok(resp) => {
        let result = resp.text().await;
        match result {
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

  pub async fn get_mowers(&self) -> String{
    let client = reqwest::Client::new();


    let resp = client.get(&self.husqvarna_uri)
    //let resp = client.get("https://postman-echo.com/get")
    .header("Authorization",format!("{} {}","Bearer",&self.bearer))
    .header("Authorization-Provider","husqvarna")
    .header("X-Api-Key",&self.application_key)
    .send().await;

    let mowers = resp.unwrap().text().await.unwrap();
    println!("\n\n\nmovers resp {:?}",mowers);
    mowers

  }

  pub async fn new() -> ServerConfig {
    dotenv().ok();

    let mut config = set_from_env();

    let resp = config.login();
    let resp = resp.await;

    println!("Hello, res {:?}", resp);
    let v: Value = serde_json::from_str(&resp).unwrap();

    println!("v {:?}",v.as_str());
    
    config.bearer = v["access_token"].as_str().unwrap().to_string();
    config.login = true;
    config
  }
}
