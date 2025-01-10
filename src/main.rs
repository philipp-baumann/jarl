use air_r_parser::RParserOptions;

use flint::check_ast::*;
use flint::fix::*;
use flint::message::*;

use clap::{arg, Parser};
use rayon::prelude::*;
use std::fs;
use std::path::Path;
use std::time::Instant;
use walkdir::WalkDir;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(
    author,
    name = "flint",
    about = "Flint: Find and Fix Lints in R Code",
    after_help = "For help with a specific command, see: `flint help <command>`."
)]
struct Args {
    #[arg(
        short,
        long,
        default_value = ".",
        help = "The directory in which to check or fix lints."
    )]
    dir: String,
    #[arg(
        short,
        long,
        default_value = "false",
        help = "Automatically fix issues detected by the linter."
    )]
    fix: bool,
}

fn main() {
    let start = Instant::now();
    let args = Args::parse();

    let r_files = WalkDir::new(args.dir)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path().extension() == Some(std::ffi::OsStr::new("R"))
                || e.path().extension() == Some(std::ffi::OsStr::new("r"))
        })
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    // let r_files = vec![Path::new("demo/foo.R").to_path_buf()];

    let parser_options = RParserOptions::default();
    let messages: Vec<Message> = r_files
        .par_iter()
        // TODO: this only ignores files where there was an error, it doesn't
        // return the error messages
        .filter_map(|file| {
            let mut checks: Vec<Message>;
            let mut has_skipped_fixes = true;
            loop {
                let contents = fs::read_to_string(Path::new(file)).expect("Invalid file");
                checks = get_checks(&contents, file, parser_options).unwrap();
                if !has_skipped_fixes || !args.fix {
                    break;
                }
                let (new_has_skipped_fixes, fixed_text) = apply_fixes(&checks, &contents);
                has_skipped_fixes = new_has_skipped_fixes;
                let _ = fs::write(file, fixed_text);
            }
            Some(checks)
        })
        .flatten()
        .collect();

    if !args.fix {
        for message in messages {
            println!("{}", message);
        }
    }
    let duration = start.elapsed();
    println!("Checked files in: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;
    use flint::location::Location;
    use flint::utils::find_new_lines;
    use tempfile::TempDir;

    fn check_string(input: &str) -> anyhow::Result<Vec<Message>> {
        let parser_options = RParserOptions::default();
        let tempdir = TempDir::new()?;
        let temppath = tempdir.path().join("test.R");
        std::fs::write(&temppath, input)?;
        let contents = fs::read_to_string(Path::new(&temppath)).expect("couldn't read file");
        let parsed = air_r_parser::parse(contents.as_str(), parser_options);
        let out = &parsed.syntax();
        let loc_new_lines = find_new_lines(out)?;
        let checks = check_ast(out, &loc_new_lines, temppath.to_str().unwrap());
        Ok(checks)
    }

    #[test]
    fn it_works() -> anyhow::Result<()> {
        let checks = check_string(
            r#"
any(is.na(x))
any(duplicated(x))
a <- 1
b <- T
"#,
        )?;
        let location = Location::new(0, 0);
        assert!(matches!(
            checks.get(0).unwrap(),
            &Message::AnyIsNa { location: _, .. } if location == Location::new(0, 0)
        ));
        let location = Location::new(1, 0);
        assert!(matches!(
            checks.get(1).unwrap(),
            &Message::AnyDuplicated { location: _, .. } if location == Location::new(1, 0)
        ));
        let location = Location::new(2, 0);
        assert!(matches!(
            checks.get(2).unwrap(),
            &Message::TrueFalseSymbol { location: _, .. } if location == Location::new(2, 0)
        ));
        Ok(())
    }
}
