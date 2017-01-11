extern crate clap;

use clap::{App, Arg};


fn main() {
        App::new("strs")
            .version("0.0.1 SNAPSHOT")
            .about("\nstrs filters a text stream for quoted strings. Follows C conventions for what a string is.")
            .author("Sam Putman: github/mnemnion")
            .arg(Arg::with_name("env")
            	.help("Wraps the strings as [a,b,c]. Joins with ',' by default, may be overridden with -j.")
            	.long("envelop")
            	.short("e")
            	.takes_value(false)
            	.required(false))
            .arg(Arg::with_name("join")
                .help("Join the strings with <join>. Defaults to \" \" or \",\" with -e.")
                .long("join")
                .short("j")
                .takes_value(true)
                .required(false))
            .get_matches();
    }