#[macro_use]
extern crate lazy_static;
extern crate futures;

use std::error::Error;

mod common;
mod token;
mod share_page;

pub fn token(env: &str, login: &str) -> Result<String, Box<dyn Error>> {
    token::token(env, login)
}

pub fn share_page(env: &str, login: &str, uuid: &str, signature: Option<&str>, filename: &str, title: Option<&str>) -> Result<(), Box<dyn Error>> {
    let token = token::token(env, login)?;
    share_page::share_page(env, &token, uuid, signature, filename, title)
} 