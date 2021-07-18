mod calculator;

use crate::calculator::rpn::RpnCalculator;

use anyhow::Result;
use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;

#[derive(Clap, Debug)]
#[clap(
    name = "My RPN",
    version = "0.1.0",
    author = "sdaigo",
    about = "awesome rpn"
)]
struct Opts {
    #[clap(short, long)]
    verbose: bool,

    #[clap(name = "FILE")]
    formula_file: Option<PathBuf>,
}

fn run<R: BufRead>(reader: R, verbose: bool) -> Result<()> {
    let calc = RpnCalculator::new(verbose);

    for line in reader.lines() {
        let line = line?;

        match calc.eval(&line) {
            Ok(answer) => println!("{}", answer),
            Err(e) => eprintln!("{:#?}", e),
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    if let Some(path) = opts.formula_file {
        let f = File::open(path).unwrap();
        let reader = BufReader::new(f);
        run(reader, opts.verbose)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        run(reader, opts.verbose)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let calculator = RpnCalculator::new(false);

        assert_eq!(calculator.eval("5"), 5);
        assert_eq!(calculator.eval("50"), 50);
        assert_eq!(calculator.eval("-50"), -50);

        assert_eq!(calculator.eval("2 3 +"), 5);
        assert_eq!(calculator.eval("2 3 *"), 6);
        assert_eq!(calculator.eval("2 3 -"), -1);
        assert_eq!(calculator.eval("2 3 /"), 0);
        assert_eq!(calculator.eval("2 3 %"), 2);
    }

    #[test]
    #[should_panic]
    fn test_ng() {
        let calculator = RpnCalculator::new(false);

        calculator.eval("1 1 ^");
    }
}
