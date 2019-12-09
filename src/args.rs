use clap::Arg;

pub fn env_arg<'a>() -> Arg<'a, 'a> {
    Arg::with_name("env")
        .long("env")
        .value_name("env")
        .help("the env")
        .takes_value(true)
        .required(true)
        .possible_values(&["local", "cloudtest", "cloudtest2", "prod"])
        .env("NEBOCLI_ENV")
}

pub fn login_arg<'a>()-> Arg<'a, 'a> {
    Arg::with_name("login")
        .long("login")
        .value_name("login")
        .help("the login")
        .takes_value(true)
        .required(true)
        .env("NEBOCLI_LOGIN")
}