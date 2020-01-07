use std::error::Error;

use super::common;

use reqwest::StatusCode;

pub fn delete_page(
    env: &str, 
    token: &str, 
    uuid: &str, 
    ) -> Result<(), Box<dyn Error>> {
    println!("deleting page {} on {}", &uuid, &env);
    print!("deleting page... ");
    let response = common::get_default_client(token)
        .delete(format!("{}{}/{}", common::ENV[env].neboapp_url, common::NEBO_API_URI_PAGES, uuid).as_str())
        .send()?;
        
    let status = response.status();
    let _dummy = response.text();
    if status.is_success() {
        println!("ok");
    }
    else if status == StatusCode::NOT_FOUND {
        println!("was already deleted");
    }
    else {
        return Err(Box::from("error during call to share api"));
    }

    Ok(())
}