use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::SecondsFormat;

use std::error::Error;

use std::borrow::Cow;

use serde::{Serialize};

use reqwest::blocking::Client;

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
    
    println!("sharing page {} on {}", &uuid, &env);
    call_share_api(env, token, uuid, &signature, &title, &date)
} 

fn call_share_api(env: &str, token: &str,uuid: &str, signature: &str, title: &str, date: &str) -> Result<(), Box<dyn Error>> {
    print!("Calling api... ");
    let serialized = serde_json::to_string(&Page::new (&uuid, &signature, &title, &date)).unwrap();
    let response = Client::new()
        .post(format!("{}/api/v2.0/nebo/pages", common::ENV[env].neboapp_url).as_str())
        .header(http::header::AUTHORIZATION, token)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(serialized)
        .send()?;
    
    let status = response.status();
    let _dummy = response.text();
    if !status.is_success() {
        return Err(Box::from("error during call to api"));
    }
    println!("ok");
    return Ok(());
}