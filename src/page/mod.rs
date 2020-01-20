use std::time::SystemTime;
use chrono::offset::Utc;
use chrono::DateTime;
use chrono::SecondsFormat;

use std::error::Error;

use std::borrow::Cow;

use serde::{Serialize, Deserialize};

use reqwest::blocking::Client;
use reqwest::StatusCode;

use rusoto_core::credential::StaticProvider;

use super::common;
use super::configuration::Configuration;
pub mod aws;

use log::{info, debug, error};

use threadpool::ThreadPool;

const NB_THREADS_DELETE: usize = 10;
pub const NB_THREADS_SHAREPAGES: usize = 10;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PageMetadata {
    page_title: String,
    page_id: String,
    last_modification_date: String,
    creation_date: String
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Page {
    uuid: String,
    signature: String,
    metadata: PageMetadata
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Pages {
    content: Vec<Page>
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PageOption {
     visibility: String
}

impl Page {
    fn new(uuid: &str, signature: &str, title: &str, date: &str) -> Page {
        Page {
            uuid: uuid.into(),
            signature: signature.into(),
            metadata: PageMetadata {
                page_title: title.into(),
                page_id: "toto".into(),
                last_modification_date: date.into(),
                creation_date: date.into()
            }
        }
    }
}

pub fn make_private(env: &str, token: &str, uuid: &str) -> Result<(), Box<dyn Error>> {
    info!("Setting page {} private", uuid);
    let client = common::get_default_client(token);
    let serialized = serde_json::to_string(&PageOption{visibility: "PRIVATE".into()}).unwrap();
    let response = client
        .put(format!("{}{}/{}", common::ENV[env].neboapp_url, common::NEBO_API_URI_PAGES, uuid).as_str())
        .body(serialized)
        .send()?;
    
    let status = response.status();
    let _dummy = response.text();
    if !status.is_success() {
        return Err(Box::from(format!("error during setting page private {}", _dummy.unwrap())));
    }
    info!("Setting page {} private OK", uuid);
    Ok(())
}

pub fn share_page(
    env: &str, 
    token: &str, 
    uuid: &str, 
    signature: Option<&str>, 
    filename: &str, 
    title: Option<&str>, 
    share_with_myscript: Option<&str>,
    collect_login: Option<&str>,
    credential_provider: StaticProvider,
    configuration: &Configuration ) -> Result<(), Box<dyn Error>> {
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

    info!("Begin share page {}", uuid);
    call_share_api(env, &client, uuid, &signature, &title, &date)?;

    aws::upload_file(
        filename,
        uuid, 
        &signature,
        &configuration,
        credential_provider,
        share_with_myscript,
        collect_login)?;
    info!("End share page {} OK", uuid);
    Ok(())
} 

fn call_share_api(env: &str, client: &Client, uuid: &str, signature: &str, title: &str, date: &str) -> Result<(), Box<dyn Error>> {
    debug!("Begin Calling share api");
    let serialized = serde_json::to_string(&Page::new (&uuid, &signature, &title, &date)).unwrap();
    let response = client
        .post(format!("{}{}", common::ENV[env].neboapp_url, common::NEBO_API_URI_PAGES).as_str())
        .body(serialized)
        .send()?;
    
    let status = response.status();
    let _dummy = response.text();
    if !status.is_success() {
        return Err(Box::from(format!("error during call to delete api {}", _dummy.unwrap())));
    }
    debug!("End Calling share api OK");
    Ok(())
}

pub fn delete_page(
    env: &str, 
    token: &str, 
    uuid: &str, 
    ) -> Result<(), Box<dyn Error>> {
    info!("Begin deleting page {}", &uuid);
    let response = common::get_default_client(token)
        .delete(format!("{}{}/{}", common::ENV[env].neboapp_url, common::NEBO_API_URI_PAGES, uuid).as_str())
        .send()?;
        
    let status = response.status();
    let _dummy = response.text();
    if status.is_success() {
        info!("End deleting page {} OK", &uuid);
    }
    else if status == StatusCode::NOT_FOUND {
        info!("End deleting page {} OK (was already deleted)", &uuid);
    }
    else {
        return Err(Box::from(format!("error during call to delete api {} for {}", _dummy.unwrap(), uuid)));
    }

    Ok(())
}

pub fn delete_pages(
    env: &str, 
    token: &str, 
    login: &str, 
    ) -> Result<(), Box<dyn Error>> {
    
    let client = common::get_default_client(token);

    let pages = get_pages(env, &client, login)?;
    if pages.content.len() == 0 {
        info!("no page to delete");
    }
    let pool = ThreadPool::with_name("delete".into(), NB_THREADS_DELETE);
    for page in pages.content {
        let (env, token, uuid) = (
            String::from(env),
            String::from(token),
            page.uuid.clone()
        );
        pool.execute(move || {
            if let Err(e) = delete_page(&env, &token, &uuid) {
                error!("{}", e)
            }
        });
    }
    pool.join();
    Ok(())
}

fn get_pages(env: &str, client: &Client, login: &str) -> Result<Pages, Box<dyn Error>> {
    info!("Begin Getting pages for {}", login);
    let response = client
        .get(format!("{}{}", common::ENV[env].neboapp_url, common::NEBO_API_URI_PAGES).as_str())
        .send()?;
    
    let status = response.status();
    let text = response.text()?;
    
    if !status.is_success() {
        return Err(Box::from(format!("error during call to get pages {}", text)));
    }
    info!("End Getting pages ok");
    Ok(serde_json::from_str(&text)?)
}