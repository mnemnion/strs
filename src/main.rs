extern crate clap;

use clap::{App, Arg, ArgMatches};

fn getOpts<'a>() -> ArgMatches<'a> {
    App::new("strs")
            .version("0.0.1 SNAPSHOT")
            .about("\nstrs filters a text stream for quoted strings, passing the result to stdout. \
                   Follows C conventions for what a string is.")
            .author("Sam Putman: github/mnemnion")
            .arg(Arg::with_name("env")
                .help("Wraps the strings as [a,b,c]. Joins with ',' by default. May be overridden with -j.")
                .long("envelop")
                .short("e")
                .takes_value(false)
                .required(false))
            .arg(Arg::with_name("single")
                .help("Capture only 'single quoted' strings.")
                .long("single")
                .short("s")
                .takes_value(false)
                .required(false))
            .arg(Arg::with_name("double")
                .help("Capture only \"double quoted\" strings.")
                .long("double")
                .short("d")
                .takes_value(false)
                .required(false))
            .arg(Arg::with_name("unwrap")
                .help("Unwraps the strings from their quote marks. Does not un-escape sequences.")
                .long("unwrap")
                .short("u")
                .takes_value(false)
                .required(false))
            .arg(Arg::with_name("join")
                .help("Join the strings with <join>. Defaults to \" \" or \",\" with -e.")
                .long("join")
                .short("j")
                .takes_value(true)
                .required(false))
            .arg(Arg::with_name("file")
                .help("If specfied, str will read from <file>... instead of stdin.")
                .takes_value(true)
                .multiple(true))
            .get_matches()
}


fn main() {
    let matches = getOpts();
    }