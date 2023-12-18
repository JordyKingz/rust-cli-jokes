use clap::{Arg, Command};
use reqwest;
use tokio;
use serde_json;

async fn fetch_joke(category: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://sv443.net/jokeapi/v2/joke/{}", category);

    let response = reqwest::get(url).await?.text().await?;

    let json: serde_json::Value = serde_json::from_str(&response).unwrap();
    let setup = json["setup"].as_str().unwrap();
    let delivery = json["delivery"].as_str().unwrap();

    let joke = format!("{}\n{}", setup, delivery);
    Ok(joke)
}

#[tokio::main]
async fn main() {
    let matches = Command::new("joke")
        .version("1.0")
        .about("Tells a joke based on provided category")
        .arg(Arg::new("category")
             .required(false)// empty will result in "any" category
             .index(1))
        .get_matches();

    if let Some(category) = matches.value_of("category") {
        println!("Searching for {} jokes...", category);
        match fetch_joke(category).await {
            Ok(joke) => println!("{}", joke),
            Err(e) => eprintln!("Error fetching joke: {}", e),
        }
    } else {
        println!("Searching for any jokes...");
        match fetch_joke("any").await {
            Ok(joke) => println!("{}", joke),
            Err(e) => eprintln!("Error fetching joke: {}", e),
        }
    }
}