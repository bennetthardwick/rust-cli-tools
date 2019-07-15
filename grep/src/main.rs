use regex::Regex;
use std::env;
use std::io::{self, BufRead, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = &args[1];

    let re = Regex::new(&format!("(?P<r>{})", &pattern)).unwrap();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let lines = stdin.lock().lines().map(|l| l.unwrap());

    write!(
        handle,
        "{}",
        lines
            .filter(|l| re.is_match(&l))
            .map(|l| String::from(re.replace_all(&l, "\x1b[0;31;1)m$r\x1b[0m" )) )
            .collect::<String>()
    )
    .unwrap();
}
