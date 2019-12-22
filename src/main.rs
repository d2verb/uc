extern crate clap;
use clap::{App, Arg};

use uc::common;

const VERSION: &'static str = "0.0.1";

fn main() {
    let mut app = App::new("uc")
        .version(VERSION)
        .author("d2verb")
        .about("uc is a compiler for c-like syntax language")
        .arg(Arg::with_name("version")
             .short("v")
             .long("version")
             .help("Show version info"))
        .arg(Arg::with_name("FILE")
             .help("Input file")
             .index(1));

    let matches = app.clone().get_matches();

    if let Some(filename) = matches.value_of("FILE") {
        common::compile(filename);
    } else {
        app.print_help().unwrap();
        println!();
    }
}
