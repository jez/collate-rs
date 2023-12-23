extern crate atty;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub mod options;

fn collate<T: BufRead>(input: &mut T) -> () {
    let lines: Vec<String> = input.lines().filter_map(Result::ok).collect();

    for line in lines.iter().step_by(2) {
        println!("{}", line);
    }
    for line in lines.iter().skip(1).step_by(2) {
        println!("{}", line);
    }
    ()
}

fn main() -> io::Result<()> {
    let options = options::parse_options_or_die();

    match &options.filename {
        None => {
            if atty::is(atty::Stream::Stdin) {
                eprintln!("Warning: reading from stdin, which is a tty.");
            }
            collate(&mut io::stdin().lock())
        }
        Some(filename) => {
            let file = File::open(filename)?;
            let mut reader = BufReader::new(file);
            collate(&mut reader)
        }
    };

    Ok(())
}
