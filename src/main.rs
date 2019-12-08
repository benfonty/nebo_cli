use clap::{App, Arg};



fn main() {
    let matches = App::new("nebo_cli")
        .version("1.0")
        .author("benoît F")
        .about("rust version of nebo_cli")
        .arg(Arg::with_name("env")
                .long("env")
                .value_name("env")
                .help("the env")
                .takes_value(true)
                .required(true)
                .possible_values(&["local", "cloudtest", "cloudtest2", "prod"])
            )
        .subcommand(
            App::new("token")
                .about("get the jwt token of a user")
                    )
        .get_matches();
    
        println!("{}", matches.subcommand_name().unwrap());
        println!("{}", matches.value_of("env").unwrap())
}
