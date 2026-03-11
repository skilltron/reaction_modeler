//! Binary: read variants (JSON) and emit full HTML report including expanded MCAS and related conditions.
//!
//! Usage:
//!   genetic-report-html [variants.json [output.html]]
//! If no args: read variants from stdin (JSON array), write HTML to stdout.
//! If one arg: read from file, write HTML to stdout.
//! If two args: read from first file, write HTML to second file.

use genetic_conditions::{check_variants_against_all, html_report, VariantInput};
use std::env;
use std::io::{self, BufReader, Read, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let (json_input, out_path, capacity_hint): (Box<dyn Read>, Option<&str>, Option<usize>) = match args.len() {
        1 => (Box::new(io::stdin()), None, None),
        2 => {
            let f = std::fs::File::open(&args[1])?;
            let len = f.metadata().map(|m| m.len() as usize).unwrap_or(0);
            (Box::new(BufReader::new(f)), None, Some(len.max(4096)))
        }
        3 => {
            let f = std::fs::File::open(&args[1])?;
            let len = f.metadata().map(|m| m.len() as usize).unwrap_or(0);
            (Box::new(BufReader::new(f)), Some(args[2].as_str()), Some(len.max(4096)))
        }
        _ => {
            eprintln!("Usage: genetic-report-html [variants.json [output.html]]");
            eprintln!("  No args: read JSON from stdin, write HTML to stdout");
            eprintln!("  One arg: read from file, write HTML to stdout");
            eprintln!("  Two args: read from first file, write to second file");
            std::process::exit(1);
        }
    };

    let mut raw = String::new();
    if let Some(cap) = capacity_hint {
        raw.reserve(cap);
    }
    let mut reader = json_input;
    reader.read_to_string(&mut raw)?;
    let variants: Vec<VariantInput> = serde_json::from_str(&raw)?;

    let report = check_variants_against_all(&variants);
    let report_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let title = "Genetic Conditions Report (MCAS & related expanded)";
    let html = html_report::all_conditions_to_html(&report, title, &report_date);

    if let Some(path) = out_path {
        std::fs::write(path, html)?;
    } else {
        io::stdout().write_all(html.as_bytes())?;
    }
    Ok(())
}
