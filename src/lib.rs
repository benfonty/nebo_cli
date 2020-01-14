#[macro_use]
extern crate lazy_static;
extern crate futures;

use std::error::Error;

mod common;
mod token;
mod page;
mod configuration;
mod contacts;

pub fn token(env: &str, login: &str) -> Result<String, Box<dyn Error>> {
    token::token(env, login)
}

pub fn share_page(env: &str, login: &str, uuid: &str, signature: Option<&str>, filename: &str, title: Option<&str>, share_with_myscript: Option<&str>, collect_login: Option<&str>) -> Result<(), Box<dyn Error>> {
    let token = token::token(env, login)?;
    page::share_page(env, &token, uuid, signature, filename, title, share_with_myscript, collect_login)
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