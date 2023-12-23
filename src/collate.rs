extern crate atty;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub mod options;

fn collate<T: BufRead>(input: &mut T) -> () {
    let lines: Vec<String> = input.lines().filter_map(Result::ok).collect();
    let mid = (lines.len() + 1) / 2;
    let top_half = lines.iter().take(mid);
    let bot_half = lines.iter().skip(mid);
    for (top_line, bot_line) in top_half.zip(bot_half) {
        println!("{}", top_line);
        println!("{}", bot_line);
    }
    if !lines.is_empty() && lines.len() % 2 != 0 {
        println!("{}", lines[lines.len() / 2]);
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
