use std::env;
use std::error::Error;
use reqwest::{blocking::Client, header};
use serde_json::{json, Value};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let message = &args[1];
    println!("You: {:?}", message);
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        },
    };

    let client = Client::new();
    let url = "https://api.openai.com/v1/chat/completions";
    let payload = json!({
        "model": "gpt-3.5-turbo",
        "max_tokens": 512,
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