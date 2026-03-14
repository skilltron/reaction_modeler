//! Plain text report output. Use until HTML/browser issues are resolved.
//! One report per run; identity and MCAS section clearly at the top.

use crate::AllConditionsReport;
use std::fmt::Write;

/// Format the full conditions report as plain text (identity + MCAS + counts).
/// `clinvar_used`: true if CLINVAR_INDEX_PATH was set and variants were annotated (findings can be > 0).
/// `alignment_*`: when present, print a DATA ALIGNMENT block so you can verify this run's data.
pub fn report_to_plain_text(
    report: &AllConditionsReport,
    report_title: &str,
    report_date: &str,
    report_datetime: Option<&str>,
    dataset_fingerprint: &str,
    clinvar_used: bool,
    alignment_variant_count: Option<usize>,
    alignment_first: Option<&str>,
    alignment_last: Option<&str>,
) -> String {
    let mut out = String::with_capacity(8192);

    writeln!(out, "================================================================================").unwrap();
    writeln!(out, "GENETIC CONDITIONS REPORT — PLAIN TEXT (this report only)").unwrap();
    writeln!(out, "================================================================================").unwrap();
    writeln!(out, "Report: {}", report_title).unwrap();
    writeln!(out, "Dataset: {}", dataset_fingerprint).unwrap();
    writeln!(out, "Report date: {}", report_date).unwrap();
    if let Some(dt) = report_datetime {
        writeln!(out, "Generated: {}", dt).unwrap();
    }
    if clinvar_used {
        writeln!(out, "ClinVar: used (pathogenic/likely pathogenic from index).").unwrap();
    } else {
        writeln!(out, "ClinVar: NOT USED — set CLINVAR_INDEX_PATH so pathogenic/likely pathogenic findings can appear. All section counts may be 0 until then.").unwrap();
    }
    if alignment_variant_count.is_some() || alignment_first.is_some() || alignment_last.is_some() {
        writeln!(out, "--------------------------------------------------------------------------------").unwrap();
        writeln!(out, "DATA ALIGNMENT (verify this run — should match Report/Dataset above)").unwrap();
        writeln!(out, "  ALIGN Report: {}", report_title).unwrap();
        writeln!(out, "  ALIGN Dataset: {}", dataset_fingerprint).unwrap();
        if let Some(n) = alignment_variant_count {
            writeln!(
                out,
                "  ALIGN Variant count: {}  First: {}  Last: {}",
                n,
                alignment_first.unwrap_or("—"),
                alignment_last.unwrap_or("—")
            )
            .unwrap();
        }
        writeln!(out, "--------------------------------------------------------------------------------").unwrap();
    }
    writeln!(out, "--------------------------------------------------------------------------------").unwrap();

    writeln!(out, "MCAS / MAST CELL–RELATED").unwrap();
    writeln!(out, "  KIT D816V in this dataset: {}", if report.kit_d816v_detected { "DETECTED" } else { "not detected" }).unwrap();
    let inflammation_count: usize = report.inflammation.iter().map(|r| r.findings.len()).sum();
    writeln!(out, "  MCAS/mastocytosis variant findings (pathogenic/likely pathogenic): {}", inflammation_count).unwrap();
    if inflammation_count > 0 {
        for r in &report.inflammation {
            for f in &r.findings {
                let ref_alt = match (&f.reference_allele, &f.alternate_allele) {
                    (Some(ref_a), Some(alt_a)) => format!(" {} -> {}", ref_a, alt_a),
                    _ => String::new(),
                };
                let note_suffix = if f.note.is_empty() { "" } else { " | " };
                let note_part = if f.note.is_empty() { "" } else { f.note.as_str() };
                writeln!(
                    out,
                    "    - {} | {} | {} {}:{}{}{}",
                    f.gene,
                    r.condition_name,
                    f.variant.chromosome,
                    f.variant.position,
                    ref_alt,
                    note_suffix,
                    note_part
                )
                .unwrap();
            }
        }
    } else if !report.kit_d816v_detected {
        writeln!(out, "  No matching variants in KIT or TPSAB1 (reference/normal for this run).").unwrap();
    }
    writeln!(out, "--------------------------------------------------------------------------------").unwrap();

    let immune_count: usize = report.immune.iter().map(|r| r.findings.len()).sum();
    let exposure_count: usize = report.exposure.iter().map(|r| r.findings.len()).sum();
    let sulfur_count: usize = report.sulfur.iter().map(|r| r.findings.len()).sum();
    let rare_count: usize = report.rare.iter().map(|r| r.findings.len()).sum();
    let cancer_count: usize = report.cancer.iter().map(|r| r.findings.len()).sum();
    let disorders_count: usize = report.disorders.iter().map(|r| r.findings.len()).sum();

    writeln!(out, "OTHER SECTIONS (counts only)").unwrap();
    writeln!(out, "  Immune: {} | Exposure: {} | Sulfur: {} | Rare: {} | Cancer: {} | Disorders: {}", immune_count, exposure_count, sulfur_count, rare_count, cancer_count, disorders_count).unwrap();
    writeln!(out, "================================================================================").unwrap();
    writeln!(out, "For research and educational use only. Not for clinical diagnosis.").unwrap();
    writeln!(out, "================================================================================").unwrap();

    out
}
