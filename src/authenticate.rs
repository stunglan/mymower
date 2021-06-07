use dotenv::dotenv;
use std::collections::HashMap;
use std::env;

fn get_env(key: &str) -> String {
  match env::var(key) {
    Ok(val) => val,
    Err(e) => {
      panic!("couldn't interpret {} got {}", key, e)
    }
  }
}

pub async fn test1() -> Result<String, Box<dyn std::error::Error>> {
 

  dotenv().ok();

  println!("get_uri: {}", get_env("REDIRECT_URI"));

  let user_name = get_env("USER_NAME");
  let user_password = get_env("USER_PASSWORD");
  let application_key = get_env("APPLICATION_KEY");

  let mut map = HashMap::new();
  map.insert("grant_type", "password");
  map.insert("client_id", &application_key);
  map.insert("username", &user_name);
  map.insert("password", &user_password);

  for (key, value) in &map {
    println!("{}: {}", key, value);
  }
  let client = reqwest::Client::new();
  let resp = client.post("https://api.authentication.husqvarnagroup.dev/v1/oauth2/token")
    .form(&map)
    .send()
    .await?;

  
  println!("Hello, res {:?}", resp);

  println!("Hello, test!");
  Ok(resp.text().await?)
}
