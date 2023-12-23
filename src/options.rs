use std::env;
use std::process::exit;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Default)]
pub struct Options {
    pub filename: Option<String>,
}

const USAGE: &str = "\
Collate or uncollate a series of lines.

Usage:
  collate [options] [<filename>]
  uncollate [options] [<filename>]

Arguments:
  <filename>        The file to read from. When omitted, reads from stdin.

Options:
  -h, --help        Print this help message
  -v, --version     Print the version and exit

Example:
  ❯ cat foo.txt
  1
  2
  3
  a
  b
  c

  ❯ collate < foo.txt  │  ❯ collate < foo.txt | uncollate
  1                    │  1
  a                    │  a
  2                    │  2
  b                    │  b
  3                    │  3
  c                    │  c
";

pub fn parse_options_or_die() -> Options {
    fn die(msg: &str, arg: &str) -> ! {
        eprint!("{} '{}'\n\n{}", msg, arg, USAGE);
        exit(1);
    }

    let mut argv = env::args();

    if argv.next().is_none() {
        eprint!("{}", USAGE);
        exit(1);
    }

    let mut options = Options::default();
    while let Some(arg) = argv.next() {
        if arg.is_empty() {
            die("Unrecognized argument:", &arg);
        }

        if arg == "-h" || arg == "--help" {
            print!("{}", USAGE);
            exit(0);
        }

        if arg == "-v" || arg == "--version" {
            println!("{}", VERSION);
            exit(0);
        }

        if &arg[..1] == "-" {
            die("Unrecognized option:", &arg);
        }

        if options.filename.is_some() {
            die("Extra argument:", &arg);
        }

        options.filename = Some(arg.to_string());
    }

    options
}
