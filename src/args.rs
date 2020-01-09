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

pub fn verbose_arg<'a>() -> Arg<'a, 'a> {
    Arg::with_name("v")
        .short("v")
        .value_name("v")
        .takes_value(false)
        .required(false)
        .multiple(true)
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

pub fn share_with_myscript_arg<'a>()-> Arg<'a, 'a> {
    Arg::with_name("share-with-myscript")
        .long("share-with-myscript")
        .value_name("share-with-myscript")
        .help("the login")
        .takes_value(true)
        .env("NEBOCLI_SHARE_WITH_MYSCRIPT")
}

pub fn collect_login_arg<'a>()-> Arg<'a, 'a> {
    Arg::with_name("collect-login")
        .long("collect-login")
        .value_name("collect-login")
        .help("the collect login")
        .takes_value(true)
        .env("NEBOCLI_COLLECT_LOGIN")
}

pub fn email_arg<'a>()-> Arg<'a, 'a> {
    Arg::with_name("email")
        .long("email")
        .value_name("email")
        .help("the email")
        .takes_value(true)
        .required(true)
        .env("NEBOCLI_EMAIL")
}

pub fn name_arg<'a>()-> Arg<'a, 'a> {
    Arg::with_name("name")
        .long("name")
        .value_name("name")
        .help("the name")
        .takes_value(true)
        .required(false)
        .env("NEBOCLI_NAME")
}

pub fn message_arg<'a>()-> Arg<'a, 'a> {
    Arg::with_name("message")
        .long("message")
        .value_name("message")
        .help("the message")
        .takes_value(true)
        .required(false)
        .env("NEBOCLI_MESSAGE")
}