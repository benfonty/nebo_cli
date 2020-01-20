#[macro_use]
extern crate lazy_static;
extern crate futures;

use std::error::Error;
use std::path::Path;

mod common;
mod token;
mod page;
mod configuration;
mod contacts;

use log::{info, error};
use configuration::Configuration;

use rusoto_core::credential::StaticProvider;

use threadpool::ThreadPool;

pub fn token(env: &str, login: &str) -> Result<String, Box<dyn Error>> {
    token::token(env, login)
}

pub fn share_page(
    env: &str, 
    login: &str, 
    uuid: &str, 
    signature: Option<&str>, 
    filename: &str, 
    title: Option<&str>, 
    share_with_myscript: Option<&str>, 
    collect_login: Option<&str>,
    email: Option<&str>) -> Result<(), Box<dyn Error>> {
    let token = token::token(env, login)?;
    let configuration = Configuration::get(env, &common::get_default_client(&token))?;
    let provider = StaticProvider::from(
        page::aws::get_cognito_credentials(
            &configuration.credentials.access_token, 
            &configuration.credentials.identity_id,
            &configuration.credentials.identity_pool_id,
            &configuration.credentials.identity_provider,
            &configuration.credentials.region
        )?
    );
    page::share_page(env, &token, uuid, signature, filename, title, share_with_myscript, collect_login, provider, &configuration)?;
    if let Some(email) = email {
        page::make_private(env, &token, uuid)?;
        contacts::add_contact(env, &token, uuid, &email, None, None)?;
    }
    Ok(())
}

pub fn share_pages(env: &str, login: &str, dir: &str) -> Result<(), Box<dyn Error>> {
    info!("Sharing pages of directory {}", dir);
    let files = common::scan_dir(dir)?;
    info!("Found {} files to share", files.len());
    let token = token::token(env, login)?;
    let configuration = Configuration::get(env, &common::get_default_client(&token))?;
    let provider = StaticProvider::from(
        page::aws::get_cognito_credentials(
            &configuration.credentials.access_token, 
            &configuration.credentials.identity_id,
            &configuration.credentials.identity_pool_id,
            &configuration.credentials.identity_provider,
            &configuration.credentials.region
        )?
    );
    let pool = ThreadPool::with_name("sharepages".into(), page::NB_THREADS_SHAREPAGES);
    for file in files {
        let (env, token, uuid, configuration, provider) = (
            env.to_owned(),
            token.clone(),
            Path::new(&file).file_name().unwrap().to_str().unwrap().split('.').next().unwrap().to_owned(),
            configuration.clone(),
            provider.clone()
    );
        pool.execute(move || {
            if let Err(e) = page::share_page(&env, &token, &uuid, None, &file, None, None, None, provider, &configuration) {
                error!("Sharing file {} KO ({})", file, e);
            }
        })
    }
    pool.join();
    
    Ok(())
} 

pub fn delete_page(env: &str, login: &str, uuid: &str) -> Result<(), Box<dyn Error>> {
    let token = token::token(env, login)?;
    page::delete_page(env, &token, uuid)
} 

pub fn add_contact(env: &str, login: &str, uuid: &str, email: &str, name: Option<&str>, message: Option<&str>) -> Result<(), Box<dyn Error>> {
    let token = token::token(env, login)?;
    contacts::add_contact(env, &token, uuid, email, name, message)
} 

pub fn remove_contact(env: &str, login: &str, uuid: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let token = token::token(env, login)?;
    contacts::remove_contact(env, &token, uuid, email)
} 

pub fn delete_pages(env: &str, login: &str) -> Result<(), Box<dyn Error>> {
    let token = token::token(env, login)?;
    page::delete_pages(env, &token, login)
} 