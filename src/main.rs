#[macro_use]
extern crate lazy_static;
extern crate futures;
use std::error::Error;

use clap::{App};
mod token;
mod args;
mod common;

fn main() -> Result<(), Box<dyn Error>>{
    let cmd = App::new("nebo_cli")
        .version("1.0")
        .author("benoît F")
        .about("rust version of nebo_cli")
        .subcommand(
            App::new("token")
                .about("get the jwt token of a user")
                .arg(args::env_arg())
                .arg(args::login_arg())
                    )
        .get_matches();
    
    match cmd.subcommand() {
        ("token", args) => {
            token::token(args.unwrap())
        },
        _ => Err(Box::from("no match"))
    }   
}
