extern crate clap;

use clap::{App, Arg, ArgMatches};
use std::io::{self, Read, BufRead, Write, stdin, stdout};
use std::fs::File;

struct Input<'a> {
    source: Box<BufRead + 'a>
}

impl<'a> Input<'a> {
    fn console(stdin: &'a io::Stdin) -> Input<'a> {
        Input { source: Box::new(stdin.lock()) }
    }

    fn file(path: &str) -> io::Result<Input<'a>> {
        File::open(path)
            .map(|file| Input { source: Box::new(io::BufReader::new(file)) })
    }
}

impl<'a> Read for Input<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.source.read(buf)
    }
}

impl<'a> BufRead for Input<'a> {
    fn fill_buf(&mut self) -> io::Result<&[u8]> {
        self.source.fill_buf()
    }
    fn consume(&mut self, amt: usize) {
        self.source.consume(amt);
    }
}


/// Parse the command line arguments.
fn getOpts<'a>() -> ArgMatches<'a> {
    App::new("strs")
            .version("0.1.0 SNAPSHOT")
            .about("\nstrs filters a text stream for quoted strings, passing the result to stdout. \
                   Follows C conventions for what a string is.")
            .author("Sam Putman: github/mnemnion")
            .arg(Arg::with_name("env")
                .help("Wraps the strings as [str1,...,strN]. \
                       Joins with ',' by default. \
                       May be overridden with -j.")
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
    let input = stdin();
    let output = stdout();
    let mut stream = Input::console(&input); 
    let mut handle = output.lock();
    handle.write(stream.fill_buf().unwrap());
    }