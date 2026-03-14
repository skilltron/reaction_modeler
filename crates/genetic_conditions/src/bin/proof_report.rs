//! Minimal proof report: shows exactly which file was read, variant count, first/last variant, and one SNP.
//! Use to verify the pipeline shows different output for different genomes.
//!
//! Usage: proof-report <variants.json> [output.html]
//!   If no output: prints to stdout. Otherwise writes HTML to the given path.

use genetic_conditions::VariantInput;
use std::env;
use std::fs;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: proof-report <variants.json> [output.html]");
        std::process::exit(1);
    }
    let path = &args[1];
    let out_path = args.get(2).map(String::as_str);

    eprintln!("[proof-report] Reading from: {}", path);
    let raw = fs::read_to_string(path)?;
    let variants: Vec<VariantInput> = serde_json::from_str(&raw)?;
    if variants.is_empty() {
        eprintln!("Error: 0 variants");
        std::process::exit(1);
    }

    let first = &variants[0];
    let last = variants.last().unwrap();
    let mthfr = variants
        .iter()
        .find(|v| v.rsid.as_deref() == Some("rs1801133"));
    let mthfr_str = mthfr
        .map(|v| {
            format!(
                "{} (chr {} pos {} ref {:?} alt {:?} genotype {:?})",
                v.rsid.as_deref().unwrap_or(""),
                v.chromosome,
                v.position,
                v.ref_allele,
                v.alt_allele,
                v.genotype
            )
        })
        .unwrap_or_else(|| "not found in this variant set".to_string());

    let html = format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="UTF-8"><title>Proof report</title></head>
<body style="font-family: sans-serif; margin: 2em;">
<h1>Minimal proof report</h1>
<p><strong>This report was generated from exactly one file.</strong> No other data source.</p>
<table border="1" cellpadding="8" style="border-collapse: collapse;">
<tr><td><strong>File read</strong></td><td>{path}</td></tr>
<tr><td><strong>Variant count</strong></td><td>{count}</td></tr>
<tr><td><strong>First variant</strong></td><td>chr {f_chr} pos {f_pos} rsid {f_rsid} ref {f_ref:?} alt {f_alt:?} genotype {f_gt:?}</td></tr>
<tr><td><strong>Last variant</strong></td><td>chr {l_chr} pos {l_pos} rsid {l_rsid} ref {l_ref:?} alt {l_alt:?} genotype {l_gt:?}</td></tr>
<tr><td><strong>MTHFR rs1801133 (proof SNP)</strong></td><td>{mthfr}</td></tr>
</table>
<p>If you run this for two different people, the table above must differ (different count, first/last, and/or rs1801133).</p>
<p>Generated: {now}</p>
</body>
</html>"#,
        path = path.replace('<', "&lt;").replace('>', "&gt;"),
        count = variants.len(),
        f_chr = first.chromosome,
        f_pos = first.position,
        f_rsid = first.rsid.as_deref().unwrap_or("—"),
        f_ref = first.ref_allele,
        f_alt = first.alt_allele,
        f_gt = first.genotype,
        l_chr = last.chromosome,
        l_pos = last.position,
        l_rsid = last.rsid.as_deref().unwrap_or("—"),
        l_ref = last.ref_allele,
        l_alt = last.alt_allele,
        l_gt = last.genotype,
        mthfr = mthfr_str.replace('<', "&lt;").replace('>', "&gt;"),
        now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
    );

    if let Some(out) = out_path {
        fs::write(out, &html)?;
        eprintln!("[proof-report] Wrote: {}", out);
    } else {
        io::stdout().write_all(html.as_bytes())?;
    }
    Ok(())
}
