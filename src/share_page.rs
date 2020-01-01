use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::SecondsFormat;

use std::error::Error;

use std::borrow::Cow;

use serde::{Serialize};

#[derive(Serialize)]
struct PageMetadata<'a> {
    title: &'a str,
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
                title: title,
                lastModificationDate: date,
                creationDate: date
            }
        }
    }
}

pub fn share_page(env: &str, login: &str, uuid: &str, signature: Option<&str>, filename: &str, title: Option<&str>) -> Result<(), Box<dyn Error>> {
    let now = Into::<DateTime<Utc>>::into(SystemTime::now());
    let signature = match signature {
        Some(s) => Cow::from(s),
        None => Cow::from(String::from(format!("{}", now.timestamp())))
    };
    let title = match title {
        Some(s) => Cow::from(s),
        None => Cow::from(format!("the page {}", uuid))
    };
    let date = now.to_rfc3339_opts(SecondsFormat::Millis, true);
    println!("sharing page {} on {}", &uuid, &env);
    let serialized = serde_json::to_string(&Page::new (&uuid, &signature, &title, &date)).unwrap();
    println!("{}", serialized);
    
    Ok(())
} 