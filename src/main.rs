mod authenticate;

use serde_json::Value;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let config = authenticate::ServerConfig::new().await;

    println!("\nconfig {:}", config);


    match config.get_mowers().await {
        Ok(response) => {
            let parsed_json: Result<Value, reqwest::Error> = response.json().await;
            
            // Further processing of the response, e.g., parsing the JSON, goes here
            match parsed_json {
                Ok(json_value) => {
                    println!("Parsed JSON: {:#?}", &json_value);
                    
                    // Further processing of the JSON, e.g., accessing specific values, goes here
                }
                Err(error) => {
                    println!("Failed to parse JSON: {}", error);
                }
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
}
