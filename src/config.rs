use clap::ArgMatches;

pub enum NumberType {
    Any,
    Odd,
    Even,
    Prime,
}

pub struct Config {
    pub min: f64,
    pub max: f64,
    pub is_float: bool,
    pub number_type: NumberType,
}

impl Config {
    pub fn new(matches: &ArgMatches) -> Config {
        let min = matches
            .value_of("min")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or(0.0);
        let max = matches
            .value_of("max")
            .and_then(|v| v.parse::<f64>().ok())
            .unwrap_or_else(|| if min > 0.0 { min + 10.0 } else { 100.0 });
        let is_float = matches.is_present("float");

        let number_type = if matches.subcommand_matches("prime").is_some() {
            NumberType::Prime
        } else if matches.subcommand_matches("odd").is_some() {
            NumberType::Odd
        } else if matches.subcommand_matches("even").is_some() {
            NumberType::Even
        } else {
            NumberType::Any
        };

        Config {
            min,
            max,
            is_float,
            number_type,
        }
    }
}
