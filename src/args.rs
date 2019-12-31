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

pub fn uuid_arg<'a>()-> Arg<'a, 'a> {
    Arg::with_name("uuid")
        .long("uuid")
        .value_name("uuid")
        .help("the uuid of the page")
        .takes_value(true)
        .required(true)
        .env("NEBOCLI_UUID")
}

pub fn signature_arg<'a>()-> Arg<'a, 'a> {
    Arg::with_name("signature")
        .long("signature")
        .value_name("signature")
        .help("the signature")
        .takes_value(true)
        .required(false)
        .env("NEBOCLI_SIGNATURE")
}

pub fn file_arg<'a>()-> Arg<'a, 'a> {
    Arg::with_name("file")
        .long("file")
        .value_name("file")
        .help("the file")
        .takes_value(true)
        .required(true)
        .env("NEBOCLI_FILE")
}

pub fn title_arg<'a>()-> Arg<'a, 'a> {
    Arg::with_name("title")
        .long("title")
        .value_name("title")
        .help("the title")
        .takes_value(true)
        .required(false)
        .env("NEBOCLI_TITLE")
}