use std::{env, process};
use reqwest::Client;
use url::Url;

#[tokio::main]
async fn main() {
    // Get the command line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure a URL is provided
    if args.len() != 2 {
        eprintln!("USAGE: {} <url>", args[0]);
        process::exit(1);
    }

    let url = &args[1];

    // Validate the URL
    if Url::parse(url).is_err() {
        eprintln!("Error: The provided URL '{}' is not valid.", url);
        process::exit(1);
    }

    // Perform the TRACE request
    match perform_trace(url).await {
        Ok(_) => {
            println!("TRACE request to '{}' succeeded.", url);
            process::exit(0);
        }
        Err(e) => {
            eprintln!("TRACE request to '{}' failed: {}", url, e);
            process::exit(1);
        }
    }
}

async fn perform_trace(url: &str) -> Result<(), reqwest::Error> {
    let client = Client::new();

    // Perform the TRACE request
    let response = client
        .request(reqwest::Method::TRACE, url)
        .send()
        .await?;

    // Ensure the response was successful
    if response.status().is_success() {
        Ok(())
    } else {
        Err(reqwest::Error::new(
            reqwest::StatusCode::from_u16(response.status().as_u16()).unwrap(),
        ))
    }
}
