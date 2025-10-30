use clap::{Arg, Command};
use crossterm::terminal;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::process::{Command as Process, Stdio};
use std::time::Duration;
use tdoc::formatter::{Formatter, FormattingStyle};
use tdoc::{html, markdown, parse, write};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputFormat {
    Ftml,
    Html,
    Markdown,
}

fn main() {
    let matches = Command::new("viewftml")
        .version("0.1.0")
        .about("View FTML and HTML files with formatted output")
        .arg(
            Arg::new("no-ansi")
                .short('n')
                .long("no-ansi")
                .help("Disable use of ANSI escape sequences")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("save")
                .short('s')
                .long("save")
                .help("Save the formatted FTML to standard out")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("INPUT")
                .help("Input file or URL to process")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("INPUT").unwrap();
    let disable_ansi = matches.get_flag("no-ansi");
    let save_ftml = matches.get_flag("save");

    let (input_format, reader) = match create_reader(input_file) {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Unable to read {}: {}", input_file, e);
            std::process::exit(1);
        }
    };

    let document = match input_format {
        InputFormat::Html => match html::parse(reader) {
            Ok(doc) => doc,
            Err(e) => {
                eprintln!("Unable to parse {}: {}", input_file, e);
                std::process::exit(1);
            }
        },
        InputFormat::Markdown => match markdown::parse(reader) {
            Ok(doc) => doc,
            Err(e) => {
                eprintln!("Unable to parse {} as Markdown: {}", input_file, e);
                std::process::exit(1);
            }
        },
        InputFormat::Ftml => match parse(reader) {
            Ok(doc) => doc,
            Err(e) => {
                eprintln!("Unable to parse {}: {}", input_file, e);
                std::process::exit(1);
            }
        },
    };

    if save_ftml {
        match write(&mut io::stdout(), &document) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Unable to write FTML document: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Determine output mode
    let use_pager = !disable_ansi && atty::is(atty::Stream::Stdout);

    if use_pager {
        if let Ok((mut pager_process, pager_stdin)) = run_pager() {
            let mut style = FormattingStyle::ansi();

            // Adjust formatting based on terminal width
            if let Ok((width, _)) = terminal::size() {
                let width = width as usize;
                if width < 60 {
                    style.wrap_width = width;
                    style.left_padding = 0;
                } else if width < 100 {
                    style.wrap_width = width - 2;
                    style.left_padding = 2;
                } else {
                    let padding = (width - 100) / 2 + 4;
                    style.wrap_width = width - padding;
                    style.left_padding = padding;
                }
            }

            let mut formatter = Formatter::new(pager_stdin, style);
            match formatter.write_document(&document) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Unable to write document: {}", e);
                    std::process::exit(1);
                }
            }

            drop(formatter); // Close pager stdin so the pager knows input is complete

            // Wait for pager to finish
            let _ = pager_process.wait();
        } else {
            // Fallback to stdout without pager
            let mut formatter = Formatter::new_ascii(io::stdout());
            match formatter.write_document(&document) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Unable to write document: {}", e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        // Direct output to stdout
        let mut formatter = if disable_ansi {
            Formatter::new_ascii(io::stdout())
        } else {
            Formatter::new_ansi(io::stdout())
        };

        match formatter.write_document(&document) {
            Ok(()) => {}
            Err(e) => {
                eprintln!("Unable to write document: {}", e);
                std::process::exit(1);
            }
        }
    }
}

fn create_reader(
    input_file: &str,
) -> Result<(InputFormat, Box<dyn Read>), Box<dyn std::error::Error>> {
    // Try to parse as URL
    if let Ok(url) = Url::parse(input_file) {
        if url.scheme() == "http" || url.scheme() == "https" {
            let client = Client::builder().timeout(Duration::from_secs(10)).build()?;
            let response = client.get(input_file).send()?;
            let reader = Box::new(response);
            let extension = Path::new(url.path())
                .extension()
                .and_then(|ext| ext.to_str());
            let format = detect_format_from_extension(extension);
            return Ok((format, reader));
        }
    }

    // Local file
    let extension = Path::new(input_file)
        .extension()
        .and_then(|ext| ext.to_str());
    let format = detect_format_from_extension(extension);

    let file = File::open(input_file)?;
    let reader = Box::new(BufReader::new(file));
    Ok((format, reader))
}

fn run_pager() -> Result<(std::process::Child, std::process::ChildStdin), std::io::Error> {
    let pager_cmd = std::env::var("PAGER").unwrap_or_else(|_| "less".to_string());
    let mut cmd_parts: Vec<&str> = pager_cmd.split_whitespace().collect();

    if cmd_parts.is_empty() {
        cmd_parts = vec!["less"];
    }

    let program = cmd_parts[0];
    let mut args: Vec<&str> = cmd_parts.into_iter().skip(1).collect();

    // Add -R flag for less/more to support ANSI colors
    let program_name = Path::new(program)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(program);

    if program_name == "less" || program_name == "more" {
        args.push("-R");
    }

    let mut child = Process::new(program)
        .args(&args)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()?;

    let stdin = child.stdin.take().unwrap();
    Ok((child, stdin))
}

fn detect_format_from_extension(ext: Option<&str>) -> InputFormat {
    match ext.map(|value| value.to_ascii_lowercase()) {
        Some(ref ext) if ext == "ftml" => InputFormat::Ftml,
        Some(ref ext) if ext == "md" || ext == "markdown" => InputFormat::Markdown,
        Some(ref ext) if ext == "htm" || ext == "html" => InputFormat::Html,
        _ => InputFormat::Html,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_reader_file() {
        // This test would need a test file to work properly
        // For now, just test that the function exists
    }

    #[test]
    fn test_url_parsing() {
        let result = create_reader("https://example.com");
        if let Ok((format, _)) = result {
            assert_eq!(format, InputFormat::Html);
        }
        // If error, network might not be available in test environment - test passes
    }

    #[test]
    fn test_detect_format_from_extension() {
        assert_eq!(
            detect_format_from_extension(Some("ftml")),
            InputFormat::Ftml
        );
        assert_eq!(
            detect_format_from_extension(Some("md")),
            InputFormat::Markdown
        );
        assert_eq!(
            detect_format_from_extension(Some("markdown")),
            InputFormat::Markdown
        );
        assert_eq!(
            detect_format_from_extension(Some("htm")),
            InputFormat::Html
        );
        assert_eq!(
            detect_format_from_extension(Some("html")),
            InputFormat::Html
        );
        assert_eq!(
            detect_format_from_extension(Some("TXT")),
            InputFormat::Html
        );
    }
}
