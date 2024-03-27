mod wordlist;

use crate::wordlist::get_random_element;
use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Charset {
    /// Lowercase letters a-z
    Lower,
    /// Uppercase letters A-Z
    Upper,
    /// Words from the xkcd list, separated by '-'
    Words,
    /// Alphabetic a-z and A-Z
    Alpha,
    /// a-z, A-Z, 0-9
    AlphaNum,
    /// Decimal digits 0-9
    Dec,
    /// Decimal digits 0-9 and letters A-F
    Hex,
}

impl Default for Charset {
    fn default() -> Self {
        Charset::AlphaNum
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Length of the output, measured in characters or words
    length: usize,

    /// The character set to use when generating the random string (default alpha-num)
    #[arg(value_enum)]
    charset: Option<Charset>,
}

fn main() {
    let cli = Cli::parse();

    let n = cli.length;
    let charset = cli.charset.unwrap_or_default();

    for i in 0..n {
        print!("{}", get_random_element(charset));

        if charset == Charset::Words && i != n - 1 {
            print!("-");
        }
    }

    println!()
}
