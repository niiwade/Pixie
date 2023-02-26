use dotenv::dotenv;
use hyper::{body::Buf, header, Body, Client, Request};
use hyper_tls::HttpsConnector;
// use serde::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::{env, io::{stdin, stdout, Write}};


#[derive(serde_derive::Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>,
}

#[derive(serde_derive::Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(serde_derive::Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    dotenv().ok();
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);
    let uri = "https://api.openai.com/v1/engines/davinci-codex/completions";
    let prompt = "Generate and explain code for a given problem";
    let oai_token = env::var("OAI_TOKEN").unwrap();
    let auth_header_val = format!("Bearer {}", oai_token);
    println!("{esc}c", esc = 27 as char);
    loop {
        println!("> ");
        stdout().flush().unwrap();
        let mut user_text = String::new();
        stdin().read_line(&mut user_text).expect("Failed to read line");
        println!();
        let mut sp = Spinner::new(Spinners::Dots12, "\t\t Pixie is thinking.........".into());
        let oai_request = OAIRequest {
            prompt: format!("{} {}", prompt, user_text),
            max_tokens: 1000,
        };
        let body = serde_json::to_vec(&oai_request)?;
        let req = Request::builder()
            .method("POST")
            .uri(uri)
            .header(header::CONTENT_TYPE, "application/json")
            .header("Authorization", &auth_header_val)
            .body(Body::from(body))?;
        let res = client.request(req).await?;
        let body = hyper::body::aggregate(res).await?;
        let json: OAIResponse = serde_json::from_reader(body.reader())?;
        sp.stop();
        println!("{}", json.choices[0].text);
    }
}
