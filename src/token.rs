use reqwest;
use reqwest::RedirectPolicy;
use reqwest::Client;
use reqwest::StatusCode;

use std::error::Error;
use dialoguer::PasswordInput;
use std::env;
use url::Url;

use super::common;
use std::ops::Deref;

pub fn token(env: &str, login: &str) -> Result<String, Box<dyn Error>> {
    let client = reqwest::ClientBuilder::new()
        .redirect(RedirectPolicy::none())
        .cookie_store(true)
        .build()?;
    
    let sso_url = common::ENV[env].sso_url;    
    
    first_call(&client, env, sso_url)?;
    let location = second_call(&client, sso_url, login)?;
    Ok(third_call(&client, sso_url, location.as_str())?)
} 

fn first_call(client: &Client, env: &str, sso_url: &str) -> Result<(), Box<dyn Error>> {
    let response = client
        .get(format!("{}/oauth/authorize",sso_url).as_str())
        .query(&[
            ("client_id", common::ENV[env].client_id),
            ("response_type", "token"),
            ("scope", "read"),
            ("redirect_uri", common::ENV[env].sso_redirect_uri)
        ])
        .send()?;
    if response.status() != StatusCode::FOUND {
        return Err(Box::from("wrong answer from first call"))
    }
    Ok(())
}

fn get_password() -> Result<String, std::io::Error> {
    env::var("NEBOCLI_PASSWORD").or_else(|_| PasswordInput::new().with_prompt("password").interact())
}

fn second_call(client: &Client, sso_url: &str, login: &str)-> Result<String, Box<dyn Error>> {
    let password = get_password()?;
    let response = client
        .post(format!("{}/public/customlogin",sso_url).as_str())
        .form(&[("email", login), ("password", password.as_str())])
        .send()?;
    if response.status() != StatusCode::FOUND {
        return Err(Box::from("wrong answer from second call"))
    }
    Ok(response.headers()
        .get("location")
        .ok_or("Second call: No location found in header")?
        .to_str()
        .unwrap()
        .to_string())
}

fn third_call(client: &Client, sso_url: &str, location: &str)-> Result<String, Box<dyn Error>> {
    let url = Url::parse(location)?;

    let query_params: Vec<_> = url
        .query_pairs()
        .filter(|x| ["client_id", "response_type", "scope", "redirect_uri"].contains(&x.0.deref()))
        .collect();
    
    let response = client
        .get(format!("{}{}",sso_url, url.path()).as_str())
        .query(&query_params)
        .send()?;
    
    if response.status() != StatusCode::FOUND {
        return Err(Box::from("wrong answer from third call"))
    }

    let callback_url = Url::parse(
        response.headers()
            .get("location")
            .ok_or("Second call: No location found in header")?
            .to_str()
            .unwrap()
        )?;

    let fragment = callback_url.fragment().ok_or("No fragment in callback url")?;

    Ok(format!(
        "Bearer {}", 
        match fragment.find("&") {
            None => &fragment["access_token=".len()..],
            Some(index) => &fragment["access_token=".len()..index]
        }
    ))
}

