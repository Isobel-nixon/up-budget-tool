
use dotenv::dotenv;
use serde;
use reqwest;
use chrono::{self, TimeZone};


#[macro_use]
extern crate dotenv_codegen;

const BASE_URL: &str = "https://api.up.com.au/api/v1/transactions";

fn main() {
    get_up_data().unwrap();
}

fn get_up_data() -> Result<(), reqwest::Error>{
    dotenv().ok();
    let api_key = dotenv!("API_KEY");

    let header = format!("Authorization: Bearer {api_key}");

    let params = [("filter[since]", dbg!(chrono::Utc.with_ymd_and_hms(2024,04,1, 0, 0, 0).unwrap().to_rfc3339()))];

    let url = reqwest::Url::parse_with_params(BASE_URL, &params).unwrap();
    
    let client = reqwest::blocking::Client::new();
    let data = client
        .get(url)
        .bearer_auth(api_key)
        .send()?
        .text()?;

    println!("{}", data);
    Ok(())
}
