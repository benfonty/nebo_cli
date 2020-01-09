use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::SecondsFormat;

use std::error::Error;

use std::borrow::Cow;

use serde::Serialize;

use reqwest::blocking::Client;

use super::common;
use super::configuration::Configuration;
mod aws;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PageMetadata<'a> {
    page_title: &'a str,
    page_id: &'a str,
    last_modification_date: &'a str,
    creation_date: &'a str
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
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



pub fn share_page(
    env: &str, 
    token: &str, 
    uuid: &str, 
    signature: Option<&str>, 
    filename: &str, 
    title: Option<&str>, 
    share_with_myscript: Option<&str>,
    collect_login: Option<&str>) -> Result<(), Box<dyn Error>> {
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
    
    let client = common::get_default_client(token);

    println!("sharing page {} on {}", &uuid, &env);
    call_share_api(env, &client, uuid, &signature, &title, &date)?;
    let configuration = Configuration::get(env, &client)?;

    aws::upload_file(
        filename,
        uuid, 
        &signature,
        configuration.credentials,
        configuration.s3,
        share_with_myscript,
        collect_login)?;
    Ok(())
} 

fn call_share_api(env: &str, client: &Client, uuid: &str, signature: &str, title: &str, date: &str) -> Result<(), Box<dyn Error>> {
    print!("Calling share api... ");
    let serialized = serde_json::to_string(&Page::new (&uuid, &signature, &title, &date)).unwrap();
    let response = client
        .post(format!("{}{}", common::ENV[env].neboapp_url, common::NEBO_API_URI_PAGES).as_str())
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