extern crate clap;
extern crate regex;

use clap::{App, Arg, ArgMatches};
use regex::Regex;
use std::io::{self, Read, BufRead, Write, stdin, stdout};
use std::fs::File;
use std::str::from_utf8;
use std::process::exit;

const CAP: usize = 1024; // Initial String capacity


fn main() {
    let mut strs = String::with_capacity(CAP);
    let mut return_code = 0;
    let matches = get_opts();
    // Set the string to join captures
    let default_join = match matches.is_present("envelop") {
        true => ",",
        false => " "
    };
    let joinery = match matches.value_of("join") {
        None => default_join,
        Some(s) => s,
    };
    let matcher = build_regex(matches.is_present("single"),
            matches.is_present("double"));
    let unwrap = matches.is_present("unwrap");
    let envelop = matches.is_present("envelop");
    if envelop {
        strs.push('[')
    };
    match matches.values_of("file") {
        None => {
            let input = stdin();
            let stream = Input::console(&input);
            strs.push_str(&capture_strings(
                    stream, &matcher, joinery, unwrap));
        },
        Some(value) => {
            for filestring in value {
                match Input::file(filestring) {
                    // Replace the below with something sensible. 
                    Err(_) => { println!("No such file or directory");
                                  exit(2) },
                    Ok(file) => strs.push_str(&capture_strings(
                        file, &matcher, joinery, unwrap)),  
                }
            }
        },
    }
    // Remove final join string, only if we've captured anything.
    if strs.len() > joinery.len() { 
        strs = strs[..(strs.len() - joinery.len())].to_string();
    }
    if envelop {
        strs.push(']')
    };
    write_out(strs); 
}

fn write_out(strs: String) {
    let output = stdout();
    let mut handle = output.lock();
    let _ = handle.write(strs.as_bytes());
}

/// Parse the command line arguments.
fn get_opts<'a>() -> ArgMatches<'a> {
    App::new("strs")
        .version("0.1.1 SNAPSHOT")
        .about("\nstrs filters a text stream for quoted strings, \
                passing the result to stdout. \
                Follows C conventions for what a string is.")
        .author("Sam Putman: github/mnemnion")
        .arg(Arg::with_name("envelop")
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
            .required(false)
            .conflicts_with("double"))
        .arg(Arg::with_name("double")
            .help("Capture only \"double quoted\" strings.")
            .long("double")
            .short("d")
            .takes_value(false)
            .required(false))
        .arg(Arg::with_name("unwrap")
            .help("Unwraps the strings from their quote marks. \
                   Does not un-escape sequences.")
            .long("unwrap")
            .short("u")
            .takes_value(false)
            .required(false))
        .arg(Arg::with_name("join")
            .help("Join the strings with <join>. \
                   Defaults to \" \" or with -e to \",\".")
            .long("join")
            .short("j")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("file")
            .help("If specfied, str will read from <file>... \
                   instead of stdin.")
            .takes_value(true)
            .multiple(true))
        .get_matches()
}

fn build_regex(single: bool, double: bool) -> Regex {
    let double_quote = "(?s)\"[^\"\\\\]*(\\\\.[^\"\\\\]*)*\"";
    let single_quote = "(?s)'[^'\\\\]*(\\\\.[^'\\\\]*)*'";
    if single {
        Regex::new(single_quote).unwrap()
    } else if double {
        Regex::new(double_quote).unwrap()
    } else {
        let joint = &format!("{}|{}", single_quote, double_quote);
        Regex::new(joint).unwrap()
    }
}

fn capture_strings(mut stream: Input, matcher: &Regex, 
                   joinery: &str, unwrap: bool) -> String {
    let maybe_utf8 = from_utf8(stream.fill_buf().unwrap());
    let mut captures = String::with_capacity(CAP);
    let phrase = match maybe_utf8 {
        Ok(utf8) => String::from(utf8),
        Err(why) => panic!("Invalid utf-8 in input: {:?}", why),
    };
    for offsets in matcher.find_iter(&phrase) {
        let quote = if unwrap {
            &phrase[offsets.start()+1..offsets.end()-1]
        } else {
            &phrase[offsets.start()..offsets.end()]
        };
        captures.push_str(quote);
        captures.push_str(joinery);
    }
    captures
}

/// Input. Generalizes over files and stdin. 
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