use clap::{ App, Arg, ArgMatches };
use std::error::Error;
use std::fs::File;
use std::io::{ BufRead, BufReader };
use std::path::Path;

pub struct Params {
    pub dict: Vec<String>,
    pub word: String,
}

pub fn read_args() -> Result<Params, String> {
    let matches = load_matches();

    // Return early if we didn't get a word to look for
    let word = match matches.value_of("word") {
        Some(word) => word,
        None => return Err("Word not provided".to_string()),
    };

    // Return early if we didn't get a dictionary
    let dict = match matches.value_of("dict").map(|p| File::open(&Path::new(p))) {
        Some(Ok(file)) => file,
        Some(Err(e)) => return Err(format!("Unable to open file: {}", e.description())),
        None => return Err("Dictionary not provided".to_string()),
    };

    let sort = matches.is_present("sort");
    let dedup = matches.is_present("dedup");

    Ok(Params { dict: load_file(BufReader::new(dict), sort, dedup), word: word.to_string() })
}

fn load_file<R: BufRead>(r: R, sort: bool, dedup: bool) -> Vec<String> {
    let mut vec: Vec<String> = r.lines()
        .filter_map(|l| l.map(|l| l.trim().to_string()).ok())
        .collect();

    if sort { vec.sort(); }
    if dedup { vec.dedup(); }

    vec
}

fn load_matches<'a>() -> ArgMatches<'a> {
    App::new("correct")
        .version("0.0.1")
        .author("J/A <archer884@gmail.com>")
        .about("Provides spelling suggestions")

        .arg(Arg::new("word")
            .help("Word to correct")
            .required(true)
            .index(1))

        .arg(Arg::new("dict")
            .help("Dictionary input file")
            .required(true)
            .index(2))

        .arg(Arg::new("sort")
            .short("s")
            .long("sort")
            .help("Sort dictionary")
            .takes_value(false))

        .arg(Arg::new("dedup")
            .short("d")
            .long("dedup")
            .help("Deduplicate dictionary")
            .takes_value(false))

        .get_matches()
}
