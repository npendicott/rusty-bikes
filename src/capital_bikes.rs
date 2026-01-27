use std::error::Error;
use bytes::Bytes;

use reqwest::blocking::Client;

use serde::{Serialize, Deserialize};
use serde_xml_rs;


// Capital Bike API
const BIKESHARE_HISTORY_URL: &str = "https://s3.amazonaws.com/capitalbikeshare-data";


// API data structures
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ListBucketResult {
    pub name: String,
    pub max_keys: i32,
    pub is_truncated: bool,
    pub contents: Vec<Contents>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Contents {
    pub key: String,
    pub last_modified: String,
    pub e_tag: String,
    pub size: i32,
    pub storage_class: String, 
}



pub fn get_bikeshare_history_index() -> Result<ListBucketResult, Box<dyn Error>> {

    let bikeshare_history_index_url: String = format!("{BIKESHARE_HISTORY_URL}");
    println!("Getting index from {bikeshare_history_index_url}");

    // Pull resp - One off
    let resp = reqwest::blocking::get(bikeshare_history_index_url)?;

    // // Pull resp - Client
    // let client = Client::new();
    // let resp = client.get(bikeshare_history_index_url).send()?;
    
    // Parse output
    let body = resp.text()?;
    let index: ListBucketResult = serde_xml_rs::from_str(body.as_str()).unwrap();

    Ok(index)
}

pub fn get_bikeshare_history_file(historic_file_ref: &Contents) -> Result<Bytes, Box<dyn Error>> {

    // Pull resp - Client
    let bikeshare_history_file_url: String = format!("{BIKESHARE_HISTORY_URL}/{0}", historic_file_ref.key);
    println!("Getting index from {bikeshare_history_file_url}");

    let client = Client::new();
    let resp = client.get(bikeshare_history_file_url).send()?;

    let body = resp.bytes()?;

    Ok(body)
}
 