use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::fmt;
use serde_json::Value;

#[derive(Debug)]
pub struct ServerConfig {
  user_name: String,
  user_password: String,
  application_key: String,
  auth_uri: String,
  app_uri: String,
  bearer: String,
}

fn get_env(key: &str) -> String {
  match env::var(key) {
    Ok(val) => val,
    Err(e) => {
      panic!("couldn't interpret {} got {}", key, e)
    }
  }
}

impl fmt::Display for ServerConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "(user_name: {},  application_key: {}, auth_uri: {}, app_uri: {}, bearer: {})",
      self.user_name,
      self.application_key,
      self.auth_uri,
      self.app_uri,
      self.bearer
    )
  }
}

impl ServerConfig {
  async fn login(
    user_name: &str,
    user_password: &str,
    application_key: &str,
    auth_uri: &str,
  ) -> String {
    let mut map = HashMap::new();
    map.insert("grant_type", "password");
    map.insert("client_id", &application_key);
    map.insert("username", &user_name);
    map.insert("password", &user_password);

    let client = reqwest::Client::new();
    let resp = client.post(auth_uri).form(&map).send().await;
    match resp {
      Ok(resp) => {

        let result = resp.text().await;
        match result {
          Ok(value) => {
            return value;
          }
          Err(err) => {
            panic!("Error in extracting text from what returned from {:}\nErr; {:}",auth_uri, err)
          }
        }
      }
      Err(err) => {
        panic!("Error in extracting text from  {:}\nErr; {:}", auth_uri,err)
      }
    }
  }

  pub async fn new() -> ServerConfig {
    dotenv().ok();

    let user_name = get_env("USER_NAME");
    let user_password = get_env("USER_PASSWORD");
    let application_key = get_env("APPLICATION_KEY");
    let auth_uri = get_env("AUTH_URI");
    let app_uri = get_env("AUTH_URI");

    let resp = ServerConfig::login(&user_name, &user_password, &application_key, &auth_uri);
    
    let resp = resp.await;

    println!("Hello, res {:?}", resp);
    let v: Value = serde_json::from_str(&resp).unwrap();

    ServerConfig {
      user_name: String::from(user_name),
      user_password: String::from(user_password),
      application_key: String::from(application_key),
      auth_uri: String::from(auth_uri),
      app_uri: String::from(app_uri),
      bearer: v["access_token"].to_string(),
    }
  }
}
