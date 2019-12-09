use clap::ArgMatches;

pub fn token(arg: &ArgMatches) {
    println!("calling token subcommand with env={} and login={}", arg.value_of("env").unwrap(), arg.value_of("login").unwrap());
}