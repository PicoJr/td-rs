use clap::{App, Arg};
use std::str::FromStr;

pub struct Config {
    pub(crate) interactive: bool,
    pub(crate) towers: usize,
    pub(crate) units: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            interactive: false,
            towers: 10,
            units: 10,
        }
    }
}

pub fn get_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Does awesome things")
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .takes_value(false)
                .required(false)
                .help("prompt between simulation steps"),
        )
        .arg(
            Arg::with_name("towers")
                .long("towers")
                .takes_value(true)
                .default_value("10"),
        )
        .arg(
            Arg::with_name("units")
                .long("units")
                .takes_value(true)
                .default_value("10"),
        )
}

pub fn get_config() -> anyhow::Result<Config> {
    let matches = get_app().get_matches();
    let interactive = matches.is_present("interactive");
    let towers = match matches.value_of("towers") {
        Some(t) => usize::from_str(t)?,
        None => Config::default().towers,
    };
    let units = match matches.value_of("units") {
        Some(u) => usize::from_str(u)?,
        None => Config::default().units,
    };
    Ok(Config {
        interactive,
        towers,
        units,
    })
}
