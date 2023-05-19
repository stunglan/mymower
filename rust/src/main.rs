mod authenticate;
mod getwebsocket;

use serde_json::Value;

#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let config = authenticate::ServerConfig::new().await;

    println!("\nconfig {:}", config);

    let mowers = match config.get_mowers().await {
        Ok(response) => {
            let parsed_json: Result<Value, reqwest::Error> = response.json().await;

            // Further processing of the response, e.g., parsing the JSON, goes here
            match parsed_json {
                Ok(json_value) => {
                    json_value
                    // Further processing of the JSON, e.g., accessing specific values, goes here
                }
                Err(error) => {
                    println!("Failed to parse JSON: {}", error);
                    panic!();
                }
            }
        }
        Err(error) => {
            println!("Error: {:?}", error);
            panic!();
        }
    };
    println!(
        "\nParsed JSON id after getmowers: {:#?}",
        &mowers["data"][0]["id"]
    );

    let mower = match config
        .get_mower(&mowers["data"][0]["id"].as_str().unwrap())
        .await
    {
        Ok(response) => {
            let parsed_json: Result<Value, reqwest::Error> = response.json().await;

            // Further processing of the response, e.g., parsing the JSON, goes here
            match parsed_json {
                Ok(json_value) => {
                    json_value
                    // Further processing of the JSON, e.g., accessing specific values, goes here
                }
                Err(error) => {
                    println!("Failed to parse JSON in getmover: {}", error);
                    panic!();
                }
            }
        }
        Err(error) => {
            println!("Error after getmower: {:?}", error);
            panic!();
        }
    };
    println!("\nParsed JSON id after getmower: {:#?}", &mower);

 
    println!("\nParsed config after getmower:{:#?} {:#?}",&config.husqvarna_uri,&config.bearer);

    getwebsocket::get_data(&config.husqvarna_uri, &config.bearer);
}
