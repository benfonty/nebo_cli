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
        .subcommand(
            App::new("sharePage")
                .about("share a page")
                .arg(args::env_arg())
                .arg(args::login_arg())
                .arg(args::uuid_arg())
                .arg(args::signature_arg())
                .arg(args::file_arg())
                .arg(args::title_arg())
                .arg(args::share_with_myscript_arg())
                .arg(args::collect_login_arg())
            )
            .subcommand(
                App::new("deletePage")
                    .about("unshare a page")
                    .arg(args::env_arg())
                    .arg(args::login_arg())
                    .arg(args::uuid_arg())
                )
                    
        .get_matches();
    
    match cmd.subcommand() {
        ("token", args) => {
            let unwrapped_args = args.unwrap();
            let token = nebo_cli::token(unwrapped_args.value_of("env").unwrap(), unwrapped_args.value_of("login").unwrap())?;
            println!("{}", token);
            Ok(())
        },
        ("sharePage", args) => {
            let unwrapped_args = args.unwrap();
            nebo_cli::share_page(
                unwrapped_args.value_of("env").unwrap(), 
                unwrapped_args.value_of("login").unwrap(),
                unwrapped_args.value_of("uuid").unwrap(),
                unwrapped_args.value_of("signature"),
                unwrapped_args.value_of("file").unwrap(),
                unwrapped_args.value_of("title"),
                unwrapped_args.value_of("share-with-myscript"),
                unwrapped_args.value_of("collect-login")
            )?;
            Ok(())
        },
        ("deletePage", args) => {
            let unwrapped_args = args.unwrap();
            nebo_cli::delete_page(
                unwrapped_args.value_of("env").unwrap(), 
                unwrapped_args.value_of("login").unwrap(),
                unwrapped_args.value_of("uuid").unwrap(),
            )?;
            Ok(())
        },
        _ => Err(Box::from("please choose a subcommand"))
    }   
}
