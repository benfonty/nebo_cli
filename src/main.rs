use std::error::Error;

use clap::{App};
use ::nebo_cli;

mod logs;
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
                .arg(args::verbose_arg())
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
                .arg(args::verbose_arg())
            )
            .subcommand(
                App::new("sharePages")
                    .about("share pages from directory")
                    .arg(args::env_arg())
                    .arg(args::login_arg())
                    .arg(args::dir_arg())
                    .arg(args::verbose_arg())
                ).subcommand(
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
                .arg(args::verbose_arg())
            )
        .subcommand(
            App::new("deletePage")
                .about("unshare a page")
                .arg(args::env_arg())
                .arg(args::login_arg())
                .arg(args::uuid_arg())
                .arg(args::verbose_arg())
            )
        .subcommand(
                App::new("deletePages")
                    .about("delete all the pages of a user")
                    .arg(args::env_arg())
                    .arg(args::login_arg())
                    .arg(args::verbose_arg())
            )
        .subcommand(
            App::new("addContact")
                .about("add a contact")
                .arg(args::uuid_arg())
                .arg(args::env_arg())
                .arg(args::login_arg())
                .arg(args::email_arg())
                .arg(args::message_arg())
                .arg(args::name_arg())
                .arg(args::verbose_arg())
            )
        .subcommand(
            App::new("removeContact")
                .about("remove a contact")
                .arg(args::uuid_arg())
                .arg(args::env_arg())
                .arg(args::login_arg())
                .arg(args::email_arg())
                .arg(args::verbose_arg())
            )
                    
        .get_matches();
    
    match cmd.subcommand() {
        ("token", args) => {
            let unwrapped_args = args.unwrap();
            logs::init(unwrapped_args.occurrences_of("v"));
            let token = nebo_cli::token(unwrapped_args.value_of("env").unwrap(), unwrapped_args.value_of("login").unwrap())?;
            println!("{}", token);
            Ok(())
        },
        ("sharePage", args) => {
            let unwrapped_args = args.unwrap();
            logs::init(unwrapped_args.occurrences_of("v"));
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
        ("sharePages", args) => {
            let unwrapped_args = args.unwrap();
            logs::init(unwrapped_args.occurrences_of("v"));
            nebo_cli::share_pages(
                unwrapped_args.value_of("env").unwrap(), 
                unwrapped_args.value_of("login").unwrap(),
                unwrapped_args.value_of("dir").unwrap()
            )?;
            Ok(())
        },
        ("deletePage", args) => {
            let unwrapped_args = args.unwrap();
            logs::init(unwrapped_args.occurrences_of("v"));
            nebo_cli::delete_page(
                unwrapped_args.value_of("env").unwrap(), 
                unwrapped_args.value_of("login").unwrap(),
                unwrapped_args.value_of("uuid").unwrap(),
            )?;
            Ok(())
        },
        ("deletePages", args) => {
            let unwrapped_args = args.unwrap();
            logs::init(unwrapped_args.occurrences_of("v"));
            nebo_cli::delete_pages(
                unwrapped_args.value_of("env").unwrap(), 
                unwrapped_args.value_of("login").unwrap()
            )?;
            Ok(())
        },
        ("addContact", args) => {
            let unwrapped_args = args.unwrap();
            logs::init(unwrapped_args.occurrences_of("v"));
            nebo_cli::add_contact(
                unwrapped_args.value_of("env").unwrap(), 
                unwrapped_args.value_of("login").unwrap(),
                unwrapped_args.value_of("uuid").unwrap(),
                unwrapped_args.value_of("email").unwrap(),
                unwrapped_args.value_of("name"),
                unwrapped_args.value_of("message"),
            )?;
            Ok(())
        },
        ("removeContact", args) => {
            let unwrapped_args = args.unwrap();
            logs::init(unwrapped_args.occurrences_of("v"));
            nebo_cli::remove_contact(
                unwrapped_args.value_of("env").unwrap(), 
                unwrapped_args.value_of("login").unwrap(),
                unwrapped_args.value_of("uuid").unwrap(),
                unwrapped_args.value_of("email").unwrap()
            )?;
            Ok(())
        },
        _ => Err(Box::from("please choose a subcommand"))
    }   
}
