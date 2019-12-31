use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;

use std::error::Error;

use std::borrow::Cow;

pub fn share_page(env: &str, login: &str, uuid: &str, signature: Option<&str>, filename: &str, title: Option<&str>) -> Result<(), Box<dyn Error>> {
    let signature = match signature {
        Some(s) => Cow::from(s),
        None => Cow::from(String::from(format!("{}", Into::<DateTime<Utc>>::into(SystemTime::now()).timestamp())))
    };
    let title = match title {
        Some(s) => Cow::from(s),
        None => Cow::from(format!("the page {}", uuid))
    };
    println!("sharing page {} on {}", &uuid, &env);
    Ok(())
} 