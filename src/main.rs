use std::error::Error;

use clap::{App};
use ::nebo_cli;
mod args;


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
            let unwrapped_args = args.unwrap();
            nebo_cli::token(unwrapped_args.value_of("env").unwrap(), unwrapped_args.value_of("login").unwrap())
        },
        _ => Err(Box::from("no match"))
    }   
}
