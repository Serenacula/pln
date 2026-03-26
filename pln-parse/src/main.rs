use std::io::Read;
use std::process;

use clap::Parser;

/// Parser and validator for Panel Layout Notation (PLN).
#[derive(Parser)]
#[command(version, about)]
struct Args {
    /// Input file (omit or use "-" for stdin)
    file: Option<String>,

    /// Validate only; exit 0 if valid, 1 if invalid
    #[arg(short = 'c', long)]
    check: bool,

    /// Pretty-print JSON output
    #[arg(short, long)]
    pretty: bool,

    /// Omit span information from JSON output
    #[arg(long)]
    no_spans: bool,
}

fn main() {
    let args = Args::parse();

    let input = match read_input(&args.file) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("error: {}", error);
            process::exit(2);
        }
    };

    let input = input.trim();

    match pln_parse::parse_and_validate(input) {
        Ok(item) => {
            if args.check {
                process::exit(0);
            }

            let json = if args.no_spans {
                let stripped = strip_spans(&serde_json::to_value(&item).unwrap());
                if args.pretty {
                    serde_json::to_string_pretty(&stripped)
                } else {
                    serde_json::to_string(&stripped)
                }
            } else if args.pretty {
                serde_json::to_string_pretty(&item)
            } else {
                serde_json::to_string(&item)
            };

            println!("{}", json.unwrap());
        }
        Err(error) => {
            eprint!("{}", error.format_with_source(input));
            process::exit(1);
        }
    }
}

fn read_input(file: &Option<String>) -> Result<String, Box<dyn std::error::Error>> {
    match file.as_deref() {
        None | Some("-") => {
            let mut buffer = String::new();
            std::io::stdin().read_to_string(&mut buffer)?;
            Ok(buffer)
        }
        Some(path) => Ok(std::fs::read_to_string(path)?),
    }
}

/// Recursively remove "span" keys from a JSON value.
fn strip_spans(value: &serde_json::Value) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let filtered = map
                .iter()
                .filter(|(key, _)| key.as_str() != "span")
                .map(|(key, value)| (key.clone(), strip_spans(value)))
                .collect();
            serde_json::Value::Object(filtered)
        }
        serde_json::Value::Array(array) => {
            serde_json::Value::Array(array.iter().map(strip_spans).collect())
        }
        other => other.clone(),
    }
}
