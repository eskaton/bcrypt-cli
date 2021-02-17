use std::env;

use clap::{App, AppSettings, Arg, ArgMatches};
use clap::crate_version;

struct Config {
    cost: u32,
    password: String,
}

impl Config {
    fn new(matches: &ArgMatches) -> Config {
        let password = matches.value_of("password").unwrap().to_string();
        let cost = match matches.value_of("cost") {
            Some(c) => c.parse().expect(format!("Invalid cost: {}", c).as_str()),
            None => 10u32
        };

        Config { cost, password }
    }
}

pub fn parse_args(args: &Vec<String>) -> ArgMatches {
    return App::new("BCrypt CLI")
        .version(crate_version!())
        .setting(AppSettings::GlobalVersion)
        .arg(Arg::with_name("cost")
            .help("The cost")
            .short("c")
            .long("cost")
            .takes_value(true))
        .arg(Arg::with_name("password")
            .help("The password to encrypt")
            .required(true)
            .index(1))
        .get_matches_from(args);
}

fn main() {
    let args = env::args().collect();
    let config = Config::new(&parse_args(&args));
    let result = bcrypt::hash(config.password, config.cost);

    println!("{}", result.unwrap());
}
