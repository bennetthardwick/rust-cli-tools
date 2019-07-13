use itertools::sorted;
use std::io::{self, Stdin, Write};

struct Reader {
    stdin: Stdin
}

impl Iterator for Reader {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let mut data = String::new();
        match self.stdin.read_line(&mut data) {
            Ok(size) => {
                if size == 0 {
                    return None
                } else {
                    return Some(data);
                }
            }
            _ => return None
        }
    }
}

fn main() {
    let r = Reader { stdin: io::stdin() };
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for l in sorted(r) {
        write!(handle, "{}", l).unwrap();
    }

}
