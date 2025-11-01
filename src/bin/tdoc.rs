use clap::{Parser, ValueEnum, ValueHint};
use crossterm::terminal;
use reqwest::blocking::Client;
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command as Process, Stdio};
use std::time::Duration;
use tdoc::formatter::{Formatter, FormattingStyle};
use tdoc::{html, markdown, pager, parse, write, Document};
use url::Url;

#[derive(Parser)]
#[command(
    name = "tdoc",
    version,
    about = "View and export FTML, HTML, and Markdown documents"
)]
struct Cli {
    /// Input file or URL (omit to read from stdin)
    #[arg(value_name = "INPUT")]
    input: Option<String>,

    /// Disable ANSI escape sequences in terminal output
    #[arg(long = "no-ansi")]
    no_ansi: bool,

    /// Explicitly set the input format when auto-detection is insufficient
    #[arg(long = "input-format", value_enum)]
    input_format: Option<InputFormatArg>,

    /// Write the rendered document to a file instead of the terminal
    #[arg(short = 'o', long = "output", value_name = "FILE", value_hint = ValueHint::FilePath)]
    output: Option<PathBuf>,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum InputFormat {
    Ftml,
    Html,
    Markdown,
}

#[derive(Copy, Clone, ValueEnum)]
enum InputFormatArg {
    Ftml,
    Html,
    Markdown,
}

impl From<InputFormatArg> for InputFormat {
    fn from(value: InputFormatArg) -> Self {
        match value {
            InputFormatArg::Ftml => InputFormat::Ftml,
            InputFormatArg::Html => InputFormat::Html,
            InputFormatArg::Markdown => InputFormat::Markdown,
        }
    }
}

struct InputSource {
    format: InputFormat,
    reader: Box<dyn Read>,
    display_name: String,
}

enum OutputFormat {
    Text,
    Ftml,
    Markdown,
    Html,
}

fn main() {
    if let Err(message) = run() {
        eprintln!("{message}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();
    let input_override = cli.input_format.map(InputFormat::from);
    let input_source = create_reader(cli.input.as_deref(), input_override)?;
    let InputSource {
        format,
        reader,
        display_name,
    } = input_source;
    let document = parse_document(format, reader, &display_name)?;

    if let Some(output_path) = cli.output {
        write_output(&document, &output_path)?;
    } else {
        view_document(&document, cli.no_ansi)?;
    }

    Ok(())
}

fn create_reader(
    argument: Option<&str>,
    override_format: Option<InputFormat>,
) -> Result<InputSource, String> {
    match argument {
        None => Ok(InputSource {
            format: override_format.unwrap_or(InputFormat::Ftml),
            reader: Box::new(io::stdin()),
            display_name: "stdin".to_string(),
        }),
        Some("-") => Ok(InputSource {
            format: override_format.unwrap_or(InputFormat::Ftml),
            reader: Box::new(io::stdin()),
            display_name: "stdin".to_string(),
        }),
        Some(value) => {
            if let Ok(url) = Url::parse(value) {
                if url.scheme() == "http" || url.scheme() == "https" {
                    let client = Client::builder()
                        .timeout(Duration::from_secs(10))
                        .build()
                        .map_err(|err| format!("Unable to initialize HTTP client: {err}"))?;
                    let response = client
                        .get(value)
                        .send()
                        .map_err(|err| format!("Unable to fetch {value}: {err}"))?;
                    let extension = Path::new(url.path())
                        .extension()
                        .and_then(|ext| ext.to_str());
                    let format = override_format
                        .or_else(|| detect_input_format(extension))
                        .unwrap_or(InputFormat::Html);
                    return Ok(InputSource {
                        format,
                        reader: Box::new(response),
                        display_name: value.to_string(),
                    });
                }
            }

            let path = Path::new(value);
            let file = File::open(path)
                .map_err(|err| format!("Unable to open {value} for reading: {err}"))?;
            let extension = path.extension().and_then(|ext| ext.to_str());
            let format = override_format
                .or_else(|| detect_input_format(extension))
                .unwrap_or(InputFormat::Ftml);

            Ok(InputSource {
                format,
                reader: Box::new(BufReader::new(file)),
                display_name: value.to_string(),
            })
        }
    }
}

fn detect_input_format(extension: Option<&str>) -> Option<InputFormat> {
    let ext = extension?.to_ascii_lowercase();
    match ext.as_str() {
        "ftml" => Some(InputFormat::Ftml),
        "html" | "htm" => Some(InputFormat::Html),
        "md" | "markdown" => Some(InputFormat::Markdown),
        _ => None,
    }
}

fn parse_document(
    format: InputFormat,
    reader: Box<dyn Read>,
    display_name: &str,
) -> Result<Document, String> {
    match format {
        InputFormat::Ftml => {
            parse(reader).map_err(|err| format!("Unable to parse {display_name} as FTML: {err}"))
        }
        InputFormat::Html => html::parse(reader)
            .map_err(|err| format!("Unable to parse {display_name} as HTML: {err}")),
        InputFormat::Markdown => markdown::parse(reader)
            .map_err(|err| format!("Unable to parse {display_name} as Markdown: {err}")),
    }
}

fn view_document(document: &Document, no_ansi: bool) -> Result<(), String> {
    let stdout_is_tty = atty::is(atty::Stream::Stdout);
    let use_ansi = !no_ansi && stdout_is_tty;
    let use_pager = use_ansi;

    if use_pager {
        // if let Ok((mut pager, pager_stdin)) = run_pager() {
        //     let mut style = FormattingStyle::ansi();
        //     configure_style_for_terminal(&mut style);

        //     let mut formatter = Formatter::new(pager_stdin, style);
        //     formatter
        //         .write_document(document)
        //         .map_err(|err| format!("Unable to write document: {err}"))?;

        //     drop(formatter);
        //     let _ = pager.wait();
        //     return Ok(());
        // }

        let mut buf = Vec::new();
        if use_ansi {
            let mut style = FormattingStyle::ansi();
            configure_style_for_terminal(&mut style);
            {
                let mut formatter = Formatter::new(&mut buf, style);
                formatter
                    .write_document(document)
                    .map_err(|err| format!("Unable to write document: {err}"))?;
            }
        } else {
            let mut formatter = Formatter::new_ascii(&mut buf);
            formatter
                .write_document(document)
                .map_err(|err| format!("Unable to write document: {err}"))?;
        }

        let initial = String::from_utf8(buf).map_err(|err| format!("UTF-8 error: {err}"))?;

        if use_ansi {
            let regenerator = |new_width: u16, _new_height: u16| -> Result<String, String> {
                let mut buf = Vec::new();
                let mut style = FormattingStyle::ansi();
                configure_style_for_width(&mut style, new_width as usize);
                {
                    let mut formatter = Formatter::new(&mut buf, style);
                    formatter
                        .write_document(document)
                        .map_err(|err| format!("Unable to write document: {err}"))?;
                }
                String::from_utf8(buf).map_err(|err| format!("UTF-8 error: {err}"))
            };

            return pager::page_output_with_regenerator(&initial, Some(regenerator));
        }

        return pager::page_output(&initial);
    }

    let mut formatter = if use_ansi {
        let mut style = FormattingStyle::ansi();
        configure_style_for_terminal(&mut style);
        Formatter::new(io::stdout(), style)
    } else {
        Formatter::new_ascii(io::stdout())
    };

    formatter
        .write_document(document)
        .map_err(|err| format!("Unable to write document: {err}"))
}

fn configure_style_for_terminal(style: &mut FormattingStyle) {
    if let Ok((width, _height)) = terminal::size() {
        configure_style_for_width(style, width as usize);
    }
}

fn configure_style_for_width(style: &mut FormattingStyle, width: usize) {
    if width < 60 {
        style.wrap_width = width;
        style.left_padding = 0;
    } else if width < 100 {
        style.wrap_width = width.saturating_sub(2);
        style.left_padding = 2;
    } else {
        let padding = (width.saturating_sub(100)) / 2 + 4;
        style.wrap_width = width.saturating_sub(padding);
        style.left_padding = padding;
    }
}

fn run_pager() -> io::Result<(std::process::Child, std::process::ChildStdin)> {
    let pager_cmd = std::env::var("PAGER").unwrap_or_else(|_| "less".to_string());
    let mut parts: Vec<&str> = pager_cmd.split_whitespace().collect();

    if parts.is_empty() {
        parts.push("less");
    }

    let program = parts[0];
    let program_name = Path::new(program)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(program);

    let mut final_args: Vec<&str> = parts.into_iter().skip(1).collect();
    if program_name == "less" || program_name == "more" {
        final_args.push("-R");
    }

    let mut child = Process::new(program)
        .args(&final_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()?;

    let stdin = child
        .stdin
        .take()
        .ok_or_else(|| io::Error::other("Unable to open pager stdin"))?;

    Ok((child, stdin))
}

fn write_output(document: &Document, output_path: &Path) -> Result<(), String> {
    if output_path == Path::new("-") {
        return Err(
            "Use stdout by omitting --output; it already writes to stdout by default.".to_string(),
        );
    }

    let extension = output_path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase());

    let format = determine_output_format(extension.as_deref()).ok_or_else(|| {
        format!(
            "Unable to determine output format for {}",
            output_path.display()
        )
    })?;

    match format {
        OutputFormat::Text => {
            let file = File::create(output_path).map_err(|err| {
                format!(
                    "Unable to open {} for writing: {err}",
                    output_path.display()
                )
            })?;
            let mut formatter = Formatter::new_ascii(file);
            formatter.write_document(document).map_err(|err| {
                format!(
                    "Unable to write document to {}: {err}",
                    output_path.display()
                )
            })?;
            Ok(())
        }
        OutputFormat::Ftml => {
            let mut file = File::create(output_path).map_err(|err| {
                format!(
                    "Unable to open {} for writing: {err}",
                    output_path.display()
                )
            })?;
            write(&mut file, document).map_err(|err| {
                format!("Unable to write FTML to {}: {err}", output_path.display())
            })?;
            file.flush()
                .map_err(|err| format!("Unable to flush {}: {err}", output_path.display()))
        }
        OutputFormat::Markdown => {
            let mut file = File::create(output_path).map_err(|err| {
                format!(
                    "Unable to open {} for writing: {err}",
                    output_path.display()
                )
            })?;
            markdown::write(&mut file, document).map_err(|err| {
                format!(
                    "Unable to write Markdown to {}: {err}",
                    output_path.display()
                )
            })?;
            file.flush()
                .map_err(|err| format!("Unable to flush {}: {err}", output_path.display()))
        }
        OutputFormat::Html => {
            let mut file = File::create(output_path).map_err(|err| {
                format!(
                    "Unable to open {} for writing: {err}",
                    output_path.display()
                )
            })?;
            write_html_document(&mut file, document).map_err(|err| {
                format!("Unable to write HTML to {}: {err}", output_path.display())
            })?;
            file.flush()
                .map_err(|err| format!("Unable to flush {}: {err}", output_path.display()))
        }
    }
}

fn determine_output_format(extension: Option<&str>) -> Option<OutputFormat> {
    let ext = extension?;
    match ext {
        "txt" | "text" => Some(OutputFormat::Text),
        "ftml" => Some(OutputFormat::Ftml),
        "md" | "markdown" => Some(OutputFormat::Markdown),
        "html" | "htm" => Some(OutputFormat::Html),
        _ => None,
    }
}

fn write_html_document<W: Write>(mut writer: W, document: &Document) -> io::Result<()> {
    writer.write_all(
        b"<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"utf-8\" />\n</head>\n<body>\n",
    )?;
    write(&mut writer, document)?;
    writer.write_all(b"\n</body>\n</html>\n")
}
