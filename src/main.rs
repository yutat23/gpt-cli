use std::env;
use std::error::Error;
use std::time::Duration;
use reqwest::{blocking::Client, header};
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn Error>> {
    let (message, timeout, max_tokens) = parse_args();

    println!("You: {:?}", message);
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        },
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(timeout))
        .build()?;

    let url = "https://api.openai.com/v1/chat/completions";
    let payload = json!({
        "model": "gpt-3.5-turbo",
        "max_tokens": max_tokens,
        "messages": [
            {
                "role": "user",
                "content": message
            }
        ]
    });

    let response = client
        .post(url)
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::AUTHORIZATION, format!("Bearer {}", api_key))
        .json(&payload)
        .send()?;

    let buffer = response.text()?;
    let json_response: Value = serde_json::from_str(&buffer)?;
    let content = json_response["choices"][0]["message"]["content"].as_str().unwrap_or_default();
    println!("{}", content);

    Ok(())
}

fn parse_args() -> (String, u64, u32) {
    let args: Vec<String> = env::args().collect();

    let mut message = String::new();
    let mut timeout = 30u64;
    let mut max_tokens = 512u32;
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-h" | "--help" => {
                print_usage();
                std::process::exit(0);
            }
            "-t" | "--timeout" => {
                timeout = args[i + 1].parse::<u64>().unwrap_or(30);
                i += 2;
            }
            "-m" | "--message" => {
                message = args[i + 1].clone();
                i += 2;
            }
            "--max_tokens" => {
                max_tokens = args[i + 1].parse::<u32>().unwrap_or(512);
                i += 2;
            }
            _ => {
                message = args[i].clone();
                i += 1;
            }
        }
    }

    (message, timeout, max_tokens)
}

fn print_usage() {
    println!("Usage: cargo run [OPTIONS] [MESSAGE]");
    println!("Options:");
    println!("  -h, --help        Show this help message and exit");
    println!("  -t, --timeout     Set the request timeout in seconds (default: 30)");
    println!("  -m, --message     Set the input message for the chatbot");
    println!("  --max_tokens      Set the maximum number of tokens for the response (default: 512)");
}
