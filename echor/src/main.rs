use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Tengiz Sharafiev <b@g.c>")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1)
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .takes_value(false)
        )
        .get_matches();

    let text = matches.values_of("text").unwrap().collect::<Vec<&str>>().join(" ");
    let ending = if matches.is_present("omit_newline") { "" } else {"\n" };
    print!("{}{}", text, ending);
}
