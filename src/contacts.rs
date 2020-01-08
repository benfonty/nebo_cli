use std::error::Error;

use super::common;

use reqwest::StatusCode;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Contact<'a> {
    email: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<&'a str>,
    permission: &'a str
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ContactPayload<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<&'a str>,
    contacts: Vec<Contact<'a>>
}

impl<'a> ContactPayload<'a> {
    fn new(mail: &'a str, name: Option<&'a str>, message: Option<&'a str>) -> ContactPayload<'a> {
        let contact = Contact{email: mail, name: name, permission: "READ"};
        let mut v = Vec::with_capacity(1);
        v.push(contact);
        ContactPayload {
            message: message,
            contacts: v
        }
    }
}

pub fn add_contact(env: &str, token: &str, uuid: &str, email: &str, name: Option<&str>, message: Option<&str>) -> Result<(), Box<dyn Error>> {
    print!("adding contact... ");
    let  payload = serde_json::to_string(&ContactPayload::new(email, name, message))?;
    let response = common::get_default_client(token)
        .post(format!("{}{}/{}/contacts", common::ENV[env].neboapp_url, common::NEBO_API_URI_PAGES, uuid).as_str())
        .body(payload)
        .send()?;
    
    let status = response.status();
    let _dummy = response.text();
    if status.is_success() {
        println!("ok");
    }
    else if status == StatusCode::NOT_FOUND {
        return Err(Box::from("page not found"));
    }
    else {
        return Err(Box::from(format!("error duting add contact {}", _dummy?)));
    }
    
    Ok(())
}

pub fn remove_contact (
    env: &str, 
    token: &str, 
    uuid: &str,
    email: &str 
    ) -> Result<(), Box<dyn Error>> {
    print!("deleting contact... ");
    let response = common::get_default_client(token)
        .delete(format!("{}{}/{}/contacts/{}", common::ENV[env].neboapp_url, common::NEBO_API_URI_PAGES, uuid, email).as_str())
        .send()?;
        
    let status = response.status();
    let _dummy = response.text();
    if status.is_success() {
        println!("ok");
    }
    else {
        return Err(Box::from(format!("error during call to api {}", _dummy?)));
    }

    Ok(())
}