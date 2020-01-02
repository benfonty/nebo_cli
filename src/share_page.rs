use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::SecondsFormat;

use std::error::Error;

use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use reqwest::blocking::Client;
use reqwest::blocking::ClientBuilder;
use reqwest::header::HeaderMap;

use super::common;

#[derive(Serialize)]
struct PageMetadata<'a> {
    #[serde(rename = "pageTitle")]
    page_title: &'a str,
    #[serde(rename = "pageId")]
    page_id: &'a str,
    #[serde(rename = "lastModificationDate")]
    last_modification_date: &'a str,
    #[serde(rename = "creationDate")]
    creation_date: &'a str
}

#[derive(Serialize)]
struct Page<'a> {
    uuid: &'a str,
    signature: &'a str,
    metadata: PageMetadata<'a>
}

#[derive(Deserialize, Debug)]
struct Credentials {
    #[serde(rename = "accessToken")]
    access_token: String,
    #[serde(rename = "identityPoolId")]
    identity_pool_id: String,
    region: String,
    #[serde(rename = "identityId")]
    identity_id: String,
    #[serde(rename = "identityProvider")]
    identity_provider: String
}

#[derive(Deserialize, Debug)]
struct S3 {
    bucket: String,
    #[serde(rename = "clientDirectoryPrefix")]
    client_directory_prefix: String,
    region: String,
    #[serde(rename = "kmsKey")]
    kms_key: String
}

#[derive(Deserialize, Debug)]
struct Configuration {
    #[serde(rename = "sharingUrlPrefix")]
    sharing_url_prefix: String,
    s3: S3,
    credentials: Credentials
}

impl<'a> Page<'a> {
    fn new(uuid: &'a str, signature: &'a str, title: &'a str, date: &'a str) -> Page<'a> {
        Page {
            uuid: uuid,
            signature: signature,
            metadata: PageMetadata {
                page_title: title,
                page_id: "toto",
                last_modification_date: date,
                creation_date: date
            }
        }
    }
}

fn get_default_client(token: &str) -> Client {
    let mut headers = HeaderMap::new();
    headers.insert(http::header::AUTHORIZATION, token.parse().unwrap());
    headers.insert(http::header::CONTENT_TYPE, "application/json".parse().unwrap());
    
    ClientBuilder::new()
        .default_headers(headers)   
        .build()
        .unwrap()
}

pub fn share_page(env: &str, token: &str, uuid: &str, signature: Option<&str>, filename: &str, title: Option<&str>) -> Result<(), Box<dyn Error>> {
    let now = Into::<DateTime<Utc>>::into(SystemTime::now());
    let signature = match signature {
        Some(s) => Cow::from(s),
        None => Cow::from(String::from(format!("{}", now.timestamp())))
    };
    let title = match title {
        Some(s) => Cow::from(s),
        None => Cow::from(format!("the page {}", uuid))
    };
    let date = now.to_rfc3339_opts(SecondsFormat::Secs, true);
    
    let client = get_default_client(token);

    println!("sharing page {} on {}", &uuid, &env);
    call_share_api(env, &client, uuid, &signature, &title, &date)?;
    let configuration = call_configutation_api(env, &client)?;
    println!("{:?}", configuration);
    Ok(())
} 

fn call_configutation_api(env: &str, client: &Client) -> Result<Configuration, Box<dyn Error>>{
    print!("Calling configuration api... ");
    let response = client
        .get(format!("{}/api/v1.0/nebo/configuration", common::ENV[env].neboapp_url).as_str())
        .send()?;
    
    let status = response.status();
    let text = response.text()?;
    if !status.is_success() {
        return Err(Box::from("error during call to configuration api"));
    }
    println!("ok");
    Ok(serde_json::from_str(&text).unwrap())
}

fn call_share_api(env: &str, client: &Client,uuid: &str, signature: &str, title: &str, date: &str) -> Result<(), Box<dyn Error>> {
    print!("Calling share api... ");
    let serialized = serde_json::to_string(&Page::new (&uuid, &signature, &title, &date)).unwrap();
    let response = client
        .post(format!("{}/api/v2.0/nebo/pages", common::ENV[env].neboapp_url).as_str())
        .body(serialized)
        .send()?;
    
    let status = response.status();
    let _dummy = response.text();
    if !status.is_success() {
        return Err(Box::from("error during call to share api"));
    }
    println!("ok");
    Ok(())
}