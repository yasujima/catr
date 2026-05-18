use std::error::Error;
use clap::{App, Arg};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
	"-" => Ok(Box::new(BufReader::new(io::stdin()))),
	_ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
	match open(&filename) {
	    Err(err) => eprintln!("Failed to open {}: {}", filename, err),
	    Ok(c) => {
		let mut last_num = 0;
		for (num, line) in c.lines().enumerate() {
		    let line = line?;
		    if config.number_lines {
			println!("{:>6}\t{}", num + 1, line);
		    } else if config.number_nonblank {
			if !line.is_empty() {
			    last_num += 1;
			    println!("{:>6}\t{}", last_num, line);
			} else {
			    println!();
			}
		    } else {
			println!("{}", line);
		    }
		}
	    },
	}
    }
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
	.version("0.1.0")
	.author("yasujima<yasujima@gmail.com>")
	.about("Rust cat")
        .template("\
{bin} {version}
{about}

Usage: {usage}

FLAGS:
{flags}
OPTIONS:
{options}
ARGS:
{positionals}")
	.arg(
	    Arg::with_name("file")
		.value_name("FILE")
		.help("Input file(s)")
		.default_value("-")
		.multiple(true),
	    
	)
	.arg(
	    Arg::with_name("number")
		.short("n")
		.long("number")
		.help("Print line number")
		.takes_value(false),
	)
	.arg(
	    Arg::with_name("number-nonblank")
		.short("b")
		.long("number-nonblank")
		.conflicts_with("number")
		.help("Print line number non blank")
		.takes_value(false),
	)
	.get_matches();

    let files = matches.values_of_lossy("file").unwrap();
    let number_lines = matches.is_present("number");
    let number_nonblank = matches.is_present("number-nonblank");

    Ok(Config {
	files: files,
	number_lines: number_lines,
	number_nonblank: number_nonblank,
    })
}
