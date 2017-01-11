extern crate clap;

use clap::{App};


fn main() {
        App::new("strs")
            .version("0.0.1 SNAPSHOT")
            .about("\nstrs filters a text stream for quoted strings. Follows C conventions for what a string is.")
            .author("Sam Putman: github/mnemnion")
            .get_matches();
    }