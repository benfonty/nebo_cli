#[macro_use]
extern crate lazy_static;
extern crate futures;

use std::error::Error;

mod common;
mod token;

pub fn token(env: &str, login: &str) -> Result<(), Box<dyn Error>> {
    token::token(env, login)
}