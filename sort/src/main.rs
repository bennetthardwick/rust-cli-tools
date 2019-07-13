use itertools::sorted;
use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let lines = stdin.lock().lines().map(|l| l.unwrap());
    for l in sorted(lines) {
        write!(handle, "{}", l).unwrap();
    }
}
