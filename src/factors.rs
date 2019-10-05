extern crate clap;
use std::env;
use std::fmt::Display;
use clap::{Arg, App, SubCommand, ArgSettings, ArgMatches};

fn main() {
    let matches: ArgMatches = App::new("Factors")
        .arg(Arg::with_name("target")
            .value_name("INTEGER")
            .required(true)
            .help("Number to factor")
            .takes_value(true)).get_matches();

    let target: i64 = matches.value_of("target")
        .expect("Number not found")
        .trim()
        .parse()
        .expect("Failed to convert string to integer");
    println!("Finding factors for {}...", target);

    for i in 1 .. (target / 2) + 1 {
        if (target % i == 0) {
            println!("{}", i);
        }
    }
}