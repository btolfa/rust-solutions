use clap::{Command, Arg};
use std::error::Error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

type MyResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Tengiz Sharafiev <b@g.c>")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("display first lines of a file")
                .multiple_values(true)
                .default_value("-")
                .hide_default_value(true)
                .allow_invalid_utf8(true)
        )
        .arg(Arg::new("lines")
            .value_name("LINES")
            .help("print the K lines instead of the first 10")
            .short('n')
            .long("lines")
            .takes_value(true)
            .default_value("10")
            .allow_invalid_utf8(true)
        )
        .arg(Arg::new("bytes")
            .help("print the first K bytes of each file")
            .short('c')
            .value_name("BYTES")
            .long("bytes")
            .conflicts_with("lines")
            .takes_value(true)
            .allow_invalid_utf8(true)
        )
        .get_matches();

    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        lines: parse_positive_int(&matches.value_of_lossy("lines").unwrap())
            .map_err(|err|format!("illegal line count -- {}", err))?,
        bytes: matches.value_of_lossy("bytes")
            .map(|x| parse_positive_int(&x))
            .transpose()
            .map_err(|err|format!("illegal byte count -- {}", err))?,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for (idx, filename) in config.files.iter().enumerate() {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(mut reader) => {
                if config.files.len() > 1 {
                    println!("{}==> {} <==", if idx > 0 {"\n"} else {""}, &filename);
                }
                if let Some(bytes) = config.bytes {
                    process_bytes(&mut reader, bytes)?;
                } else {
                    process_lines(&mut reader, config.lines)?;
                }
            }
        }
    }
    Ok(())
}

fn process_bytes(reader:&mut Box<dyn BufRead>, bytes: usize) -> MyResult<()> {
    let mut handler = reader.take(bytes as u64);
    let mut buffer = vec![0u8; bytes];
    let bytes = handler.read(&mut buffer)?;

    print!("{}", String::from_utf8_lossy(&buffer[..bytes]));
    Ok(())
}

fn process_lines(reader:&mut Box<dyn BufRead>, lines: usize) -> MyResult<()> {
    let mut line = String::new();
    for _ in 0..lines {
        match reader.read_line(&mut line)? {
            0 => return Ok(()),
            _ => print!("{}", line),
        };
    }
    Ok(())
}

fn parse_positive_int(value: &str) -> MyResult<usize> {
    match value.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(value)),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

#[test]
fn test_parse_positive_int() {
    // "3" is Ok
    let res = parse_positive_int("3");
    assert!(res.is_ok());
    assert_eq!(res.unwrap(), 3);

    // Any other string is an error
    let res = parse_positive_int("hello");
    assert!(res.is_err());

    // Zero is an error
    let res = parse_positive_int("0");
    assert!(res.is_err());
}