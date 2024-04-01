use anyhow::{Context, Result};
use clap::Parser;
use std::io::{self, Write};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    find_matches(&content, &args.pattern, &mut std::io::stdout())?;

    Ok(())
}

fn find_matches(content: &str, pattern: &str, mut writer: impl Write) -> io::Result<()> {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line)?;
        }
    }
    Ok(())
}
#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result).unwrap();
    assert_eq!(result, b"lorem ipsum\n");
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = assert_cmd::Command::cargo_bin("rust_cli")?;

    cmd.arg("foobar").arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicates::prelude::predicate::str::contains("could not read file"));

    Ok(())
}

//use assert_fs::prelude::*;
//use assert_fs::fixture::FileWriteStr;
// use predicates::prelude::*;
use assert_fs::fixture::FileWriteStr;
#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("sample.txt")?;
    file.write_str("A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = assert_cmd::Command::cargo_bin("rust_cli")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicates::prelude::predicate::str::contains("A test\nAnother test"));

    Ok(())
}