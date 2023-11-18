mod config;
mod generator;

use clap::{App, Arg, SubCommand};
use config::Config;
use generator::generate_random_number;

fn main() {
    let app = App::new("Random Number Generator")
        .version("1.0")
        .author("Your Name")
        .about("Generates a random number within a specified range")
        .arg(
            Arg::with_name("min")
                .short('m')
                .long("min")
                .global(true) // Make this argument global
                .takes_value(true)
                .help("Minimum value of the range"),
        )
        .arg(
            Arg::with_name("max")
                .short('M')
                .long("max")
                .global(true) // Make this argument global
                .takes_value(true)
                .help("Maximum value of the range"),
        )
        .arg(
            Arg::with_name("float")
                .short('f')
                .long("float")
                .takes_value(false) // Make this argument global
                .help("Generate a floating-point number (only for 'genum', not subcommands)"),
        )
        .subcommand(SubCommand::with_name("odd").about("Generates a random odd number"))
        .subcommand(SubCommand::with_name("even").about("Generates a random even number"))
        .subcommand(SubCommand::with_name("prime").about("Generates a random prime number"));

    let matches = app.get_matches();

    let config = Config::new(&matches);

    match generate_random_number(&config) {
        Ok(number) => println!("Random Number: {}", number),
        Err(e) => eprintln!("Error: {}", e),
    }
}
