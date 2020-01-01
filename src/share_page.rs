use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::SecondsFormat;

use std::error::Error;

use std::borrow::Cow;

use serde::{Serialize};

use reqwest::blocking::Client;
use reqwest::StatusCode;

use super::common;

#[derive(Serialize)]
struct PageMetadata<'a> {
    pageTitle: &'a str,
    pageId: &'a str,
    lastModificationDate: &'a str,
    creationDate: &'a str
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
                pageTitle: title,
                pageId: "toto",
                lastModificationDate: date,
                creationDate: date
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
    let serialized = serde_json::to_string(&Page::new (&uuid, &signature, &title, &date)).unwrap();
    let response = Client::new()
        .post(format!("{}/api/v2.0/nebo/pages", common::ENV[env].neboapp_url).as_str())
        .header(http::header::AUTHORIZATION, token)
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(serialized)
        .send()?;
    
    let status = response.status();
    println!("{}", response.text()?);
    if !status.is_success() {
        return Err(Box::from("error during call to api"));
    }
    
    Ok(())
} 