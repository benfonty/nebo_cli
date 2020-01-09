use std::error::Error;

use super::common;

use reqwest::StatusCode;

use log::{info};

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
        return Err(Box::from("error during call to share api"));
    }

    Ok(())
}