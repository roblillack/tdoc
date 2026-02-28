#[cfg(feature = "remote")]
mod gemini_client;

use clap::{Parser, ValueEnum, ValueHint};
use crossterm::terminal;
#[cfg(feature = "remote")]
use reqwest::blocking::Client;
#[cfg(feature = "remote")]
use reqwest::header::USER_AGENT;
use std::fs::File;
#[cfg(feature = "remote")]
use std::io::Cursor;
use std::io::{self, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
#[cfg(feature = "remote")]
use std::time::Duration;
use tdoc::formatter::{Formatter, FormattingStyle};
use tdoc::{gemini, html, markdown, pager, parse, write, Document};
use url::Url;

#[derive(Parser)]
#[command(
    name = "tdoc",
    version,
    about = "View and export FTML, HTML, Markdown, and Gemini documents"
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
    Gemini,
}

#[derive(Copy, Clone, ValueEnum)]
enum InputFormatArg {
    Ftml,
    Html,
    Markdown,
    Gemini,
}

impl From<InputFormatArg> for InputFormat {
    fn from(value: InputFormatArg) -> Self {
        match value {
            InputFormatArg::Ftml => InputFormat::Ftml,
            InputFormatArg::Html => InputFormat::Html,
            InputFormatArg::Markdown => InputFormat::Markdown,
            InputFormatArg::Gemini => InputFormat::Gemini,
        }
    }
}

#[derive(Clone)]
enum ContentOrigin {
    #[cfg_attr(not(feature = "remote"), allow(dead_code))]
    Url(Url),
    File(PathBuf),
    Stdin,
}

struct InputSource {
    format: InputFormat,
    reader: Box<dyn Read>,
    display_name: String,
    origin: ContentOrigin,
}

enum OutputFormat {
    Text,
    Ftml,
    Markdown,
    Html,
    Gemini,
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
        origin,
    } = input_source;
    let document = parse_document(format, reader, &display_name)?;

    if let Some(output_path) = cli.output {
        write_output(&document, &output_path)?;
    } else {
        view_document(document, cli.no_ansi, origin, input_override)?;
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
            origin: ContentOrigin::Stdin,
        }),
        Some("-") => Ok(InputSource {
            format: override_format.unwrap_or(InputFormat::Ftml),
            reader: Box::new(io::stdin()),
            display_name: "stdin".to_string(),
            origin: ContentOrigin::Stdin,
        }),
        Some(value) => {
            if let Ok(url) = Url::parse(value) {
                #[cfg(feature = "remote")]
                {
                    if url.scheme() == "http" || url.scheme() == "https" {
                        let client = Client::builder()
                            .timeout(Duration::from_secs(10))
                            .build()
                            .map_err(|err| format!("Unable to initialize HTTP client: {err}"))?;
                        let response = client
                            .get(value)
                            .header(
                                USER_AGENT,
                                concat!(
                                    "tdoc/",
                                    env!("CARGO_PKG_VERSION"),
                                    " (https://github.com/roblillack/tdoc)"
                                ),
                            )
                            .send()
                            .map_err(|err| format!("Unable to fetch {value}: {err}"))?;
                        let final_url = response.url().clone();
                        let origin = ContentOrigin::Url(final_url.clone());
                        let extension = Path::new(final_url.path())
                            .extension()
                            .and_then(|ext| ext.to_str());
                        let format = override_format
                            .or_else(|| detect_input_format(extension))
                            .unwrap_or(InputFormat::Html);
                        return Ok(InputSource {
                            format,
                            reader: Box::new(response),
                            display_name: final_url.to_string(),
                            origin,
                        });
                    } else if url.scheme() == "gemini" {
                        // Fetch via Gemini protocol
                        let response = gemini_client::fetch(value)
                            .map_err(|err| format!("Unable to fetch {value}: {err}"))?;

                        // Handle redirects
                        if response.is_redirect() {
                            if let Some(redirect_url) = response.redirect_url() {
                                return Err(format!(
                                    "Gemini redirect to: {} (status {})\nPlease follow the redirect manually.",
                                    redirect_url, response.status
                                ));
                            }
                        }

                        // Check for success status
                        if !response.is_success() {
                            return Err(format!(
                                "Gemini request failed with status {}: {}",
                                response.status, response.meta
                            ));
                        }

                        // Gemini responses default to text/gemini (gemtext)
                        let format = override_format.unwrap_or(InputFormat::Gemini);

                        return Ok(InputSource {
                            format,
                            reader: Box::new(Cursor::new(response.body)),
                            display_name: url.to_string(),
                            origin: ContentOrigin::Url(url.clone()),
                        });
                    }
                }

                #[cfg(not(feature = "remote"))]
                if matches!(url.scheme(), "http" | "https" | "gemini") {
                    return Err(format!(
                        "Remote URL support is not available (built without the \"remote\" feature)"
                    ));
                }
            }

            let path = Path::new(value);
            let file = File::open(path)
                .map_err(|err| format!("Unable to open {value} for reading: {err}"))?;
            let extension = path.extension().and_then(|ext| ext.to_str());
            let format = override_format
                .or_else(|| detect_input_format(extension))
                .unwrap_or(InputFormat::Ftml);

            let origin = ContentOrigin::File(path.to_path_buf());

            Ok(InputSource {
                format,
                reader: Box::new(BufReader::new(file)),
                display_name: value.to_string(),
                origin,
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
        "gmi" | "gemini" => Some(InputFormat::Gemini),
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
        InputFormat::Gemini => gemini::parse(reader)
            .map_err(|err| format!("Unable to parse {display_name} as Gemini: {err}")),
    }
}

fn view_document(
    document: Document,
    no_ansi: bool,
    origin: ContentOrigin,
    input_override: Option<InputFormat>,
) -> Result<(), String> {
    let stdout_is_tty = atty::is(atty::Stream::Stdout);
    let use_ansi = !no_ansi && stdout_is_tty;
    let use_pager = use_ansi;

    if !use_pager {
        let mut formatter = if use_ansi {
            let mut style = FormattingStyle::ansi();
            configure_style_for_terminal(&mut style);
            Formatter::new(io::stdout(), style)
        } else {
            Formatter::new_ascii(io::stdout())
        };

        return formatter
            .write_document(&document)
            .map_err(|err| format!("Unable to write document: {err}"));
    }

    let shared_state = Arc::new(Mutex::new(LinkEnvironment {
        document: document.clone(),
        origin: origin.clone(),
    }));

    let initial = render_document_for_terminal(&document, matches!(origin, ContentOrigin::Url(_)))?;
    let regen_state = shared_state.clone();
    let regenerator = move |new_width: u16, _new_height: u16| -> Result<String, String> {
        let guard = regen_state
            .lock()
            .map_err(|_| "Failed to access document for resize".to_string())?;
        render_document_for_width(
            &guard.document,
            new_width as usize,
            matches!(guard.origin, ContentOrigin::Url(_)),
        )
    };

    let link_policy = build_link_policy(&origin);
    let link_callback: Option<Arc<dyn pager::LinkCallback>> = match origin {
        ContentOrigin::Stdin => None,
        _ => Some(Arc::new(LinkCallbackState::new(
            shared_state.clone(),
            input_override,
        ))),
    };

    let mut options = pager::PagerOptions {
        link_policy,
        link_callback,
        ..pager::PagerOptions::default()
    };
    if matches!(origin, ContentOrigin::Url(_)) && stdout_is_tty {
        options.force_page = true;
    }

    pager::page_output_with_options_and_regenerator(&initial, Some(regenerator), options)
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

fn render_document_for_terminal(
    document: &Document,
    disable_link_footnotes: bool,
) -> Result<String, String> {
    let mut buf = Vec::new();
    let mut style = FormattingStyle::ansi();
    configure_style_for_terminal(&mut style);
    if disable_link_footnotes {
        style.link_footnotes = false;
    }
    {
        let mut formatter = Formatter::new(&mut buf, style);
        formatter
            .write_document(document)
            .map_err(|err| format!("Unable to write document: {err}"))?;
    }
    String::from_utf8(buf).map_err(|err| format!("UTF-8 error: {err}"))
}

fn render_document_for_width(
    document: &Document,
    width: usize,
    disable_link_footnotes: bool,
) -> Result<String, String> {
    let mut buf = Vec::new();
    let mut style = FormattingStyle::ansi();
    configure_style_for_width(&mut style, width);
    if disable_link_footnotes {
        style.link_footnotes = false;
    }
    {
        let mut formatter = Formatter::new(&mut buf, style);
        formatter
            .write_document(document)
            .map_err(|err| format!("Unable to write document: {err}"))?;
    }
    String::from_utf8(buf).map_err(|err| format!("UTF-8 error: {err}"))
}

struct LinkEnvironment {
    document: Document,
    origin: ContentOrigin,
}

struct LinkCallbackState {
    shared: Arc<Mutex<LinkEnvironment>>,
    input_override: Option<InputFormat>,
}

impl LinkCallbackState {
    fn new(shared: Arc<Mutex<LinkEnvironment>>, input_override: Option<InputFormat>) -> Self {
        Self {
            shared,
            input_override,
        }
    }
}

impl pager::LinkCallback for LinkCallbackState {
    fn on_link(
        &self,
        target: &str,
        context: &mut pager::LinkCallbackContext<'_>,
    ) -> Result<(), String> {
        let trimmed = target.trim();
        if trimmed.is_empty() {
            return Ok(());
        }

        let origin = {
            let guard = self
                .shared
                .lock()
                .map_err(|_| "Unable to read current document state".to_string())?;
            guard.origin.clone()
        };

        context.set_status(format!("Loading {trimmed} ..."))?;

        match navigate_to_target(&origin, trimmed, self.input_override) {
            Ok(Some((document, new_origin))) => {
                let render_width = context.content_width().max(1);
                let rendered = render_document_for_width(
                    &document,
                    render_width,
                    matches!(new_origin, ContentOrigin::Url(_)),
                )?;
                context.replace_content(&rendered)?;
                context.set_link_policy(build_link_policy(&new_origin));
                {
                    let mut guard = self
                        .shared
                        .lock()
                        .map_err(|_| "Unable to update current document state".to_string())?;
                    guard.document = document;
                    guard.origin = new_origin;
                }
                context.clear_status()?;
            }
            Ok(None) => {
                context.set_status("Unable to open link".to_string())?;
            }
            Err(err) => {
                context.set_status(format!("Error: {err}"))?;
            }
        }

        Ok(())
    }
}

fn build_link_policy(origin: &ContentOrigin) -> pager::LinkPolicy {
    match origin {
        ContentOrigin::Url(base_url) => {
            let base = base_url.clone();
            pager::LinkPolicy::new(
                false,
                Arc::new(move |target: &str| {
                    let trimmed = target.trim();
                    if trimmed.is_empty() || trimmed.starts_with('#') {
                        return false;
                    }
                    match Url::options().base_url(Some(&base)).parse(trimmed) {
                        Ok(resolved) => matches!(resolved.scheme(), "http" | "https" | "gemini"),
                        Err(_) => false,
                    }
                }),
            )
        }
        ContentOrigin::File(path) => {
            let base_dir = path
                .parent()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            pager::LinkPolicy::new(
                true,
                Arc::new(move |target: &str| {
                    let trimmed = target.trim();
                    if trimmed.is_empty() || is_absolute_url(trimmed) {
                        return false;
                    }
                    let candidate = if Path::new(trimmed).is_absolute() {
                        PathBuf::from(trimmed)
                    } else {
                        base_dir.join(trimmed)
                    };
                    match std::fs::canonicalize(&candidate) {
                        Ok(resolved) => resolved.is_file(),
                        Err(_) => false,
                    }
                }),
            )
        }
        ContentOrigin::Stdin => pager::LinkPolicy::new(true, Arc::new(|_| false)),
    }
}

fn navigate_to_target(
    origin: &ContentOrigin,
    target: &str,
    input_override: Option<InputFormat>,
) -> Result<Option<(Document, ContentOrigin)>, String> {
    let trimmed = target.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    match origin {
        ContentOrigin::Url(current_url) => {
            if trimmed.starts_with('#') {
                return Ok(None);
            }
            let resolved = match Url::options().base_url(Some(current_url)).parse(trimmed) {
                Ok(url) => url,
                Err(_) => return Ok(None),
            };
            if !matches!(resolved.scheme(), "http" | "https" | "gemini") {
                return Ok(None);
            }
            if &resolved == current_url {
                return Ok(None);
            }

            let input_source = create_reader(Some(resolved.as_str()), input_override)?;
            let InputSource {
                format,
                reader,
                display_name,
                origin,
            } = input_source;
            let document = parse_document(format, reader, &display_name)?;
            Ok(Some((document, origin)))
        }
        ContentOrigin::File(current_path) => {
            if is_absolute_url(trimmed) {
                return Ok(None);
            }
            let base_dir = current_path
                .parent()
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            let candidate = if Path::new(trimmed).is_absolute() {
                PathBuf::from(trimmed)
            } else {
                base_dir.join(trimmed)
            };
            let resolved = match std::fs::canonicalize(&candidate) {
                Ok(path) => path,
                Err(_) => return Ok(None),
            };
            if !resolved.is_file() {
                return Ok(None);
            }
            let path_string = match resolved.to_str() {
                Some(value) => value.to_owned(),
                None => return Ok(None),
            };
            let input_source = create_reader(Some(path_string.as_str()), input_override)?;
            let InputSource {
                format,
                reader,
                display_name,
                origin,
            } = input_source;
            let document = parse_document(format, reader, &display_name)?;
            Ok(Some((document, origin)))
        }
        ContentOrigin::Stdin => Ok(None),
    }
}

fn is_absolute_url(value: &str) -> bool {
    Url::parse(value).is_ok()
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
        OutputFormat::Gemini => {
            let mut file = File::create(output_path).map_err(|err| {
                format!(
                    "Unable to open {} for writing: {err}",
                    output_path.display()
                )
            })?;
            gemini::write(&mut file, document).map_err(|err| {
                format!("Unable to write Gemini to {}: {err}", output_path.display())
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
        "gmi" | "gemini" => Some(OutputFormat::Gemini),
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
