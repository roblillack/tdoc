use clap::{Arg, Command};
use ftml::{markdown, parse};
use std::fs::File;
use std::io::{self, BufReader};

fn main() {
    let matches = Command::new("ftml2md")
        .version("0.1.0")
        .about("Convert FTML documents to Markdown")
        .arg(
            Arg::new("INPUT")
                .help("Input FTML file (use '-' for stdin)")
                .default_value("-")
                .index(1),
        )
        .arg(
            Arg::new("OUTPUT")
                .help("Output Markdown file (use '-' for stdout)")
                .default_value("-")
                .index(2),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("INPUT").unwrap();
    let output_file = matches.get_one::<String>("OUTPUT").unwrap();

    // Open input
    let reader: Box<dyn std::io::Read> = if input_file == "-" {
        Box::new(io::stdin())
    } else {
        match File::open(input_file) {
            Ok(file) => Box::new(BufReader::new(file)),
            Err(e) => {
                eprintln!("Unable to open {} for reading: {}", input_file, e);
                std::process::exit(1);
            }
        }
    };

    // Parse FTML document
    let document = match parse(reader) {
        Ok(doc) => doc,
        Err(e) => {
            eprintln!("Unable to parse {}: {}", input_file, e);
            std::process::exit(1);
        }
    };

    // Open output
    let mut writer: Box<dyn std::io::Write> = if output_file == "-" {
        Box::new(io::stdout())
    } else {
        match File::create(output_file) {
            Ok(file) => Box::new(file),
            Err(e) => {
                eprintln!("Unable to open {} for writing: {}", output_file, e);
                std::process::exit(1);
            }
        }
    };

    // Write Markdown
    match markdown::write(&mut writer, &document) {
        Ok(()) => {}
        Err(e) => {
            eprintln!("Unable to write document to {}: {}", output_file, e);
            std::process::exit(1);
        }
    }

    // Close output file if it's not stdout
    if output_file != "-" {
        if let Err(e) = writer.flush() {
            eprintln!("Unable to close {} after writing: {}", output_file, e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::process::Command;

    #[test]
    fn test_cli_help() {
        let output = Command::new("cargo")
            .args(&["run", "--bin", "ftml2md", "--", "--help"])
            .output()
            .expect("Failed to execute command");
        
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("Convert FTML documents to Markdown"));
    }
}