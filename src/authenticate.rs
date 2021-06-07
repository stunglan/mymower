use dotenv::dotenv;
use std::collections::HashMap;
use std::env;
use std::fmt;

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
      write!(f, "({}, {}, {}, {}, {},{})", self.user_name, self.user_password,self.application_key,self.auth_uri,self.application_key,self.bearer)
  }
}

impl ServerConfig {

  async fn login(user_name: String, user_password: String, application_key: String,auth_uri: String) -> Result<String, Box<dyn std::error::Error>> {
    let mut map = HashMap::new();
    map.insert("grant_type", "password");
    map.insert("client_id", &application_key);
    map.insert("username", &user_name);
    map.insert("password", &user_password);

    let client = reqwest::Client::new();
    let resp = client.post(auth_uri).form(&map).send().await?;

    println!("Hello, login {:?}", resp);

    println!("Hello, test!");
    Ok(resp.text().await?)

  }

  pub async fn new() -> ServerConfig {
    dotenv().ok();

    let user_name = get_env("USER_NAME");
    let user_password = get_env("USER_PASSWORD");
    let application_key = get_env("APPLICATION_KEY");
    let auth_uri = get_env("AUTH_URI");

    let resp = ServerConfig::login(user_name, user_password, application_key,auth_uri);

    println!("Hello, res {:?}", resp.await);

    ServerConfig {
      user_name: String::from("String"),
      user_password: String::from("String"),
      application_key: String::from("String"),
      auth_uri: String::from("String"),
      app_uri: String::from("String"),
      bearer: String::from("String"),
    }
  }
}


