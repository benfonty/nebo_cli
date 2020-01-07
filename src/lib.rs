#[macro_use]
extern crate lazy_static;
extern crate futures;

use std::error::Error;

mod common;
mod token;
mod share_page;
mod delete_page;
mod configuration;

pub fn token(env: &str, login: &str) -> Result<String, Box<dyn Error>> {
    token::token(env, login)
}

pub fn share_page(env: &str, login: &str, uuid: &str, signature: Option<&str>, filename: &str, title: Option<&str>, share_with_myscript: Option<&str>, collect_login: Option<&str>) -> Result<(), Box<dyn Error>> {
    let token = token::token(env, login)?;
    share_page::share_page(env, &token, uuid, signature, filename, title, share_with_myscript, collect_login)
} 

pub fn delete_page(env: &str, login: &str, uuid: &str) -> Result<(), Box<dyn Error>> {
    let token = token::token(env, login)?;
    delete_page::delete_page(env, &token, uuid)
} 