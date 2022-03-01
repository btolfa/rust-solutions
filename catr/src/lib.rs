use std::error::Error;
use clap::{Arg, Command};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Tengiz Sharafiev <b@g.c>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Concat FILE(s) to standard output")
                .multiple_values(true)
                .default_value("-")
                .hide_default_value(true)
                .allow_invalid_utf8(true)
        )
        .arg(Arg::new("number_lines")
            .help("number all output lines")
            .short('n')
            .long("number")
            .takes_value(false)
        )
        .arg(Arg::new("number_nonblank")
            .help("number nonempty output lines")
            .short('b')
            .long("number-nonblank")
            .conflicts_with("number_lines")
            .takes_value(false)
        )
        .get_matches();

    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank")
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(buf_reader) => match (config.number_lines, config.number_nonblank_lines) {
                (true, false) => print_line_by_line_with_number(buf_reader)?,
                (false, true) => print_line_by_line_with_number_skip_empty(buf_reader)?,
                _ => print_line_by_line(buf_reader)?
            }

        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?)))
    }
}

fn print_line_by_line(reader: Box<dyn BufRead>) -> MyResult<()> {
    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}

fn print_line_by_line_with_number(reader: Box<dyn BufRead>) -> MyResult<()> {
    for (idx, line) in reader.lines().enumerate() {
        println!("{:>6}\t{}", idx + 1, line?);
    }
    Ok(())
}

fn print_line_by_line_with_number_skip_empty(reader:Box<dyn BufRead>) -> MyResult<()> {
    let mut idx = 1 as usize;
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            println!();
        } else {
            println!("{:>6}\t{}", idx, line);
            idx += 1;
        }
    }
    Ok(())
}