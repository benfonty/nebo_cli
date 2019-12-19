use clap::ArgMatches;
use reqwest;
use reqwest::RedirectPolicy;
use reqwest::Client;

use std::error::Error;
use dialoguer::PasswordInput;
use std::env;

use super::common;

pub fn token(arg: &ArgMatches) -> Result<(), Box<dyn Error>> {
    println!("calling token subcommand with env={} and login={}", arg.value_of("env").unwrap(), arg.value_of("login").unwrap());

    let client = reqwest::ClientBuilder::new()
        .redirect(RedirectPolicy::none())
        .build()?;
    
    let env = arg.value_of("env").unwrap();
    let login = arg.value_of("login").unwrap();
    let sso_url = common::ENV[env].sso_url;    
    
    first_call(&client, env, sso_url)?;
    let location = second_call(&client, sso_url, login)?;
    println!("location = {}", location);
    Ok(())
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
    if response.status() != 302 {
        return Err(Box::from("wrong answer from first call"))
    }
    Ok(())
}

fn get_password() -> Result<(String), std::io::Error> {
    env::var("NEBOCLI_PASSWORD").or_else(|_| PasswordInput::new().with_prompt("password").interact())
}

fn second_call(client: &Client, sso_url: &str, login: &str)-> Result<(String), Box<dyn Error>> {
    let password = get_password()?;
    let response = client
        .post(format!("{}/public/customlogin",sso_url).as_str())
        .form(&[("email", login), ("password", password.as_str())])
        .send()?;
    if response.status() != 302 {
        return Err(Box::from("wrong answer from first call"))
    }
    
    let resp = response.headers()
        .get("location")
        .ok_or("Second call: No location found in header")?
        .to_str()
        .unwrap();
    Ok(resp.to_string())
}

