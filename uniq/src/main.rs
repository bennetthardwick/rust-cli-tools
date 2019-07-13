#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg};
use std::io::{self, Write};
use std::mem;

type StringCompare = Box<Fn(&String, &String) -> bool>;

fn compare_offset(start: usize) -> StringCompare {
    Box::new(move |a: &String, b: &String| -> bool {
        if a.len() == b.len() {
            return a
                .as_bytes()
                .iter()
                .skip(start)
                .zip(b.as_bytes().iter().skip(start))
                .all(|(a, b)| a == b);
        }
        false
    })
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new(crate_name!())
        .setting(AppSettings::ColorAuto)
        .setting(AppSettings::ColoredHelp)
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::UnifiedHelpMessage)
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("count")
                .short("c")
                .long("count")
                .takes_value(false)
                .help("prefix lines by the number of occurrences"),
        )
        .arg(
            Arg::with_name("repeated")
                .short("d")
                .long("repeated")
                .takes_value(false)
                .help("only print duplicate lines, one for each group"),
        )
        .arg(
            Arg::with_name("duplicate")
                .short("D")
                .takes_value(false)
                .help("print all duplicate lines"),
        )
        .arg(
            Arg::with_name("all-repeated")
                .long("all-repeated")
                .takes_value(true)
                .value_name("METHOD")
                .possible_values(&["none", "prepend", "separate"])
                .default_value("none")
                .help("like -D, but allow separating groups with an empty line;"),
        )
        .arg(
            Arg::with_name("skip-chars")
                .long("skip-chars")
                .short("s")
                .takes_value(true)
                .value_name("N")
                .default_value("0")
                .help("avoid comparing the first N characters"),
        );

    let matches = app.get_matches_safe()?;

    let stdin = io::stdin();
    let stdout = io::stdout();

    let mut handle = stdout.lock();

    let mut back = String::new();
    let mut front = String::new();

    let mut count = 1;

    let should_count = matches.is_present("count");
    let show_all = !matches.is_present("repeated");

    let compare: StringCompare;

    let skip_count = matches.value_of("skip-chars").unwrap().parse::<usize>()?;

    if skip_count > 0 {
        compare = compare_offset(skip_count);
    } else {
        compare = Box::new(move |a: &String, b: &String| *a == *b);
    }

    let mut size = stdin.read_line(&mut back)?;

    if size == 0 {
        return Ok(());
    }

    loop {
        size = stdin.read_line(&mut front)?;

        if compare(&front, &back) {
            count = count + 1;
        } else {
            if should_count {
                write!(handle, "{} ", count)?;
            }

            if show_all || count > 1 {
                handle.write(back.to_string().as_bytes())?;
            }

            count = 1;

            if size == 0 {
                break;
            }
        }

        mem::swap(&mut back, &mut front);
        front.clear();
    }

    return Ok(());
}

fn main() {
    let result = run();

    if let Err(err) = result {
        if let Some(clap_err) = err.downcast_ref::<clap::Error>() {
            eprint!("{}", clap_err); // Clap errors already have newlines

            match clap_err.kind {
                // The exit code should not indicate an error for --help / --version
                clap::ErrorKind::HelpDisplayed | clap::ErrorKind::VersionDisplayed => {
                    std::process::exit(0)
                }
                _ => (),
            }
        } else {
            eprintln!("Error: {}", err);
        }
        std::process::exit(1);
    }
}
