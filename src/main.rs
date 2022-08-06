use anyhow::Result;
use base64::encode;
use clap::Parser;
use dotenv::dotenv;
use reqwest::{
    blocking::Client,
    header::{AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE},
};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::Path};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about=None)]
struct Args {
    #[clap(short, long, value_parser)]
    filepath: String,
}

#[derive(Serialize, Deserialize)]
enum Action {
    Create,
    Update,
    Upsert,
    Delete,
}

#[derive(Serialize, Deserialize)]
struct Payload {
    action: Action,
    filename: String,
    code: String,
}

fn upload(url: &str, token: &str, filepath: &Path) -> Result<()> {
    let filename = filepath.file_name().unwrap().to_str().unwrap().into();
    let code = encode(fs::read_to_string(filepath).unwrap());

    let payload = Payload {
        action: Action::Upsert,
        filename,
        code,
    };

    let payload_str = serde_json::to_string(&payload).unwrap();
    let payload_len = payload_str.len();
    //println!("{}", payload_str);

    let client = Client::new();
    let resp = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_LENGTH, payload_len)
        .header(AUTHORIZATION, format!("Bearer {}", token))
        .json(&payload)
        .send()?;

    if resp.status().is_success() {
        println!("success!");
    } else if resp.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", resp.status());
    }
    //println!("{:?}", resp);
    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();
    let url: String = env::var("URL")?;
    let token: String = env::var("API_KEY")?;
    let args = Args::parse();
    let filepath = Path::new(&args.filepath);

    upload(&url, &token, filepath)?;

    Ok(())
}
