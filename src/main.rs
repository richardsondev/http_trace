use std::{env, process};
use reqwest::{Client, Response};
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
        Ok(response) => {
            println!("{}", response);
            process::exit(0);
        }
        Err(e) => {
            eprintln!("TRACE request to '{}' failed: {}", url, e);
            process::exit(1);
        }
    }
}

async fn perform_trace(url: &str) -> Result<String, String> {
    let client = Client::new();

    // Perform the TRACE request
    let response = client
        .request(reqwest::Method::TRACE, url)
        .send()
        .await
        .map_err(|e| format!("Failed to send TRACE request: {}", e))?;

    // Ensure the response was successful
    if response.status().is_success() {
        format_trace_response(response).await
    } else {
        Err(format!(
            "Received non-success status code: {}",
            response.status()
        ))
    }
}

async fn format_trace_response(response: Response) -> Result<String, String> {
    let version = match response.version() {
        reqwest::Version::HTTP_09 => "0.9",
        reqwest::Version::HTTP_10 => "1.0",
        reqwest::Version::HTTP_11 => "1.1",
        reqwest::Version::HTTP_2 => "2",
        reqwest::Version::HTTP_3 => "3",
        _ => "Unknown",
    };

    let status_line = format!("HTTP/{} {} {}", 
        version, 
        response.status().as_u16(), 
        response.status().canonical_reason().unwrap_or(""));

    let headers = response
        .headers()
        .iter()
        .map(|(key, value)| format!("{}: {}", key, value.to_str().unwrap_or("<invalid UTF-8>")))
        .collect::<Vec<_>>()
        .join("\n");

    let body = response
        .text()
        .await
        .map_err(|e| format!("Failed to read response body: {}", e))?;

    Ok(format!("{}\n{}\n\n{}", status_line, headers, body))
}
