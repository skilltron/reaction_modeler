//! Binary: read variants (JSON) and emit full HTML report including expanded MCAS and related conditions.
//!
//! Usage:
//!   genetic-report-html [variants.json [output.html [copy_number.json [report_name]]]]
//! If no args: read variants from stdin (JSON array), write HTML to stdout.
//! If one arg: read from file, write HTML to stdout.
//! If two args: read from first file, write to second file.
//! If three args: also load copy number assay from copy_number.json (used for HαT/TPSAB1 etc.).
//! If four args: optional report_name used in the report title (e.g. "Lisa Haskell — Genetic Conditions Report").
//!
//! Copy number JSON format: array of { "gene": "TPSAB1", "copy_number": 3, "source": "optional" }.

use genetic_conditions::{clinvar_lookup, copy_number, cascade, check_variants_against_all, exercise_ammonia, gene_annotation, html_report, mcas_integrated, reference_check, report_plain_text, sequencing_parity, star_alleles, survival, VariantInput};
use std::env;
use std::io::{self, BufReader, Read, Write};
use std::process::Command;

const DATA_TAP_POINTS: usize = 100;

/// Fingerprint string for a variant set: count first_chr:pos last_chr:pos (for intercept comparison).
fn fingerprint(variants: &[VariantInput]) -> String {
    if variants.is_empty() {
        return "0 (empty)".to_string();
    }
    let first = &variants[0];
    let last = variants.last().unwrap();
    format!(
        "{} first {}:{} last {}:{}",
        variants.len(),
        first.chromosome,
        first.position,
        last.chromosome,
        last.position
    )
}

/// When REPORT_INTERCEPT=1 and REPORT_RUN_ID=<id>, write one file per stage: report_intercept_<id>_<stage>.txt
/// with a single line fingerprint. Compare two runs (e.g. Siva vs Henry) to find first stage where data corrupts.
fn intercept_write(run_id: &str, stage: &str, variants: &[VariantInput]) {
    let fp = fingerprint(variants);
    let path = format!("report_intercept_{}_{}.txt", run_id, stage);
    if let Ok(mut f) = std::fs::File::create(&path) {
        let _ = writeln!(f, "{}", fp);
        eprintln!("[report] intercept {} -> {} ({})", stage, path, fp);
    }
}

/// When REPORT_DATA_TRACKER=1, write fingerprint + 100 sample points (chr:pos) at each stage to track where data might change.
fn tap_data(tap_id: u32, stage: &str, variants: &[VariantInput], w: &mut impl Write) {
    let n = variants.len();
    if n == 0 {
        let _ = writeln!(w, "tap{}\t{}\t0\t(empty)", tap_id, stage);
        return;
    }
    let first = &variants[0];
    let last = variants.last().unwrap();
    let mut points = Vec::with_capacity(DATA_TAP_POINTS);
    for i in 0..DATA_TAP_POINTS {
        let idx = if n <= 1 {
            0
        } else {
            (i as u64 * (n - 1) as u64 / (DATA_TAP_POINTS - 1) as u64) as usize
        };
        let v = &variants[idx];
        points.push(format!("{}:{}", v.chromosome, v.position));
    }
    let _ = writeln!(
        w,
        "tap{}\t{}\t{}\tfirst {}:{} last {}:{}\t{}",
        tap_id,
        stage,
        n,
        first.chromosome,
        first.position,
        last.chromosome,
        last.position,
        points.join(",")
    );
}

fn load_copy_number(path: &str) -> Result<Vec<copy_number::CopyNumberResult>, Box<dyn std::error::Error>> {
    let raw = std::fs::read_to_string(path)?;
    Ok(serde_json::from_str(&raw)?)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    // When the script sets GENETIC_REPORT_INPUT_FILE, we read ONLY from that path (so the main report
    // cannot accidentally use a different person's file passed as args[1]).
    let variant_file_path: Option<String> = env::var("GENETIC_REPORT_INPUT_FILE").ok();
    let read_path: Option<&str> = variant_file_path
        .as_deref()
        .or_else(|| args.get(1).map(|s| s.as_str()));
    // Log exactly which source we use so runs can be audited (e.g. Lisa vs Henry mix-up).
    if let Some(p) = read_path {
        if variant_file_path.is_some() {
            eprintln!("[report] INPUT SOURCE: GENETIC_REPORT_INPUT_FILE (ignoring args) -> {}", p);
        } else {
            eprintln!("[report] INPUT SOURCE: args[1] -> {}", p);
        }
    }

    let (json_input, out_path, copy_number_path, report_name, capacity_hint): (Box<dyn Read>, Option<&str>, Option<&str>, Option<&str>, Option<usize>) = match args.len() {
        1 => (Box::new(io::stdin()), None, None, None, None),
        2 => {
            let path = read_path.ok_or("Missing variant file path")?;
            let f = std::fs::File::open(path)?;
            let len = f.metadata().map(|m| m.len() as usize).unwrap_or(0);
            (Box::new(BufReader::new(f)), None, None, None, Some(len.max(4096)))
        }
        3 => {
            let path = read_path.ok_or("Missing variant file path")?;
            let f = std::fs::File::open(path)?;
            let len = f.metadata().map(|m| m.len() as usize).unwrap_or(0);
            (Box::new(BufReader::new(f)), Some(args[2].as_str()), None, None, Some(len.max(4096)))
        }
        4 => {
            let path = read_path.ok_or("Missing variant file path")?;
            let f = std::fs::File::open(path)?;
            let len = f.metadata().map(|m| m.len() as usize).unwrap_or(0);
            let third = args[3].as_str();
            if third.ends_with(".json") && !third.is_empty() {
                (Box::new(BufReader::new(f)), Some(args[2].as_str()), Some(third), None, Some(len.max(4096)))
            } else {
                (Box::new(BufReader::new(f)), Some(args[2].as_str()), None, Some(third), Some(len.max(4096)))
            }
        }
        5 => {
            let path = read_path.ok_or("Missing variant file path")?;
            let f = std::fs::File::open(path)?;
            let len = f.metadata().map(|m| m.len() as usize).unwrap_or(0);
            (Box::new(BufReader::new(f)), Some(args[2].as_str()), Some(args[3].as_str()), Some(args[4].as_str()), Some(len.max(4096)))
        }
        _ => {
            eprintln!("Usage: genetic-report-html [variants.json [output.html [copy_number.json [report_name]]]]");
            eprintln!("  Four args: optional report_name appears in the report title.");
            eprintln!("  When GENETIC_REPORT_INPUT_FILE is set, variants are read from that path only.");
            std::process::exit(1);
        }
    };

    let mut raw = String::new();
    if let Some(cap) = capacity_hint {
        raw.reserve(cap);
    }
    let mut reader = json_input;
    reader.read_to_string(&mut raw)?;
    let variant_file_path_display: Option<&str> = read_path;
    let mut variants: Vec<VariantInput> = serde_json::from_str(&raw)?;
    if variants.is_empty() {
        eprintln!("Error: variant set is empty. Refusing to write a report (wrong or missing input).");
        std::process::exit(1);
    }
    if let Some(path) = variant_file_path_display {
        eprintln!("[report] Reading variants from: {} ({} variants)", path, variants.len());
    } else {
        eprintln!("[report] Reading variants from stdin ({} variants)", variants.len());
    }
    // Alignment marker 1: right after load — verify input path and variant bounds
    let (first_chr, first_pos, last_chr, last_pos) = if variants.is_empty() {
        ("?".to_string(), 0u64, "?".to_string(), 0u64)
    } else {
        let f = &variants[0];
        let l = variants.last().unwrap();
        (f.chromosome.clone(), f.position, l.chromosome.clone(), l.position)
    };
    eprintln!(
        "[ALIGN] 01_LOAD path={} count={} first={}:{} last={}:{}",
        variant_file_path_display.unwrap_or("stdin"),
        variants.len(),
        first_chr,
        first_pos,
        last_chr,
        last_pos
    );

    // Intercept mode: REPORT_INTERCEPT=1 and REPORT_RUN_ID=<id> → write report_intercept_<id>_<stage>.txt at each stage for 1/2 split debugging.
    let intercept_run_id: Option<String> = env::var("REPORT_INTERCEPT").ok()
        .filter(|v| v == "1" || v == "true" || v == "yes")
        .and_then(|_| env::var("REPORT_RUN_ID").ok());
    if let Some(ref run_id) = intercept_run_id {
        eprintln!("[report] INTERCEPT mode: run_id={} — writing fingerprint before each stage", run_id);
    }

    // Data tracker: when REPORT_DATA_TRACKER=1, write 100 sample points at each stage to find where data changes.
    let mut tracker: Option<std::fs::File> = env::var("REPORT_DATA_TRACKER").ok()
        .filter(|v| v == "1" || v == "true" || v == "yes")
        .and_then(|_| {
            let path = format!("report_data_tracker_{}.txt", std::process::id());
            std::fs::File::create(&path).ok().map(|f| {
                eprintln!("[report] Data tracker: writing to {}", path);
                f
            })
        });
    let mut tap_id: u32 = 0;
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "01_after_parse", &variants);
    }
    if let Some(ref mut w) = tracker {
        let _ = writeln!(w, "# REPORT_DATA_TRACKER: tap_id stage count first_last 100_sample_points");
        if let Some(p) = variant_file_path_display {
            let _ = writeln!(w, "# input_file={}", p);
        }
        tap_id += 1;
        tap_data(tap_id, "01_after_parse", &variants, w);
    }

    variants = gene_annotation::annotate_variants_with_genes(variants);
    eprintln!(
        "[ALIGN] 02_AFTER_GENE count={} first={}:{} last={}:{}",
        variants.len(),
        if variants.is_empty() { "?" } else { &variants[0].chromosome },
        if variants.is_empty() { 0 } else { variants[0].position },
        if variants.is_empty() { "?" } else { &variants.last().unwrap().chromosome },
        if variants.is_empty() { 0 } else { variants.last().unwrap().position }
    );
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "02_after_gene_annotation", &variants);
    }
    if let Some(ref mut w) = tracker {
        tap_id += 1;
        tap_data(tap_id, "02_after_gene_annotation", &variants, w);
    }
    // ClinVar: annotate from local index when CLINVAR_INDEX_PATH is set (validates/annotates variants)
    let _clinvar_used = if let Ok(path) = env::var("CLINVAR_INDEX_PATH") {
        if let Some(index) = clinvar_lookup::load_clinvar_index_optional(&path) {
            variants = clinvar_lookup::annotate_variants_with_clinvar(variants, &index);
            if let Some(ref run_id) = intercept_run_id {
                intercept_write(run_id, "03_after_clinvar", &variants);
            }
            if let Some(ref mut w) = tracker {
                tap_id += 1;
                tap_data(tap_id, "03_after_clinvar", &variants, w);
            }
            true
        } else {
            false
        }
    } else {
        false
    };

    eprintln!(
        "[ALIGN] 03_BEFORE_CHECK count={} first={}:{} last={}:{} clinvar={}",
        variants.len(),
        if variants.is_empty() { "?" } else { &variants[0].chromosome },
        if variants.is_empty() { 0 } else { variants[0].position },
        if variants.is_empty() { "?" } else { &variants.last().unwrap().chromosome },
        if variants.is_empty() { 0 } else { variants.last().unwrap().position },
        _clinvar_used
    );
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "04_before_check_variants", &variants);
    }
    if let Some(ref mut w) = tracker {
        tap_id += 1;
        tap_data(tap_id, "04_before_check_variants", &variants, w);
    }
    let report = check_variants_against_all(&variants);
    let inflammation_count: usize = report.inflammation.iter().map(|r| r.findings.len()).sum();
    eprintln!(
        "[ALIGN] 04_REPORT_BUILT title={} kit_d816v={} infl_count={}",
        report_name.unwrap_or("(none)"),
        report.kit_d816v_detected,
        inflammation_count
    );
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "05_after_check_variants", &variants);
    }
    if let Some(ref mut w) = tracker {
        tap_id += 1;
        tap_data(tap_id, "05_after_check_variants_same_slice", &variants, w);
    }
    let cascade_report = cascade::compute_cascade_from_report(&report);
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "06_before_survival", &variants);
    }
    if let Some(ref mut w) = tracker {
        tap_id += 1;
        tap_data(tap_id, "06_before_survival", &variants, w);
    }
    let survival_analysis = survival::analyze_survival(&variants);
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "07_before_mcas_integrated", &variants);
    }
    if let Some(ref mut w) = tracker {
        tap_id += 1;
        tap_data(tap_id, "07_before_mcas_integrated", &variants, w);
    }
    let mcas_integrated_report = mcas_integrated::run_mcas_integrated_analysis(&variants);
    let inflammation_finding_count: usize = report.inflammation.iter().map(|r| r.findings.len()).sum();
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "08_before_exercise_ammonia", &variants);
    }
    if let Some(ref mut w) = tracker {
        tap_id += 1;
        tap_data(tap_id, "08_before_exercise_ammonia", &variants, w);
    }
    let exercise_ammonia_report = exercise_ammonia::run_exercise_ammonia_analysis(&variants, inflammation_finding_count);
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "09_before_star_alleles", &variants);
    }
    if let Some(ref mut w) = tracker {
        tap_id += 1;
        tap_data(tap_id, "09_before_star_alleles", &variants, w);
    }
    let star_allele_results = star_alleles::infer_star_alleles(&variants);
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "10_before_parity", &variants);
    }
    if let Some(ref mut w) = tracker {
        tap_id += 1;
        tap_data(tap_id, "10_before_parity", &variants, w);
    }
    // Optional: verify star alleles against official finder (PharmCAT, StellarPGx). Set OFFICIAL_STAR_ALLELE_JSON to path to JSON.
    let star_allele_verification: Option<Vec<star_alleles::StarAlleleVerificationRow>> =
        env::var("OFFICIAL_STAR_ALLELE_JSON").ok().and_then(|path| {
            star_alleles::load_official_star_alleles_optional(&path).map(|official| {
                star_alleles::build_star_allele_verification(&star_allele_results, &official)
            })
        });
    let star_allele_verification_slice = star_allele_verification.as_deref();
    // Only compare to Sequencing.com when sequencing reports are present (env set by batch script or caller)
    let parity_result = match env::var("INCLUDE_SEQUENCING_PARITY").as_deref() {
        Ok("1") | Ok("true") | Ok("yes") => Some(sequencing_parity::check_sequencing_parity(&variants)),
        _ => None,
    };
    let parity_slice = parity_result.as_ref();
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "11_before_ref_check", &variants);
    }
    if let Some(ref mut w) = tracker {
        tap_id += 1;
        tap_data(tap_id, "11_before_ref_check", &variants, w);
    }
    let ref_check_result = reference_check::run_reference_check(&variants);
    if let Some(ref run_id) = intercept_run_id {
        intercept_write(run_id, "12_final_variants", &variants);
    }
    if let Some(ref mut w) = tracker {
        tap_id += 1;
        tap_data(tap_id, "12_final_variants", &variants, w);
        // Fill to 100 tap points so we have 100 lines to compare across runs
        while tap_id < 100 {
            tap_id += 1;
            tap_data(tap_id, &format!("tap_{:03}", tap_id), &variants, w);
        }
        let _ = writeln!(w, "# total_taps={} each_with_{}_sample_points", tap_id, DATA_TAP_POINTS);
    }
    let copy_number_results = copy_number_path.and_then(|p| load_copy_number(p).ok());
    let copy_number_slice = copy_number_results.as_deref();

    let report_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let report_datetime = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let title = report_name
        .map(|n| format!("{} — Genetic Conditions Report", n.trim()))
        .unwrap_or_else(|| "Genetic Conditions Report (MCAS & related expanded)".to_string());
    // Fingerprint proves which variant set this report was built from (different per genome)
    let dataset_fingerprint = if variants.is_empty() {
        "0 variants".to_string()
    } else {
        let first = &variants[0];
        let last = variants.last().unwrap();
        format!(
            "{} variants • first {}:{} • last {}:{}",
            variants.len(),
            first.chromosome,
            first.position,
            last.chromosome,
            last.position
        )
    };
    let mut section_tracker_file: Option<std::fs::File> = tracker.as_ref().and_then(|_| {
        let path = format!("report_section_tracker_{}.txt", std::process::id());
        std::fs::File::create(&path).ok().map(|f| {
            eprintln!("[report] Section tracker: writing to {}", path);
            f
        })
    });
    let section_tracker = section_tracker_file.as_mut().map(|f| f as &mut dyn std::io::Write);

    if let Some(p) = out_path {
        eprintln!("[report] WILL WRITE TO: {}", p);
        eprintln!("[report] TITLE: {}", title);
        eprintln!("[report] FINGERPRINT: {}", dataset_fingerprint);
        eprintln!("[report] VARIANT COUNT: {}", variants.len());
        eprintln!(
            "[ALIGN] 05_WRITING path={} title={} count={} first={}:{} last={}:{}",
            p,
            title,
            variants.len(),
            if variants.is_empty() { "?" } else { &variants[0].chromosome },
            if variants.is_empty() { 0 } else { variants[0].position },
            if variants.is_empty() { "?" } else { &variants.last().unwrap().chromosome },
            if variants.is_empty() { 0 } else { variants.last().unwrap().position }
        );
    }

    // Plain text output: GENETIC_REPORT_PLAIN_TEXT=1 or output path ends with .txt
    let plain_text = env::var("GENETIC_REPORT_PLAIN_TEXT").as_deref() == Ok("1")
        || out_path.map(|p| p.ends_with(".txt")).unwrap_or(false);
    let output: String = if plain_text {
        eprintln!("[report] PLAIN TEXT MODE — writing .txt (no HTML)");
        {
            let align_first = variants.first().map(|v| format!("{}:{}", v.chromosome, v.position));
            let align_last = variants.last().map(|v| format!("{}:{}", v.chromosome, v.position));
            report_plain_text::report_to_plain_text(
                &report,
                &title,
                &report_date,
                Some(report_datetime.as_str()),
                &dataset_fingerprint,
                _clinvar_used,
                Some(variants.len()),
                align_first.as_deref(),
                align_last.as_deref(),
            )
        }
    } else {
        let simple_test = env::var("GENETIC_REPORT_SIMPLE_TEST").as_deref() == Ok("1");
        if simple_test {
            eprintln!("[report] SIMPLE TEST MODE — writing full MCAS page only (no other tabs)");
            html_report::mcas_only_html(
                &report,
                &title,
                &report_date,
                Some(&report_datetime),
                Some(&dataset_fingerprint),
                copy_number_slice,
            )
        } else {
            html_report::all_conditions_to_html(
                &report,
                &title,
                &report_date,
                Some(&report_datetime),
                Some(&cascade_report),
                Some(&survival_analysis),
                Some(&mcas_integrated_report),
                Some(&exercise_ammonia_report),
                Some(&variants),
                Some(&star_allele_results),
                star_allele_verification_slice,
                parity_slice,
                Some(&ref_check_result),
                copy_number_slice,
                Some(&dataset_fingerprint),
                variant_file_path_display,
                section_tracker,
            )
        }
    };

    if let Some(path) = out_path {
        let pid = std::process::id();
        let temp_path = format!("{}.tmp.{}", path, pid);
        eprintln!("[report] WRITING {} bytes to temp {}", output.len(), temp_path);
        std::fs::write(&temp_path, &output)?;
        std::fs::rename(&temp_path, path)?;
        eprintln!("[report] RENAMED to final path: {}", path);
        if !plain_text {
            let identity_path = format!("{}.identity.txt", path);
            let identity_content = format!(
                "TITLE={}\nFINGERPRINT={}\nVARIANT_COUNT={}\n",
                title,
                dataset_fingerprint,
                variants.len()
            );
            if let Err(e) = std::fs::write(&identity_path, &identity_content) {
                eprintln!("[report] WARN: could not write identity file {}: {}", identity_path, e);
            } else {
                eprintln!("[report] Identity: {}", identity_content.lines().next().unwrap_or(""));
            }
        }
        if env::var("GENETIC_REPORT_WRITE_MCAS_RESULT").as_deref() == Ok("1") {
            let inflammation_count: usize = report.inflammation.iter().map(|r| r.findings.len()).sum();
            let mcas_result_path = format!("{}.mcas_result.txt", path);
            let mcas_result = format!("KIT_D816V={}\nPATHOGENIC_KIT_TPSAB1_COUNT={}\n", if report.kit_d816v_detected { 1 } else { 0 }, inflammation_count);
            if let Err(e) = std::fs::write(&mcas_result_path, &mcas_result) {
                eprintln!("[report] WARN: could not write MCAS result file {}: {}", mcas_result_path, e);
            } else {
                eprintln!("[report] Wrote MCAS result to {}", mcas_result_path);
            }
        }
        let no_open = env::var("BATCH_PRIVACY").as_deref() == Ok("1")
            || env::var("NO_BROWSER_OPEN").as_deref() == Ok("1");
        if !no_open {
            #[cfg(target_os = "macos")]
            let _ = Command::new("open").arg(path).status();
            #[cfg(target_os = "windows")]
            let _ = Command::new("cmd").args(["/C", "start", "", path]).status();
            #[cfg(all(unix, not(target_os = "macos")))]
            let _ = Command::new("xdg-open").arg(path).status();
        }
    } else {
        io::stdout().write_all(output.as_bytes())?;
    }
    Ok(())
}
