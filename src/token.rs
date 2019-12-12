use clap::ArgMatches;
use reqwest;
use reqwest::RedirectPolicy;
use reqwest::Client;

use std::error::Error;

use super::common;

pub fn token(arg: &ArgMatches) -> Result<(), Box<dyn Error>> {
    println!("calling token subcommand with env={} and login={}", arg.value_of("env").unwrap(), arg.value_of("login").unwrap());

    let client = reqwest::ClientBuilder::new()
        .redirect(RedirectPolicy::none())
        .build()?;
    
    let env = arg.value_of("env").unwrap();
    let sso_url = common::ENV[env].sso_url;    
    
    first_call(client, sso_url)
} 

fn first_call(client: Client, sso_url: &str) -> Result<(), Box<dyn Error>> {
    let first_response = client
    .get(format!("{}/oauth/authorize",sso_url).as_str())
    .send()?;
    if first_response.status() != 302 {
        return Err(Box::from("wrong answer from first call"))
    }
    Ok(())
}

