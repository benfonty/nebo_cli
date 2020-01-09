use serde::Deserialize;
use super::common;

use std::error::Error;

use reqwest::blocking::Client;

use log::debug;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Credentials {
    pub access_token: String,
    pub identity_pool_id: String,
    pub region: String,
    pub identity_id: String,
    pub identity_provider: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct S3Configuration {
    pub bucket: String,
    pub client_directory_prefix: String,
    pub region: String,
    pub kms_key: String,
    pub service_endpoint: Option<String>
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    pub sharing_url_prefix: String,
    pub s3: S3Configuration,
    pub credentials: Credentials
}

impl Configuration {
    pub fn get(env: &str, client: &Client) -> Result<Configuration, Box<dyn Error>>{
        debug!("Begin Calling configuration api");
        let response = client
            .get(format!("{}/api/v1.0/nebo/configuration", common::ENV[env].neboapp_url).as_str())
            .send()?;
    
        let status = response.status();
        let text = response.text()?;
        if !status.is_success() {
            return Err(Box::from("error during call to configuration api"));
        }
        debug!("End Calling configuration ok");
        Ok(serde_json::from_str(&text)?)
    }
}