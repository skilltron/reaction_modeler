//! HTML report generator: same style as Enhanced psychiatric report, with expanded MCAS and related conditions.
//! For research and educational use only; not for clinical diagnosis.

use crate::cascade;
use crate::inflammation::{self, InflammationReport};
use crate::pharmacopoeia::{self, EnzymeRole};
use crate::reference_check::ReferenceCheckResult;
use crate::sequencing_parity::SequencingParityResult;
use crate::star_alleles::{StarAlleleGeneResult, StarAlleleVerificationRow, star_allele_legend};
use crate::supplements;
use crate::survival;
use crate::copy_number::{self, CopyNumberResult};
use crate::AllConditionsReport;
use crate::VariantInput;
use std::fmt::Write;
use std::io::Write as IoWrite;

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            _ => out.push(c),
        }
    }
    out
}

/// Format zygosity from genotype (e.g. 0/1 → Heterozygous, 1/1 → Homozygous alternate).
/// When genotype is missing, returns a short reason so doctors see either a value or "does not apply".
fn format_zygosity(genotype: Option<&str>) -> String {
    match genotype.map(|s| s.trim()) {
        None | Some("") | Some(".") => "Not in data — genotype (GT) not reported in this VCF/source; ask lab for genotype if needed.".to_string(),
        Some("./.") | Some(".|.") => "No call — assay did not report genotype at this position.".to_string(),
        Some("0/0") => "Reference (homozygous ref)".to_string(),
        Some("0|0") => "Reference (homozygous ref)".to_string(),
        Some("1/1") | Some("1|1") => "Homozygous alternate".to_string(),
        Some("0/1") | Some("0|1") | Some("1/0") | Some("1|0") => "Heterozygous".to_string(),
        Some("1/2") | Some("1|2") | Some("2/1") | Some("2|1") => "Heterozygous (two alternates)".to_string(),
        Some(gt) if gt.contains('/') || gt.contains('|') => format!("Genotype: {}", gt),
        Some(gt) if gt.len() <= 12 => gt.to_string(),
        Some(gt) => format!("Genotype: {}…", &gt[..12.min(gt.len())]),
    }
}

/// Format "allele present" from genotype and ref/alt (e.g. 0/0 → GG, 1/1 → AA, 0/1 → G/A).
/// When genotype is missing, returns a short reason so the report never shows a bare "Not reported" without explanation.
fn allele_present_display(v: &VariantInput) -> String {
    let ref_a = v.ref_allele.as_deref().unwrap_or("?");
    let alt_a = v.alt_allele.as_deref().unwrap_or("?");
    match v.genotype.as_deref().map(|s| s.trim()) {
        None | Some("") | Some(".") => "— (genotype not in this dataset)".to_string(),
        Some("./.") | Some(".|.") => "— (no call at this position)".to_string(),
        Some("0/0") | Some("0|0") => format!("{}{}", ref_a, ref_a),
        Some("1/1") | Some("1|1") => format!("{}{}", alt_a, alt_a),
        Some("0/1") | Some("0|1") | Some("1/0") | Some("1|0") => format!("{}/{}", ref_a, alt_a),
        Some("1/2") | Some("1|2") | Some("2/1") | Some("2|1") => format!("{}/{} (two alternates)", ref_a, alt_a),
        Some(gt) if gt.len() <= 16 => gt.to_string(),
        Some(gt) => format!("{}…", &gt[..12.min(gt.len())]),
    }
}

/// Emit a short line for report: Zygosity and Confidence for every variant/finding.
/// Confidence: from pipeline when available; "not provided" when absent.
fn variant_zygosity_confidence_html(v: &VariantInput) -> String {
    let zyg = format_zygosity(v.genotype.as_deref());
    let conf = v
        .confidence
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("not provided");
    format!(
        "<p><strong>Zygosity:</strong> {} &nbsp; <strong>Confidence:</strong> {}</p>\n",
        escape(&zyg),
        escape(conf)
    )
}

/// One block for a variant finding: Allele present, Zygosity, Effect (note), Confidence.
fn variant_finding_display_html(v: &VariantInput, effect_note: &str) -> String {
    let allele = allele_present_display(v);
    let zyg = format_zygosity(v.genotype.as_deref());
    let conf = v
        .confidence
        .as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or("not provided");
    format!(
        "<p><strong>Allele present:</strong> {} &nbsp; <strong>Zygosity:</strong> {} &nbsp; <strong>Effect:</strong> {} &nbsp; <strong>Confidence:</strong> {}</p>\n",
        escape(&allele),
        escape(&zyg),
        escape(effect_note),
        escape(conf)
    )
}

const REPORT_CSS: &str = r#"
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body { font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif; line-height: 1.6; color: #333; background: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        header { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; border-radius: 10px; margin-bottom: 30px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); }
        header h1 { font-size: 2em; margin-bottom: 10px; }
        .patient-info { display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 15px; margin-top: 20px; }
        .info-card { background: rgba(255,255,255,0.2); padding: 15px; border-radius: 8px; }
        .nav-tabs { display: flex; gap: 10px; margin-bottom: 20px; flex-wrap: wrap; }
        .nav-tab { padding: 12px 24px; background: white; border: 2px solid #667eea; border-radius: 8px; cursor: pointer; transition: all 0.3s; font-weight: 600; }
        .nav-tab:hover { background: #667eea; color: white; }
        .nav-tab.active { background: #667eea; color: white; }
        .content-section { display: none; background: white; padding: 30px; border-radius: 10px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-bottom: 20px; }
        .content-section.active { display: block; }
        .alert { padding: 20px; border-radius: 8px; margin-bottom: 20px; border-left: 5px solid; }
        .alert-info { background: #d1ecf1; border-color: #0c5460; color: #0c5460; }
        .medication-card, .condition-card { background: #f8f9fa; border: 2px solid #dee2e6; border-radius: 8px; padding: 20px; margin-bottom: 20px; }
        .medication-card h3, .condition-card h3 { color: #667eea; margin-bottom: 15px; }
        .gene-variant { background: white; border-left: 4px solid #667eea; padding: 15px; margin: 10px 0; border-radius: 4px; }
        .gene-variant h4 { color: #667eea; margin-bottom: 10px; }
        .recommendation-box { background: #e7f3ff; border-left: 4px solid #2196F3; padding: 15px; margin: 15px 0; border-radius: 4px; }
        .print-button { position: fixed; bottom: 20px; right: 20px; background: #667eea; color: white; border: none; padding: 15px 25px; border-radius: 50px; cursor: pointer; box-shadow: 0 4px 6px rgba(0,0,0,0.3); font-size: 1em; font-weight: 600; }
        .print-button:hover { background: #5568d3; }
        @media print { .nav-tabs, .print-button { display: none; } .content-section { display: block !important; page-break-inside: avoid; } }
"#;

/// Standalone MCAS page only (full MCAS section as one HTML document). Used for GENETIC_REPORT_SIMPLE_TEST=1.
pub fn mcas_only_html(
    report: &AllConditionsReport,
    report_title: &str,
    report_date: &str,
    report_datetime: Option<&str>,
    dataset_fingerprint: Option<&str>,
    copy_number: Option<&[CopyNumberResult]>,
) -> String {
    let mcas_refs = inflammation::mcas_mastocytosis_ref();
    let mut out = String::with_capacity(32 * 1024);
    out.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n<title>");
    out.push_str(&escape(report_title));
    out.push_str(" — MCAS</title>\n<style>");
    out.push_str(REPORT_CSS);
    out.push_str("</style>\n</head>\n<body>\n");
    out.push_str("<div id=\"report-identity-banner\" style=\"background:#0c5460;color:#fff;padding:10px 20px;margin:0;font-size:1.05em;font-weight:600;\"><strong>This report only:</strong> ");
    out.push_str(&escape(report_title));
    out.push_str(" — ");
    out.push_str(&escape(dataset_fingerprint.unwrap_or("(none)")));
    out.push_str("</div>\n");
    out.push_str("<div class=\"container\">\n<header>\n<h1>MCAS and mast cell–related (this page only)</h1>\n<p>");
    out.push_str(&escape(report_title));
    out.push_str("</p>\n<p><strong>Report date:</strong> ");
    out.push_str(&escape(report_date));
    out.push_str("</p>\n");
    if let Some(dt) = report_datetime {
        out.push_str("<p><strong>Generated:</strong> ");
        out.push_str(&escape(dt));
        out.push_str("</p>\n");
    }
    if let Some(fp) = dataset_fingerprint {
        out.push_str("<p><strong>Dataset:</strong> ");
        out.push_str(&escape(fp));
        out.push_str("</p>\n");
    }
    out.push_str("</header>\n\n<div id=\"mcas\" class=\"content-section active\">\n<h2>MCAS and mast cell–related conditions</h2>\n");
    // Identity banner
    let inflammation_count: usize = report.inflammation.iter().map(|r| r.findings.len()).sum();
    out.push_str("<div class=\"recommendation-box\" style=\"border: 3px solid #0c5460; background: #e8f4f8; margin-bottom: 1.2em; padding: 1em;\">\n<h3 style=\"margin-top: 0; color: #0c5460;\">This report only</h3>\n<p><strong>Report:</strong> ");
    out.push_str(&escape(report_title));
    out.push_str("</p>\n<p><strong>Dataset for this report:</strong> ");
    out.push_str(&escape(dataset_fingerprint.unwrap_or("(none)")));
    out.push_str("</p>\n<p><strong>KIT D816V in this dataset:</strong> ");
    if report.kit_d816v_detected {
        out.push_str("detected.");
    } else {
        out.push_str("not detected.");
    }
    out.push_str("</p>\n<p><strong>MCAS/mastocytosis variant findings:</strong> ");
    write!(out, "{}", inflammation_count).unwrap();
    out.push_str(" (pathogenic/likely pathogenic only).</p>\n</div>\n");
    out.push_str("<div class=\"recommendation-box\" style=\"border-left: 4px solid #0c5460; margin-bottom: 1em;\">\n<h3>This run: your data was checked</h3>\n<p>");
    if report.kit_d816v_detected {
        out.push_str("<strong>KIT D816V was detected</strong>. ");
    }
    if inflammation_count > 0 {
        write!(out, "<strong>{} variant finding(s)</strong> in KIT/TPSAB1. ", inflammation_count).unwrap();
    } else if !report.kit_d816v_detected {
        out.push_str("<strong>No matching variants</strong> in KIT or TPSAB1 — your result is reference (normal) for these genes. ");
    }
    out.push_str("Cards below are reference information.</p>\n</div>\n");
    if report.kit_d816v_detected {
        out.push_str("<div class=\"recommendation-box\" style=\"border-left: 4px solid #dc3545;\">\n<h3>Problem — KIT D816V detected</h3>\n<p>Associated with systemic mastocytosis. Discuss with your clinician.</p>\n</div>\n");
    }
    let has_inflammation_findings = report.inflammation.iter().any(|r| !r.findings.is_empty());
    if has_inflammation_findings {
        out.push_str("<div class=\"recommendation-box\" style=\"border-left: 4px solid #e07c3c;\">\n<h3>Concern — variant(s) in MCAS/mastocytosis genes</h3>\n");
        for r in &report.inflammation {
            for f in &r.findings {
                let mut effect = f.note.clone();
                if let (Some(ref_a), Some(alt_a)) = (&f.reference_allele, &f.alternate_allele) {
                    effect.push_str(&format!(" Ref: {} → Alt: {}.", ref_a, alt_a));
                }
                out.push_str("<div class=\"gene-variant\">\n<h4>");
                out.push_str(&escape(&f.gene));
                out.push_str(" — ");
                out.push_str(&escape(&r.condition_name));
                out.push_str("</h4>\n");
                out.push_str(&variant_finding_display_html(&f.variant, &effect));
                out.push_str("</div>\n");
            }
        }
        out.push_str("</div>\n");
    }
    out.push_str("<div class=\"recommendation-box\" style=\"border-left: 4px solid #e07c3c;\">\n<h3>Minor criteria (SM)</h3>\n<ul>\n");
    out.push_str("<li>High tryptase — lab.</li>\n<li>CD25 on mast cells — pathology.</li>\n");
    if report.kit_d816v_detected {
        out.push_str("<li><strong>KIT D816V</strong> — detected (this report).</li>\n");
    } else {
        out.push_str("<li><strong>KIT D816V</strong> — not detected.</li>\n");
    }
    out.push_str("<li>Abnormal mast cell count — pathology.</li>\n</ul>\n</div>\n");
    out.push_str("<h3 style=\"margin-top: 1.5em;\">Reference: what each condition is</h3>\n");
    for r in &mcas_refs {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.name));
        out.push_str("</h3>\n<p>");
        out.push_str(&escape(&r.description));
        out.push_str("</p>\n<p><strong>Genes:</strong> ");
        out.push_str(&escape(&r.genes.join(", ")));
        out.push_str("</p>\n<p><strong>Biomarkers:</strong> ");
        out.push_str(&escape(&r.biomarkers.join("; ")));
        out.push_str("</p>\n");
        if r.name.contains("alpha-tryptasemia") || r.name.contains("HαT") {
            if let Some(cn) = copy_number {
                if let Some(n) = copy_number::tpsab1_copy_number(cn) {
                    if copy_number::tpsab1_gain_detected(cn) {
                        write!(out, "<p>TPSAB1 copy number {} (gain).</p>", n).unwrap();
                    } else {
                        write!(out, "<p>TPSAB1 copy number {} (no gain).</p>", n).unwrap();
                    }
                }
            } else {
                out.push_str("<p>HαT requires copy number assay; not from SNV data.</p>\n");
            }
        }
        out.push_str("</div>\n");
    }
    let stabilizers = inflammation::mcas_stabilizers_ref();
    let combo = inflammation::mcas_recommended_combo_with_cromolyn();
    out.push_str("<h2>MCAS stabilizers</h2>\n");
    for s in &stabilizers {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&s.name));
        out.push_str("</h3>\n<p>");
        out.push_str(&escape(&s.benefit_rationale));
        out.push_str("</p>\n</div>\n");
    }
    out.push_str("<div class=\"recommendation-box\">\n<h3>Suggested combo (includes cromolyn)</h3>\n<ol>");
    for name in &combo {
        out.push_str("\n<li>");
        out.push_str(&escape(name));
        out.push_str("</li>");
    }
    out.push_str("\n</ol>\n</div>\n</div>\n</div>\n</body>\n</html>");
    out
}

/// Build full HTML report including expanded MCAS and MCAS-related conditions (PMC8540348, SLK/Theodore's).
/// Optionally include cascade, survival, MCAS integrated analysis, ClinVar, star allele inference.
/// dataset_fingerprint: short string proving which variant set this report was built from (e.g. "45,231 variants • first chr1:12345 • last chrX:98765"). Used to verify each report reflects the correct genome.
/// section_tracker: when Some (e.g. REPORT_DATA_TRACKER=1), write "section\tid\tdata_fingerprint" at the start of each section to trace which data each section uses.
pub fn all_conditions_to_html(
    report: &AllConditionsReport,
    report_title: &str,
    report_date: &str,
    report_datetime: Option<&str>,
    cascade_report: Option<&cascade::IntegratedCascadeReport>,
    survival_analysis: Option<&survival::SurvivalAnalysis>,
    mcas_integrated: Option<&crate::mcas_integrated::McasIntegratedReport>,
    exercise_ammonia: Option<&crate::exercise_ammonia::ExerciseAmmoniaReport>,
    variants_with_clinvar: Option<&[VariantInput]>,
    star_alleles: Option<&[StarAlleleGeneResult]>,
    star_allele_verification: Option<&[StarAlleleVerificationRow]>,
    sequencing_parity: Option<&SequencingParityResult>,
    reference_check: Option<&ReferenceCheckResult>,
    copy_number: Option<&[CopyNumberResult]>,
    dataset_fingerprint: Option<&str>,
    variant_input_path: Option<&str>,
    mut section_tracker: Option<&mut dyn IoWrite>,
) -> String {
    let mcas_refs = inflammation::mcas_mastocytosis_ref();
    let slk_ref = inflammation::slk_theodores_ref();

    let mut tap = |section: &str, fp: &str| {
        if let Some(w) = section_tracker.as_mut() {
            let _ = writeln!(w, "section\t{}\t{}", section, fp);
        }
    };

    let mut out = String::with_capacity(64 * 1024);
    out.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n<meta http-equiv=\"Cache-Control\" content=\"no-cache, no-store, must-revalidate\">\n<meta http-equiv=\"Pragma\" content=\"no-cache\">\n<meta http-equiv=\"Expires\" content=\"0\">\n<title>");
    out.push_str(&escape(report_title));
    out.push_str("</title>\n<style>");
    out.push_str(REPORT_CSS);
    out.push_str("</style>\n</head>\n<body>\n");
    // Unambiguous run identity so this file cannot be confused with another report (no shared/cached content).
    out.push_str("<!-- REPORT_IDENTITY:");
    out.push_str(&escape(report_title));
    out.push_str(" | ");
    out.push_str(&escape(dataset_fingerprint.unwrap_or("(no fingerprint)")));
    out.push_str(" -->\n");
    out.push_str("<div id=\"report-identity-banner\" style=\"background:#0c5460;color:#fff;padding:10px 20px;margin:0;font-size:1.05em;font-weight:600;\"><strong>This report only:</strong> ");
    out.push_str(&escape(report_title));
    out.push_str(" — ");
    out.push_str(&escape(dataset_fingerprint.unwrap_or("(no fingerprint)")));
    out.push_str("</div>\n");
    out.push_str("<div class=\"container\">\n<header>\n<h1>Genetic Conditions Report</h1>\n<p style=\"opacity: 0.9;\">");
    out.push_str(&escape(report_title));
    out.push_str("</p>\n<div class=\"patient-info\">\n<div class=\"info-card\"><strong>Report date:</strong> ");
    out.push_str(&escape(report_date));
    out.push_str("</div>\n<div class=\"info-card\"><strong>Disclaimer:</strong> For research and educational use only. Not for clinical diagnosis.</div>\n<div class=\"info-card\"><strong>Privacy:</strong> This report was generated entirely on your device. No genetic data (VCF, variants, or report content) was sent to any server or third party.</div>\n");
    if let Some(dt) = report_datetime {
        out.push_str("<div class=\"info-card\" style=\"border: 1px solid #28a745; background: rgba(40,167,69,0.08);\"><strong>Generated at:</strong> ");
        out.push_str(&escape(dt));
        out.push_str(" (this run only; no cached content)</div>\n");
    }
    if let Some(path_used) = variant_input_path {
        out.push_str("<div class=\"info-card\" style=\"border: 2px solid #0c5460; background: rgba(12,84,96,0.12);\"><strong>Variant file used (this report only):</strong> ");
        out.push_str(&escape(path_used));
        out.push_str("</div>\n");
    }
    if let Some(fp) = dataset_fingerprint {
        out.push_str("<div class=\"info-card\"><strong>Dataset:</strong> ");
        out.push_str(&escape(fp));
        out.push_str(" (verifies this report is from this genome only)</div>\n");
        out.push_str("<div class=\"info-card\" style=\"border: 1px solid #0c5460; background: rgba(12,84,96,0.08);\"><strong>All results below</strong> — conditions, star alleles, cascade — are computed from <em>this</em> variant set only. Nothing is pulled from any other run or dataset.</div>\n");
    }
    tap("00_header", &format!("title={} fp={}", report_title, dataset_fingerprint.unwrap_or("none")));
    if copy_number.is_some() && !copy_number.map(|c| c.is_empty()).unwrap_or(true) {
        out.push_str("<div class=\"info-card\"><strong>Copy number assay:</strong> Included. Used for HαT (TPSAB1) and other copy-number–dependent conditions where applicable.</div>\n");
    }
    out.push_str("</div>\n</header>\n\n<div class=\"nav-tabs\">\n");
    write!(out, r#"<div class="nav-tab active" onclick="showSection('mcas')">🩺 MCAS &amp; related</div>"#).unwrap();
    out.push_str("\n<div class=\"nav-tab\" onclick=\"showSection('inflammation')\">Inflammation findings</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('immune')\">Immune</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('exposure')\">Exposure</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('sulfur')\">Sulfur</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('rare')\">Rare</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('cancer')\">Cancer screening</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('disorders')\">Disorders</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('supplements')\">Supplements</div>\n");
    if cascade_report.is_some() {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('cascade')\">Cascade</div>\n");
    }
    if mcas_integrated.is_some() {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('mcas-integrated')\">MCAS Integrated</div>\n");
    }
    if exercise_ammonia.is_some() {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('exercise-ammonia')\">Exercise Ammonia</div>\n");
    }
    if survival_analysis.is_some() {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('survival')\">Survival</div>\n");
    }
    if variants_with_clinvar.map(|v| v.iter().any(|x| x.clinvar.is_some())).unwrap_or(false) {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('clinvar')\">ClinVar</div>\n");
    }
    if star_alleles.is_some() {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('staralleles')\">Star alleles</div>\n");
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('pharmacopoeia')\">Drug interactions</div>\n");
    }
    if sequencing_parity.is_some() {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('parity')\">Sequencing.com parity</div>\n");
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('beyond')\">Beyond Sequencing.com</div>\n");
    }
    let has_immune = report.immune.iter().any(|r| !r.findings.is_empty());
    let has_exposure = report.exposure.iter().any(|r| !r.findings.is_empty());
    let has_sulfur = report.sulfur.iter().any(|r| !r.findings.is_empty());
    let has_rare = report.rare.iter().any(|r| !r.findings.is_empty());
    let has_cancer = report.cancer.iter().any(|r| !r.findings.is_empty());
    let has_disorders = report.disorders.iter().any(|r| !r.findings.is_empty());
    let has_inflammation = report.inflammation.iter().any(|r| !r.findings.is_empty());
    let any_no_match = !has_immune || !has_exposure || !has_sulfur || !has_rare || !has_cancer || !has_disorders || !has_inflammation;
    if any_no_match {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('no-matching')\">No matching variants</div>\n");
    }
    if reference_check.is_some() {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('reference-check')\">Reference check</div>\n");
    }
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('disclaimer')\">Disclaimer</div>\n</div>\n\n");

    // At a glance: normal (reference) vs may need attention — so it's clear if you have a problem or not
    out.push_str("<div class=\"recommendation-box\" style=\"border-left: 4px solid #28a745; margin-bottom: 20px;\">\n<h3 style=\"margin-top: 0;\">Reading this report: normal vs may need attention</h3>\n");
    out.push_str("<p><strong>Normal (reference)</strong> means no variant or phenotype of concern was detected for that item — your result is in the typical range and no action is usually needed for that finding.</p>\n");
    out.push_str("<p><strong>May need attention / discuss with clinician</strong> means a finding was detected (e.g. reduced or loss of function for a drug enzyme, variant in a condition gene, or a clearance need in your cascade). Share that section with your prescriber; it does not mean you definitely have a problem — it means it’s worth discussing.</p>\n");
    out.push_str("<p>Each tab and each finding card below will state whether the result is <em>reference/normal</em> or <em>discuss with your clinician</em>. Use this as your guide.</p>\n</div>\n\n");

    // No matching variants: group all categories that have zero findings
    if any_no_match {
        out.push_str("<!-- No matching variants -->\n<div id=\"no-matching\" class=\"content-section\">\n<h2>No matching variants</h2>\n");
        out.push_str("<p>The following condition groups had <strong>no matching variants</strong> in this run (not found in your variant set). They are listed here so you can see what was checked.</p>\n<ul style=\"margin: 1em 0; line-height: 1.8;\">\n");
        if !has_inflammation {
            let names: Vec<&str> = report.inflammation.iter().map(|r| r.condition_name.as_str()).collect();
            out.push_str("<li><strong>Inflammation (MCAS / mastocytosis):</strong> ");
            out.push_str(&escape(&names.join("; ")));
            out.push_str("</li>\n");
        }
        if !has_immune {
            let names: Vec<&str> = report.immune.iter().map(|r| r.disease_name.as_str()).collect();
            out.push_str("<li><strong>Immune:</strong> ");
            out.push_str(&escape(&names.join("; ")));
            out.push_str("</li>\n");
        }
        if !has_exposure {
            let names: Vec<&str> = report.exposure.iter().map(|r| r.chemical_name.as_str()).collect();
            out.push_str("<li><strong>Chemical exposure:</strong> ");
            out.push_str(&escape(&names.join("; ")));
            out.push_str("</li>\n");
        }
        if !has_sulfur {
            let names: Vec<&str> = report.sulfur.iter().map(|r| r.condition_name.as_str()).collect();
            out.push_str("<li><strong>Sulfur metabolism:</strong> ");
            out.push_str(&escape(&names.join("; ")));
            out.push_str("</li>\n");
        }
        if !has_rare {
            let names: Vec<&str> = report.rare.iter().map(|r| r.disease_name.as_str()).collect();
            out.push_str("<li><strong>Rare disease:</strong> ");
            out.push_str(&escape(&names.join("; ")));
            out.push_str("</li>\n");
        }
        if !has_cancer {
            let names: Vec<&str> = report.cancer.iter().map(|r| r.syndrome_name.as_str()).collect();
            out.push_str("<li><strong>Cancer screening (hereditary syndromes):</strong> ");
            out.push_str(&escape(&names.join("; ")));
            out.push_str("</li>\n");
        }
        if !has_disorders {
            let names: Vec<&str> = report.disorders.iter().map(|r| r.disorder_name.as_str()).collect();
            out.push_str("<li><strong>Disorders (susceptibility):</strong> ");
            out.push_str(&escape(&names.join("; ")));
            out.push_str("</li>\n");
        }
        out.push_str("</ul>\n<p>Absence of a match does not rule out a condition; it means no variant in your file matched the genes we check for that category. For research and education only.</p>\n</div>\n\n");
    }

    // Reference check: marker-based cross-check to detect build drift
    if let Some(ref rc) = reference_check {
        out.push_str("<!-- Reference check -->\n<div id=\"reference-check\" class=\"content-section\">\n<h2>Reference check (data alignment)</h2>\n");
        out.push_str("<p>Well-known marker variants (MTHFR, APOE, F5, BRCA1, BRCA2) are checked against expected positions for GRCh37 and GRCh38. If positions drift, we can detect and flag it.</p>\n");
        out.push_str("<div class=\"recommendation-box\" style=\"border-left-color: #0c5460;\">\n");
        write!(out, "<p><strong>Inferred build:</strong> {}</p>\n", escape(&rc.inferred_build)).unwrap();
        out.push_str("<p>");
        out.push_str(&escape(&rc.summary));
        out.push_str("</p>\n<p><strong>Recommendation:</strong> ");
        out.push_str(&escape(&rc.recommendation));
        out.push_str("</p>\n</div>\n");
        out.push_str("<h3>Marker results</h3>\n<table style=\"border-collapse: collapse; width: 100%; margin-top: 10px;\">\n");
        out.push_str("<thead><tr style=\"background: #dee2e6;\"><th style=\"padding: 8px; border: 1px solid #ddd;\">rsID</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Gene</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Observed (chr:pos)</th><th style=\"padding: 8px; border: 1px solid #ddd;\">In data</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Matches GRCh37</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Matches GRCh38</th></tr></thead>\n<tbody>\n");
        for m in &rc.markers {
            let obs = if m.not_found {
                "—".to_string()
            } else {
                format!("{}:{}", m.observed_chromosome, m.observed_position)
            };
            let in_data = if m.not_found { "No" } else { "Yes" };
            let m37 = if m.not_found { "—" } else if m.matches_grch37 { "Yes" } else { "No" };
            let m38 = if m.not_found { "—" } else if m.matches_grch38 { "Yes" } else { "No" };
            write!(out, "<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">{}</td><td style=\"padding: 8px; border: 1px solid #ddd;\">{}</td><td style=\"padding: 8px; border: 1px solid #ddd;\">{}</td><td style=\"padding: 8px; border: 1px solid #ddd;\">{}</td><td style=\"padding: 8px; border: 1px solid #ddd;\">{}</td><td style=\"padding: 8px; border: 1px solid #ddd;\">{}</td></tr>\n",
                escape(&m.rsid), escape(&m.gene), escape(&obs), in_data, m37, m38).unwrap();
        }
        out.push_str("</tbody>\n</table>\n<p style=\"margin-top: 1em; font-size: 0.9em;\">If positions do not match either build, confirm the reference genome used to produce your VCF and consider re-alignment.</p>\n</div>\n\n");
    }

    // MCAS & related (active) — worst first: BANNER with report identity and this run's result, then Problem/Concern, then reference content
    let inflammation_count: usize = report.inflammation.iter().map(|r| r.findings.len()).sum();
    let mcas_first = report.inflammation.iter().flat_map(|r| r.findings.iter()).next()
        .map(|f| format!("{}:{}", f.variant.chromosome, f.variant.position))
        .unwrap_or_else(|| "none".to_string());
    tap("01_mcas", &format!("infl_count={} first={}", inflammation_count, mcas_first));
    out.push_str("<!-- MCAS & related -->\n<div id=\"mcas\" class=\"content-section active\">\n<h2>MCAS and mast cell–related conditions (expanded reference)</h2>\n");
    // UNMISTAKABLE: This report's identity and dataset so MCAS is never confused with another person's. Rendered first so every report differs.
    out.push_str("<div class=\"recommendation-box\" style=\"border: 3px solid #0c5460; background: #e8f4f8; margin-bottom: 1.2em; padding: 1em;\">\n<h3 style=\"margin-top: 0; color: #0c5460;\">This report only — not shared with any other</h3>\n<p style=\"font-size: 1.05em;\"><strong>Report:</strong> ");
    out.push_str(&escape(report_title));
    out.push_str("</p>\n<p><strong>Dataset for this report:</strong> ");
    out.push_str(&escape(dataset_fingerprint.unwrap_or("(none)")));
    out.push_str("</p>\n<p><strong>KIT D816V in this dataset:</strong> ");
    if report.kit_d816v_detected {
        out.push_str("detected (see Problem below).");
    } else {
        out.push_str("not detected.");
    }
    out.push_str("</p>\n<p><strong>MCAS/mastocytosis variant findings in this dataset:</strong> ");
    write!(out, "{}", inflammation_count).unwrap();
    out.push_str(" (pathogenic/likely pathogenic only).</p>\n</div>\n");
    // Make it obvious we processed this genome: state result for this run before any reference cards
    out.push_str("<div class=\"recommendation-box\" style=\"border-left: 4px solid #0c5460; margin-bottom: 1em;\">\n<h3 style=\"margin-top: 0;\">This run: your data was checked</h3>\n<p>This report was generated from <strong>this genome only</strong> (see box above). For MCAS and mast cell–related genes (KIT, TPSAB1): ");
    if report.kit_d816v_detected {
        out.push_str("<strong>KIT D816V was detected</strong> in your variant set (see Problem box below). ");
    }
    if inflammation_count > 0 {
        write!(out, "<strong>{} variant finding(s)</strong> in these genes (see Concern box below). ", inflammation_count).unwrap();
    } else if !report.kit_d816v_detected {
        out.push_str("<strong>No matching variants</strong> in KIT or TPSAB1 in this dataset — the cards below are reference only; your result for this run is reference (normal) for these genes. ");
    }
    out.push_str("The condition cards further down are <em>reference information</em> (what each disorder is); your personal result is stated above and in the Minor criteria list.</p>\n</div>\n");
    out.push_str("<p><strong>Confidence / evidence:</strong> Variant-level <strong>Confidence</strong> is shown for each finding when provided by the pipeline (e.g. High, Medium, Low); otherwise \"not provided\". Section-level evidence: reference from literature (PMC8540348, GeneReviews). For research only.</p>\n");
    out.push_str("<p>Below, the <strong>worst</strong> (problem/concern) is shown first; reference (normal) follows.</p>\n");

    // 1. Worst first: KIT D816V detected = Problem
    if report.kit_d816v_detected {
        out.push_str("<div class=\"recommendation-box\" style=\"border-left: 4px solid #dc3545;\">\n<h3 style=\"margin-top: 0;\">Problem — KIT D816V detected</h3>\n");
        out.push_str("<p>This report detected the <strong>KIT D816V</strong> mutation in your variant set. It is associated with systemic mastocytosis. Discuss interpretation and any next steps with your clinician.</p>\n</div>\n");
    }

    // 2. Variant findings in KIT/TPSAB1 (Concern) — before reference content
    let has_inflammation_findings = report.inflammation.iter().any(|r| !r.findings.is_empty());
    if has_inflammation_findings {
        out.push_str("<div class=\"recommendation-box\" style=\"border-left: 4px solid #e07c3c;\">\n<h3 style=\"margin-top: 0;\">Concern — variant(s) in MCAS/mastocytosis genes (KIT, TPSAB1)</h3>\n");
        for r in &report.inflammation {
            for f in &r.findings {
                let mut effect = f.note.clone();
                if let (Some(ref_a), Some(alt_a)) = (&f.reference_allele, &f.alternate_allele) {
                    effect.push_str(&format!(" Ref: {} → Alt: {}.", ref_a, alt_a));
                }
                if let Some(rt) = &f.region_type {
                    effect.push_str(&format!(" Region: {}.", rt.as_str()));
                }
                out.push_str("<div class=\"gene-variant\">\n<h4>");
                out.push_str(&escape(&f.gene));
                out.push_str(" — ");
                out.push_str(&escape(&r.condition_name));
                out.push_str("</h4>\n");
                out.push_str(&variant_finding_display_html(&f.variant, &effect));
                out.push_str("</div>\n");
            }
        }
        out.push_str("</div>\n");
    }

    // 3. Minor criteria (KIT D816V result + others); then reference condition cards and stabilizers
    out.push_str("<div class=\"recommendation-box\" style=\"border-left: 4px solid #e07c3c;\">\n<h3>Minor criteria (systemic mastocytosis / SM)</h3>\n");
    out.push_str("<p>These are often considered when evaluating mast cell disorders. This report explicitly checks the genetic criterion; the others require lab or pathology.</p>\n<ul>\n");
    out.push_str("<li><strong>High levels of tryptase in the blood</strong> — Requires lab (e.g. serum tryptase). Discuss with your clinician.</li>\n");
    out.push_str("<li><strong>Unusual expression of CD25 on mast cells</strong> — Requires pathology (e.g. bone marrow). Discuss with your clinician.</li>\n");
    if report.kit_d816v_detected {
        out.push_str("<li><strong>Presence of the KIT D816V mutation</strong> — <strong>Problem:</strong> Checked by this report: detected. Associated with systemic mastocytosis; discuss with your clinician.</li>\n");
    } else {
        out.push_str("<li><strong>Presence of the KIT D816V mutation</strong> — <strong>Reference:</strong> Not detected in your variant set. (Other minor criteria still require lab/pathology.)</li>\n");
    }
    out.push_str("<li><strong>Unusually large number of abnormal mast cells</strong> — Requires pathology (e.g. bone marrow). Discuss with your clinician.</li>\n");
    out.push_str("</ul>\n<p>For research and education only; not for clinical diagnosis.</p>\n</div>\n");

    // Reference: condition descriptions (SM, CM, MCAS, HαT) — clearly labeled as reference, not "your result"
    out.push_str("<h3 style=\"margin-top: 1.5em;\">Reference: what each condition is (not your result)</h3>\n<p style=\"margin-bottom: 1em;\">The cards below describe each disorder for context. Your result for this run is stated above (\"This run: your data was checked\" and Minor criteria).</p>\n");
    for r in &mcas_refs {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.name));
        out.push_str("</h3>\n<p>");
        out.push_str(&escape(&r.description));
        out.push_str("</p>\n<p><strong>Genes:</strong> ");
        out.push_str(&escape(&r.genes.join(", ")));
        out.push_str("</p>\n<p><strong>Biomarkers / key terms:</strong> ");
        out.push_str(&escape(&r.biomarkers.join("; ")));
        out.push_str("</p>\n<p><strong>Criteria / notes:</strong> ");
        out.push_str(&escape(&r.criteria_notes.join(" ")));
        out.push_str("</p>\n<p><strong>References:</strong> ");
        out.push_str(&escape(&r.references.join("; ")));
        out.push_str("</p>\n");
        if r.name.contains("alpha-tryptasemia") || r.name.contains("HαT") {
            if let Some(cn) = copy_number {
                if let Some(n) = copy_number::tpsab1_copy_number(cn) {
                    if copy_number::tpsab1_gain_detected(cn) {
                        out.push_str("<div class=\"recommendation-box\" style=\"margin-top: 1em; border-left-color: #0c5460;\"><p><strong>Do I have this? / Zygosity:</strong> Your <strong>copy number assay</strong> shows TPSAB1 copy number <strong>");
                        write!(out, "{}", n).unwrap();
                        out.push_str("</strong> (copy number gain). This is <strong>consistent with Hereditary alpha-tryptasemia (HαT)</strong>. HαT is defined by TPSAB1 duplication. Discuss interpretation, baseline tryptase, and management with your clinician.</p></div>\n");
                    } else {
                        out.push_str("<div class=\"recommendation-box\" style=\"margin-top: 1em; border-left-color: #0c5460;\"><p><strong>Do I have this? / Zygosity:</strong> Your <strong>copy number assay</strong> shows TPSAB1 copy number <strong>");
                        write!(out, "{}", n).unwrap();
                        out.push_str("</strong> (no copy number gain). HαT is defined by TPSAB1 duplication; this result does not support HαT. If you have elevated baseline tryptase, other causes can be considered with your clinician.</p></div>\n");
                    }
                } else {
                    out.push_str("<div class=\"recommendation-box\" style=\"margin-top: 1em; border-left-color: #0c5460;\"><p><strong>Do I have this? / Zygosity:</strong> You provided copy number assay data, but <strong>TPSAB1</strong> was not included. HαT is caused by TPSAB1 copy number gain. To interpret HαT from this report, include TPSAB1 in your copy number JSON (gene: \"TPSAB1\", copy_number: &lt;number&gt;).</p></div>\n");
                }
            } else {
                out.push_str("<div class=\"recommendation-box\" style=\"margin-top: 1em; border-left-color: #0c5460;\"><p><strong>Do I have this? / Zygosity:</strong> This report <strong>does not</strong> tell you whether you have HαT. HαT is caused by a <strong>copy number gain</strong> (duplication) of TPSAB1, which is detected by dedicated genetic testing (e.g. copy number assay), not by SNV/variant data. Your data here are SNVs/indels; we do not call TPSAB1 copy number. We cannot give zygosity or a yes/no for HαT from this report. If you have elevated baseline tryptase, discuss HαT testing with your clinician.</p></div>\n");
            }
        }
        out.push_str("</div>\n");
    }

    // MCAS stabilizers: predicted benefits and best combo (includes cromolyn sodium)
    let stabilizers = inflammation::mcas_stabilizers_ref();
    let combo = inflammation::mcas_recommended_combo_with_cromolyn();
    out.push_str("<h2>Predicted benefits from MCAS stabilizers</h2>\n");
    out.push_str("<p>Reference options for mast cell stabilizers and antihistamines. Dosing and prescribing are clinician-directed.</p>\n");
    for s in &stabilizers {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&s.name));
        out.push_str("</h3>\n<p><strong>Benefit domains:</strong> ");
        out.push_str(&escape(&s.benefit_domains.join("; ")));
        out.push_str("</p>\n<p>");
        out.push_str(&escape(&s.benefit_rationale));
        out.push_str("</p>\n<p><strong>Dosing note:</strong> ");
        out.push_str(&escape(&s.dosing_note));
        out.push_str("</p>\n");
        if let Some(ref cr) = s.combo_rationale {
            out.push_str("<div class=\"recommendation-box\"><strong>In combo:</strong> ");
            out.push_str(&escape(cr));
            out.push_str("</div>\n");
        }
        out.push_str("</div>\n");
    }
    out.push_str("<div class=\"recommendation-box\">\n<h3>Suggested best combo (includes cromolyn sodium)</h3>\n<p>Recommended combination for broad predicted benefit:</p>\n<ol>");
    for name in &combo {
        out.push_str("\n<li>");
        out.push_str(&escape(name));
        out.push_str("</li>");
    }
    out.push_str("\n</ol>\n<p>Cromolyn sodium is the anchor stabilizer; H1 + H2 provide dual blockade; ketotifen adds systemic and CNS coverage. All prescribing must be by a qualified clinician.</p>\n</div>\n");

    // Cromolyn & Ketotifen for this run: node analysis + genetic analysis
    let inflammation_count: usize = report.inflammation.iter().map(|r| r.findings.len()).sum();
    let genes_with_findings: Vec<&str> = report.inflammation.iter().filter(|r| !r.findings.is_empty()).flat_map(|r| r.genes_checked.iter().map(String::as_str)).collect();
    let mast_clearance_priority = cascade_report.as_ref().map(|c| c.clearance_needs.iter().any(|n| n.process_support.contains("Mast cell") || n.process_support.contains("mast cell"))).unwrap_or(false);
    out.push_str("<div class=\"recommendation-box\" style=\"border-left: 4px solid #667eea;\">\n<h3>Cromolyn sodium &amp; Ketotifen: consideration for your run</h3>\n");
    out.push_str("<p><strong>Genetic analysis (this run):</strong> ");
    if inflammation_count > 0 {
        write!(out, "You have {} variant finding(s) in MCAS/mastocytosis genes ({}). ", inflammation_count, escape(&genes_with_findings.join(", "))).unwrap();
    } else {
        out.push_str("No variants reported in KIT or TPSAB1 in this variant set. ");
    }
    out.push_str("General genetic analysis (immune, sulfur, inflammation, disorders) is in the other tabs.</p>\n");
    out.push_str("<p><strong>Node / cascade analysis:</strong> ");
    if let Some(cr) = cascade_report {
        write!(out, "Composite cascade score (0–100): {}. ", cr.scores.composite_cgrp_runaway_cascade).unwrap();
        if mast_clearance_priority || cr.scores.calcium_mast_cell_sensitivity > 0 {
            out.push_str("Mast cell / histamine is a primary driver or clearance need in the pathway ranking; <strong>mast cell stabilisation</strong> is relevant. ");
        }
        out.push_str("Primary drivers: ");
        out.push_str(&escape(&cr.ranking.primary_drivers.join("; ")));
        out.push_str(".</p>\n");
    } else {
        out.push_str("Cascade scores not computed for this view. Run the full report for node analysis.</p>\n");
    }
    out.push_str("<p><strong>Cromolyn sodium:</strong> Oral often 100–200 mg four times daily, titrated; ampules for oral use. Anchor stabilizer for GI and systemic MCAS. Dosing is patient-specific; a clinician must prescribe.</p>\n");
    out.push_str("<p><strong>Ketotifen:</strong> Typically 0.5–2 mg once or twice daily; start low. Adds systemic and CNS coverage; complements cromolyn. Prescription; dosing is clinician-directed.</p>\n");
    out.push_str("<p>Use the node analysis (Cascade tab) and your genetic findings together with clinical assessment to decide whether and how much Cromolyn or Ketotifen to consider. This report is for research and education only; not prescribing advice.</p>\n</div>\n");

    out.push_str("<div class=\"condition-card\">\n<h3>");
    out.push_str(&escape(&slk_ref.name));
    out.push_str("</h3>\n<p>");
    out.push_str(&escape(&slk_ref.description));
    out.push_str("</p>\n<p><strong>Key terms:</strong> ");
    out.push_str(&escape(&slk_ref.key_terms.join(", ")));
    out.push_str("</p>\n<p><strong>Associations:</strong> ");
    out.push_str(&escape(&slk_ref.associations.join(", ")));
    out.push_str("</p>\n<p><strong>References:</strong> ");
    out.push_str(&escape(&slk_ref.references.join("; ")));
    out.push_str("</p>\n</div>\n");

    out.push_str("</div>\n\n");

    // Inflammation tab (all inflammation reports)
    let infl_first = report.inflammation.iter().flat_map(|r| r.findings.iter()).next()
        .map(|f| format!("{}:{}", f.variant.chromosome, f.variant.position))
        .unwrap_or_else(|| "none".to_string());
    tap("02_inflammation", &format!("count={} first={}", inflammation_count, infl_first));
    out.push_str("<!-- Inflammation findings -->\n<div id=\"inflammation\" class=\"content-section\">\n<h2>Inflammation (MCAS / mastocytosis) reports</h2>\n");
    out.push_str("<p><strong>Confidence / evidence:</strong> Variant confidence per finding when available; section: gene–condition association from literature (PMC8540348). Not diagnostic.</p>\n");
    section_reports_inflammation(&mut out, &report.inflammation);
    out.push_str("</div>\n\n");

    // Immune — worst first: reports with findings, then no findings
    let immune_count: usize = report.immune.iter().map(|r| r.findings.len()).sum();
    let immune_first = report.immune.iter().flat_map(|r| r.findings.iter()).next()
        .map(|f| format!("{}:{}", f.variant.chromosome, f.variant.position))
        .unwrap_or_else(|| "none".to_string());
    tap("03_immune", &format!("count={} first={}", immune_count, immune_first));
    out.push_str("<!-- Immune -->\n<div id=\"immune\" class=\"content-section\">\n<h2>Immune disease reports</h2>\n");
    out.push_str("<p><strong>Confidence / evidence:</strong> Variant confidence per finding when available; section: gene–disease association from literature. Susceptibility/association only; not diagnostic.</p>\n");
    out.push_str("<p>Conditions with findings are listed first; then conditions with no matching variants.</p>\n");
    for r in report.immune.iter().filter(|r| !r.findings.is_empty()).chain(report.immune.iter().filter(|r| r.findings.is_empty())) {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.disease_name));
        out.push_str("</h3>\n<p>Genes checked: ");
        out.push_str(&escape(&r.genes_checked.join(", ")));
        out.push_str("</p>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\">\n");
            out.push_str(&variant_finding_display_html(&f.variant, &f.note));
            out.push_str("</div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n\n");

    // Exposure — worst first
    let exposure_count: usize = report.exposure.iter().map(|r| r.findings.len()).sum();
    let exposure_first = report.exposure.iter().flat_map(|r| r.findings.iter()).next()
        .map(|f| format!("{}:{}", f.variant.chromosome, f.variant.position))
        .unwrap_or_else(|| "none".to_string());
    tap("04_exposure", &format!("count={} first={}", exposure_count, exposure_first));
    out.push_str("<!-- Exposure -->\n<div id=\"exposure\" class=\"content-section\">\n<h2>Chemical exposure reports</h2>\n");
    out.push_str("<p><strong>Confidence / evidence:</strong> Variant confidence per finding when available; section: chemical–gene susceptibility from published studies. Not diagnostic.</p>\n");
    out.push_str("<p>Reports with findings are listed first; then reports with no matching variants.</p>\n");
    for r in report.exposure.iter().filter(|r| !r.findings.is_empty()).chain(report.exposure.iter().filter(|r| r.findings.is_empty())) {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.chemical_name));
        out.push_str("</h3>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\">\n");
            out.push_str(&variant_finding_display_html(&f.variant, &f.note));
            out.push_str("</div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n\n");

    // Sulfur — worst first
    let sulfur_count: usize = report.sulfur.iter().map(|r| r.findings.len()).sum();
    let sulfur_first = report.sulfur.iter().flat_map(|r| r.findings.iter()).next()
        .map(|f| format!("{}:{}", f.variant.chromosome, f.variant.position))
        .unwrap_or_else(|| "none".to_string());
    tap("05_sulfur", &format!("count={} first={}", sulfur_count, sulfur_first));
    out.push_str("<!-- Sulfur -->\n<div id=\"sulfur\" class=\"content-section\">\n<h2>Sulfur metabolism reports</h2>\n");
    out.push_str("<p><strong>Confidence / evidence:</strong> Variant confidence per finding when available; section: gene–condition from OMIM/GeneReviews. Not diagnostic.</p>\n");
    out.push_str("<p>Conditions with findings are listed first; then conditions with no matching variants.</p>\n");
    for r in report.sulfur.iter().filter(|r| !r.findings.is_empty()).chain(report.sulfur.iter().filter(|r| r.findings.is_empty())) {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.condition_name));
        out.push_str("</h3>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\">\n");
            out.push_str(&variant_finding_display_html(&f.variant, &f.note));
            out.push_str("</div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n\n");

    // Rare — worst first
    let rare_count: usize = report.rare.iter().map(|r| r.findings.len()).sum();
    let rare_first = report.rare.iter().flat_map(|r| r.findings.iter()).next()
        .map(|f| format!("{}:{}", f.variant.chromosome, f.variant.position))
        .unwrap_or_else(|| "none".to_string());
    tap("06_rare", &format!("count={} first={}", rare_count, rare_first));
    out.push_str("<!-- Rare -->\n<div id=\"rare\" class=\"content-section\">\n<h2>Rare disease reports</h2>\n");
    out.push_str("<p><strong>Confidence / evidence:</strong> Variant confidence per finding when available; section: gene–disease from GeneReviews/OMIM. Not diagnostic.</p>\n");
    out.push_str("<p>Diseases with findings are listed first; then diseases with no matching variants.</p>\n");
    for r in report.rare.iter().filter(|r| !r.findings.is_empty()).chain(report.rare.iter().filter(|r| r.findings.is_empty())) {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.disease_name));
        out.push_str("</h3>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\">\n");
            out.push_str(&variant_finding_display_html(&f.variant, &f.note));
            out.push_str("</div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n\n");

    // Cancer screening — worst first
    let cancer_count: usize = report.cancer.iter().map(|r| r.findings.len()).sum();
    let cancer_first = report.cancer.iter().flat_map(|r| r.findings.iter()).next()
        .map(|f| format!("{}:{}", f.variant.chromosome, f.variant.position))
        .unwrap_or_else(|| "none".to_string());
    tap("07_cancer", &format!("count={} first={}", cancer_count, cancer_first));
    out.push_str("<!-- Cancer screening -->\n<div id=\"cancer\" class=\"content-section\">\n<h2>Cancer screening (genetic)</h2>\n");
    out.push_str("<p><strong>Confidence / evidence:</strong> Variant confidence per finding when available; section: gene–syndrome from GeneReviews/NCCN. Genetic-only; pathogenicity and clinical action require accredited testing and genetic counseling.</p>\n");
    out.push_str("<p>Hereditary cancer syndrome genes only. Not a substitute for clinical genetic testing or counseling. Syndromes with findings are listed first.</p>\n");
    for r in report.cancer.iter().filter(|r| !r.findings.is_empty()).chain(report.cancer.iter().filter(|r| r.findings.is_empty())) {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.syndrome_name));
        out.push_str("</h3>\n<p>Genes checked: ");
        out.push_str(&escape(&r.genes_checked.join(", ")));
        out.push_str("</p>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\">\n");
            out.push_str(&variant_finding_display_html(&f.variant, &f.note));
            out.push_str("</div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        if !r.screening_notes.is_empty() {
            out.push_str("<p><strong>Screening notes:</strong> ");
            out.push_str(&escape(&r.screening_notes.join(" ")));
            out.push_str("</p>\n");
        }
        out.push_str("<p class=\"alert alert-info\" style=\"font-size:0.9em;\">"); out.push_str(&escape(&r.disclaimer)); out.push_str("</p>\n");
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n\n");

    // Disorders — worst first
    let disorders_count: usize = report.disorders.iter().map(|r| r.findings.len()).sum();
    let disorders_first = report.disorders.iter().flat_map(|r| r.findings.iter()).next()
        .map(|f| format!("{}:{}", f.variant.chromosome, f.variant.position))
        .unwrap_or_else(|| "none".to_string());
    tap("08_disorders", &format!("count={} first={}", disorders_count, disorders_first));
    out.push_str("<!-- Disorders -->\n<div id=\"disorders\" class=\"content-section\">\n<h2>Disorders (susceptibility)</h2>\n<p><strong>Confidence / evidence:</strong> Variant confidence per finding when available; section: susceptibility associations only (GWAS, literature); not diagnostic.</p>\n<p>Psychiatric, autoimmune, neurological, and metabolic disorders with well-established susceptibility genes. Variants indicate association only. Disorders with findings are listed first.</p>\n");
    for r in report.disorders.iter().filter(|r| !r.findings.is_empty()).chain(report.disorders.iter().filter(|r| r.findings.is_empty())) {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.disorder_name));
        out.push_str("</h3>\n<p><strong>Category:</strong> "); out.push_str(&escape(&r.category));
        out.push_str("</p>\n<p>Genes checked: ");
        out.push_str(&escape(&r.genes_checked.join(", ")));
        out.push_str("</p>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\">\n");
            out.push_str(&variant_finding_display_html(&f.variant, &f.note));
            out.push_str("</div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("<p class=\"alert alert-info\" style=\"font-size:0.9em;\">"); out.push_str(&escape(&r.disclaimer)); out.push_str("</p>\n");
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n\n");

    // Supplements for weak areas (≤15)
    let supps = supplements::supplements_for_weak_areas();
    out.push_str("<!-- Supplements -->\n<div id=\"supplements\" class=\"content-section\">\n<h2>Supplement support for weak areas (≤15)</h2>\n<p><strong>Confidence / evidence:</strong> Pathway-based suggestions from literature; not clinical recommendations. Dosing and suitability must be determined by a clinician.</p>\n<p>These commonly support methylation, MCAS/inflammation, homocysteine, and immune pathways. For research and educational use only.</p>\n");
    if let Some(cr) = cascade_report {
        let mast = cr.scores.calcium_mast_cell_sensitivity > 0 || cr.clearance_needs.iter().any(|n| n.process_support.contains("Mast cell"));
        let sulfur = cr.scores.sulfur_burden_likelihood > 0 || cr.suspected_buildups.iter().any(|b| b.category.contains("Sulfite"));
        let antiox = cr.clearance_needs.iter().any(|n| n.process_support.contains("Antioxidant"));
        let mut best: Vec<&str> = Vec::new();
        if mast { best.push("Quercetin, Vitamin C, PEA, Apigenin, Omega-3 (mast cell / histamine)"); }
        if sulfur { best.push("B6 P-5-P, Methylfolate, B12 (sulfur / methylation)"); }
        if antiox || mast { best.push("Vitamin C, Quercetin, Magnesium (antioxidant)"); }
        if !best.is_empty() {
            out.push_str("<div class=\"recommendation-box\"><p><strong>Best for your pathways this run:</strong> ");
            out.push_str(&escape(&best.join(". ")));
            out.push_str(" Full breakdown (prescription meds + supplements by pathway) is in the <strong>Cascade</strong> tab under \"Best for your pathways: meds &amp; supplements to clear\".</p></div>\n");
        }
        // Holistic view: what will help you most, considered together
        out.push_str("<div class=\"recommendation-box\" style=\"border-left-color: #667eea;\"><h3>Holistic view: what will help you most</h3>\n<p>Supplements work best when chosen as a <strong>coherent set</strong> that matches your pathways. Consider these groups together:</p>\n<ul style=\"margin: 1em 0; line-height: 1.7;\">\n");
        if mast {
            out.push_str("<li><strong>Mast cell / inflammation stack:</strong> Quercetin + Vitamin C (stabilizer + antioxidant); PEA and Apigenin (mast cell stabilizers); Omega-3 (anti-inflammatory). Add as a set if your cascade points to mast cell or histamine clearance.</li>\n");
        }
        if sulfur {
            out.push_str("<li><strong>Methylation / sulfur stack:</strong> Methylfolate (5-MTHF) + B12 + B6 (P-5-P) support the methylation and transsulfuration pathways together. Start with folate and B12; add B6 if sulfur or homocysteine is a focus. Magnesium acts as a cofactor for many of these steps.</li>\n");
        }
        if !mast && !sulfur {
            out.push_str("<li><strong>Methylation / sulfur stack:</strong> Methylfolate + B12 + B6 (P-5-P) work together for methylation and homocysteine; add Magnesium as a cofactor. A general foundation when genetics suggest these pathways.</li>\n");
        }
        out.push_str("<li><strong>Foundation (most people):</strong> Vitamin D and Omega-3 are often low and support immune and inflammatory balance broadly. Consider testing Vitamin D and dosing to level.</li>\n");
        out.push_str("</ul>\n<p><strong>For you:</strong> ");
        if mast && sulfur {
            out.push_str("Your run suggests both mast cell and sulfur/methylation are relevant. A holistic approach would combine the mast cell stack (Quercetin, Vitamin C, Omega-3) with the methylation stack (Methylfolate, B12, B6) and Magnesium; add Vitamin D by level. Introduce one group at a time and discuss with a clinician.</p>\n");
        } else if mast {
            out.push_str("Your run points to mast cell / histamine clearance. Prioritise Quercetin, Vitamin C, and Omega-3 together; add Methylfolate/B12/Magnesium as general support. Vitamin D by level.</p>\n");
        } else if sulfur {
            out.push_str("Your run points to sulfur/methylation. Prioritise Methylfolate, B12, and B6 (P-5-P) as a set; add Magnesium and consider Quercetin + Vitamin C for antioxidant support. Vitamin D by level.</p>\n");
        } else {
            out.push_str("Use the stacks above as a guide: methylation stack + Vitamin D and Omega-3 give a broad foundation. Add the mast cell stack (Quercetin, Vitamin C) if your symptoms or phenotype suggest it. Always clinician-directed.</p>\n");
        }
        out.push_str("</div>\n");
    } else {
        out.push_str("<div class=\"recommendation-box\" style=\"border-left-color: #667eea;\"><h3>Holistic view: what will help you most</h3>\n<p>Supplements work best as a <strong>coherent set</strong>. Consider: (1) <strong>Methylation stack</strong> — Methylfolate, B12, B6 (P-5-P), and Magnesium together; (2) <strong>Mast cell / antioxidant</strong> — Quercetin + Vitamin C + Omega-3; (3) <strong>Foundation</strong> — Vitamin D and Omega-3. Run with cascade data for a personalised holistic summary above.</p>\n</div>\n");
    }
    for s in &supps {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&s.name));
        out.push_str("</h3>\n<p><strong>Weak areas:</strong> ");
        out.push_str(&escape(&s.weak_areas.join(", ")));
        out.push_str("</p>\n<p><strong>Gene effects counteracted (where the failure is):</strong></p>\n<ul style=\"margin: 0.5em 0 1em 1.2em;\">\n");
        for eff in &s.gene_effects_counteracted {
            out.push_str("<li>"); out.push_str(&escape(eff)); out.push_str("</li>\n");
        }
        out.push_str("</ul>\n<p>");
        out.push_str(&escape(&s.rationale));
        out.push_str("</p>\n<p><strong>Typical note:</strong> ");
        out.push_str(&escape(&s.typical_note));
        out.push_str("</p>\n</div>\n");
    }
    out.push_str("</div>\n\n");

    if let Some(cr) = cascade_report {
        out.push_str("<!-- Integrated Cascade -->\n<div id=\"cascade\" class=\"content-section\">\n<h2>Integrated cascade summary</h2>\n<p><strong>Confidence / evidence:</strong> Genotype-driven scores only; phenotype can refine. For research only; not clinical.</p>\n<p>Genotype-driven scores and pathway ranking. Phenotype can refine.</p>\n");
        // Context: environment (e.g. SLC winter) and stress load — same nodes, different demand
        out.push_str("<div class=\"recommendation-box\" style=\"border-left-color: #6c757d;\">\n<h3 style=\"margin-top: 0;\">Context: environment and stress load</h3>\n");
        out.push_str("<p><strong>Your genes don’t change</strong> with stress or where you live, but <strong>the demand on your pathways does</strong>. The same genetic “nodes” (e.g. mast cell, methylation, sulfur) can be under more strain when:</p>\n<ul style=\"margin: 0.5em 0 1em 1.2em;\">\n");
        out.push_str("<li><strong>Environmental load is higher.</strong> Living in a place with <strong>winter inversions</strong> (e.g. Salt Lake City) adds airborne particulates (PM2.5) and pollutants that increase oxidative and inflammatory load. Your clearance pathways work harder during inversion season; the same cascade recommendations (antioxidants, mast cell support, avoiding extra triggers) may be more relevant then.</li>\n");
        out.push_str("<li><strong>Stress load is higher.</strong> Chronic or heavy stress (work, life events, illness, sleep loss) raises sympathetic tone, cortisol, and inflammatory signals. That can make the same genotype <em>behave</em> as if it’s under more strain — nodes don’t change, but demand goes up. So under <strong>high stress</strong>, the same supplements and lifestyle measures (mast cell stabilisers, methylation support, sleep, reducing triggers) often matter more or are needed at a lower threshold.</li>\n");
        out.push_str("</ul>\n<p><strong>Spectrum:</strong> From <em>low/normal stress load</em> to <em>heavy stress load</em> — treat your cascade and clearance suggestions as a baseline. When you’re under heavy stress or in a high-exposure period (e.g. SLC winter), lean into the same recommendations a bit more rather than ignoring them. Your doctor can help you prioritise.</p>\n</div>\n");
        out.push_str("<h3>Scores (0–100)</h3>\n<ul>");
        let s = &cr.scores;
        out.push_str(&format!("<li>Calcium/mast cell sensitivity: {} ({:?})</li>", s.calcium_mast_cell_sensitivity, s.band("calcium_mast_cell_sensitivity")));
        out.push_str(&format!("<li>Trigeminal/calcium excitability: {} ({:?})</li>", s.trigeminal_calcium_excitability, s.band("trigeminal_calcium_excitability")));
        out.push_str(&format!("<li>Sulfur burden likelihood: {} ({:?})</li>", s.sulfur_burden_likelihood, s.band("sulfur_burden_likelihood")));
        out.push_str(&format!("<li>Composite CGRP cascade: {} ({:?})</li>", s.composite_cgrp_runaway_cascade, s.band("composite_cgrp_runaway_cascade")));
        out.push_str("</ul>\n<h3>Primary drivers</h3>\n<ol>");
        for d in &cr.ranking.primary_drivers {
            out.push_str("<li>"); out.push_str(&escape(d)); out.push_str("</li>");
        }
        out.push_str("</ol>\n");
        // KIT/TPSAB1 and phenotype: what "phenotype" means and what this run shows
        let kit_tpsab1_findings: usize = report.inflammation.iter().map(|r| r.findings.len()).sum();
        let kit_tpsab1_genes: Vec<&str> = report.inflammation.iter().filter(|r| !r.findings.is_empty()).flat_map(|r| r.genes_checked.iter().map(String::as_str)).collect();
        out.push_str("<div class=\"recommendation-box\"><h4>KIT/TPSAB1 and your phenotype</h4>\n<p><strong>For this run (genotype):</strong> ");
        if kit_tpsab1_findings > 0 {
            write!(out, "You have {} variant finding(s) in {} — so the &quot;if KIT/TPSAB1&quot; part of the primary driver above <strong>applies</strong> to you.", kit_tpsab1_findings, escape(&kit_tpsab1_genes.join(", "))).unwrap();
        } else {
            out.push_str("No variants in KIT or TPSAB1 in this variant set — so the cascade is not driven by those genes in this run.");
        }
        out.push_str("</p>\n<p><strong>Phenotype</strong> here means your <em>clinical picture</em>: e.g. flushing, gut pain, headache, skin reactivity, reactions to foods or triggers, brain fog — symptoms that can suggest mast cell / histamine / prostaglandin activation. This report does <strong>not</strong> collect or use your phenotype; it is genotype-only. If your symptoms align with that primary driver, consider it relevant when reading the cascade and clearance options.</p>\n</div>\n");
        out.push_str("<h3>Suspected buildup / clearance</h3>\n");
        for b in &cr.suspected_buildups {
            out.push_str("<div class=\"condition-card\"><h4>"); out.push_str(&escape(&b.category)); out.push_str("</h4>\n<p>"); out.push_str(&escape(&b.why_may_accumulate)); out.push_str("</p>\n<p><strong>Clearance:</strong> "); out.push_str(&escape(&b.clearance_category)); out.push_str("</p></div>\n");
        }
        // Best for your pathways: meds and supplements to support clearance (breakdown by pathway)
        out.push_str("<h3>Best for your pathways: meds &amp; supplements to clear</h3>\n<p>Based on your cascade scores and clearance needs above, these options best support each pathway. Prescription choices are clinician-directed.</p>\n");
        let mast_relevant = cr.scores.calcium_mast_cell_sensitivity > 0 || cr.clearance_needs.iter().any(|n| n.process_support.contains("Mast cell") || n.process_support.contains("mast cell"));
        let histamine_relevant = cr.suspected_buildups.iter().any(|b| b.category.contains("Histamine")) || mast_relevant;
        let sulfur_relevant = cr.scores.sulfur_burden_likelihood > 0 || cr.suspected_buildups.iter().any(|b| b.category.contains("Sulfite") || b.category.contains("Sulfur"));
        let antioxidant_relevant = cr.clearance_needs.iter().any(|n| n.process_support.contains("Antioxidant") || n.process_support.contains("ROS"));
        if mast_relevant || histamine_relevant {
            out.push_str("<div class=\"recommendation-box\"><h4>Mast cell / histamine clearance</h4>\n<p><strong>Why:</strong> "); out.push_str(&escape(if mast_relevant { "Your cascade flags mast cell sensitivity or stabilisation as a clearance need." } else { "Histamine buildup is a suspected clearance need." })); out.push_str("</p>\n<p><strong>Best prescription options (in order):</strong> Cromolyn sodium (anchor stabilizer), H1 blocker, H2 blocker, Ketotifen (systemic/CNS). All clinician-directed.</p>\n<p><strong>Best supplements:</strong> Quercetin + Vitamin C (mast cell stabilizer/antioxidant); PEA (palmitoylethanolamide) and Apigenin (stabilizers); Omega-3 (anti-inflammatory). Not a substitute for prescription stabilizers.</p>\n</div>\n");
        }
        if sulfur_relevant {
            out.push_str("<div class=\"recommendation-box\"><h4>Sulfur / sulfite handling</h4>\n<p><strong>Why:</strong> Your cascade suggests sulfur or sulfite burden (CBS/SUOX/CTH).</p>\n<p><strong>Best supplements:</strong> B6 (P-5-P) for transsulfuration; Methylfolate (5-MTHF) and B12 for methylation and homocysteine clearance. Dosing is clinician-directed.</p>\n</div>\n");
        }
        if antioxidant_relevant || mast_relevant {
            out.push_str("<div class=\"recommendation-box\"><h4>Antioxidant / ROS cleanup</h4>\n<p><strong>Why:</strong> Cascade indicates mitochondrial or redox stress; antioxidants can support clearance.</p>\n<p><strong>Best supplements:</strong> Vitamin C, Quercetin, Omega-3; Magnesium as cofactor. Start low; clinician-directed.</p>\n</div>\n");
        }
        if !mast_relevant && !histamine_relevant && !sulfur_relevant && !antioxidant_relevant {
            out.push_str("<p class=\"alert alert-info\">No pathway-specific clearance needs above threshold for this run. General supplement support (methylation, immune) is in the Supplements tab.</p>\n");
        }
        out.push_str("</div>\n\n");
    }

    if let Some(mi) = mcas_integrated {
        out.push_str("<!-- MCAS / Mast Cell Instability Integrated Analysis -->\n<div id=\"mcas-integrated\" class=\"content-section\">\n<h2>MCAS / Mast Cell Instability Integrated Analysis</h2>\n<p><strong>Level 3.</strong> Genetics-based evaluation of mast-cell instability risk, histamine clearance, degranulation tendency, and intervention leverage. For research and educational use only; not for clinical diagnosis.</p>\n");
        let risk_s = match mi.risk_level {
            crate::mcas_integrated::McasRiskLevel::Low => "Low",
            crate::mcas_integrated::McasRiskLevel::Moderate => "Moderate",
            crate::mcas_integrated::McasRiskLevel::High => "High",
            crate::mcas_integrated::McasRiskLevel::VeryHigh => "Very high",
        };
        let conf_s = match mi.risk_confidence {
            crate::mcas_integrated::McasConfidence::Low => "low",
            crate::mcas_integrated::McasConfidence::Medium => "medium",
            crate::mcas_integrated::McasConfidence::High => "high",
        };
        write!(out, "<div class=\"recommendation-box\" style=\"border-left: 4px solid #667eea;\"><h3 style=\"margin-top: 0;\">MCAS / mast cell instability risk</h3>\n<p><strong>Risk level:</strong> {} &nbsp; <strong>Confidence:</strong> {}</p>\n<p>Based on variant coverage and pathway coherence across signaling, histamine, cytokine, oxidative/sulfur, and neuroimmune pathways.</p>\n</div>\n", escape(risk_s), escape(conf_s)).unwrap();
        out.push_str("<h3>Pathway breakdown</h3>\n");
        for pr in &mi.pathway_reports {
            out.push_str("<div class=\"condition-card\">\n<h4>"); out.push_str(&escape(&pr.pathway_name)); out.push_str("</h4>\n");
            write!(out, "<p><strong>Baseline function:</strong> {}% &nbsp; <strong>Genes checked:</strong> {}</p>\n", pr.baseline_function_pct, escape(&pr.genes_checked.join(", "))).unwrap();
            out.push_str("<p><strong>Mechanism:</strong> "); out.push_str(&escape(&pr.mechanism_note)); out.push_str("</p>\n");
            out.push_str("<p><strong>Expected consequence:</strong> "); out.push_str(&escape(&pr.expected_consequence)); out.push_str("</p>\n");
            out.push_str("<p><strong>Likely symptom expression:</strong> "); out.push_str(&escape(&pr.likely_symptom_expression)); out.push_str("</p>\n");
            out.push_str("<p><strong>Promotes:</strong> "); out.push_str(&escape(&pr.promotes.join("; "))); out.push_str("</p>\n");
            if !pr.findings.is_empty() {
                out.push_str("<p><strong>Impacted genes / variants:</strong></p>\n");
                for f in &pr.findings {
                    out.push_str("<div class=\"gene-variant\">\n");
                    out.push_str(&variant_finding_display_html(&f.variant, &f.note));
                    out.push_str("</div>\n");
                }
            } else {
                out.push_str("<p>No variants in this pathway in this set.</p>\n");
            }
            out.push_str("</div>\n");
        }
        out.push_str("<h3>Cascade analysis</h3>\n<ul style=\"margin: 1em 0; line-height: 1.8;\">\n");
        for nar in &mi.cascade_narratives {
            out.push_str("<li>"); out.push_str(&escape(nar)); out.push_str("</li>\n");
        }
        out.push_str("</ul>\n");
        out.push_str("<h3>Interventions (ranked by estimated effectiveness)</h3>\n<p>Impact level and reasoning from pathway fit. Discuss timing and precautions with your clinician.</p>\n<table style=\"border-collapse: collapse; width: 100%; margin: 1em 0;\"><thead><tr style=\"background: #667eea; color: white;\"><th style=\"padding: 8px; border: 1px solid #ddd;\">Intervention</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Effectiveness (1–10)</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Reasoning</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Precautions</th></tr></thead><tbody>\n");
        for int in &mi.interventions {
            out.push_str("<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&int.name));
            write!(out, "</td><td style=\"padding: 8px; border: 1px solid #ddd;\">{}</td><td style=\"padding: 8px; border: 1px solid #ddd;\">", int.effectiveness_1_to_10).unwrap();
            out.push_str(&escape(&int.reasoning));
            out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">");
            out.push_str(&escape(int.precautions.as_deref().unwrap_or("—")));
            out.push_str("</td></tr>\n");
        }
        out.push_str("</tbody></table>\n");
        let pattern_s = match mi.symptom_inference.dominant_pattern {
            crate::mcas_integrated::MediatorPattern::HistamineDominant => "Histamine dominant",
            crate::mcas_integrated::MediatorPattern::ProstaglandinDominant => "Prostaglandin dominant",
            crate::mcas_integrated::MediatorPattern::LeukotrieneDominant => "Leukotriene dominant",
            crate::mcas_integrated::MediatorPattern::MixedMediator => "Mixed mediator",
            crate::mcas_integrated::MediatorPattern::CalciumTriggerNeuroimmune => "Calcium-trigger / neuroimmune amplified",
            crate::mcas_integrated::MediatorPattern::Unclear => "Unclear",
        };
        out.push_str("<h3>Question-driven mediator inference</h3>\n<div class=\"recommendation-box\"><p><strong>Dominant pattern:</strong> "); out.push_str(&escape(pattern_s)); out.push_str("</p>\n<p>"); out.push_str(&escape(&mi.symptom_inference.reasoning)); out.push_str("</p>\n<p>"); out.push_str(&escape(&mi.symptom_inference.user_pattern_note)); out.push_str("</p>\n</div>\n");
        out.push_str("<div class=\"recommendation-box\" style=\"border-left-color: #0c5460;\"><h4>User-relevant pattern</h4>\n<p>"); out.push_str(&escape(&mi.user_context_note)); out.push_str("</p>\n</div>\n");
        out.push_str("</div>\n\n");
    }

    if let Some(ea) = exercise_ammonia {
        out.push_str("<!-- Exercise Ammonia / Nitrogen Waste Handling Integrated Analysis -->\n<div id=\"exercise-ammonia\" class=\"content-section\">\n<h2>Exercise Ammonia / Nitrogen Waste Handling Integrated Analysis</h2>\n<p><strong>Level 3.</strong> Evaluates why ammonia may appear during hard exercise: urea cycle, amino acid catabolism, mitochondrial stress, AMP deamination, redox/sulfur, and mast-cell/inflammatory cross-talk. For research and educational use only; not for clinical diagnosis.</p>\n");
        let risk_s = match ea.risk_level {
            crate::exercise_ammonia::ExerciseAmmoniaRiskLevel::Low => "Low",
            crate::exercise_ammonia::ExerciseAmmoniaRiskLevel::Moderate => "Moderate",
            crate::exercise_ammonia::ExerciseAmmoniaRiskLevel::High => "High",
            crate::exercise_ammonia::ExerciseAmmoniaRiskLevel::VeryHigh => "Very high",
        };
        let conf_s = match ea.risk_confidence {
            crate::exercise_ammonia::ExerciseAmmoniaConfidence::Low => "low",
            crate::exercise_ammonia::ExerciseAmmoniaConfidence::Medium => "medium",
            crate::exercise_ammonia::ExerciseAmmoniaConfidence::High => "high",
        };
        write!(out, "<div class=\"recommendation-box\" style=\"border-left: 4px solid #28a745;\"><h3 style=\"margin-top: 0;\">Exercise ammonia / nitrogen clearance risk</h3>\n<p><strong>Risk level:</strong> {} &nbsp; <strong>Confidence:</strong> {}</p>\n<p><strong>Root-cause pattern:</strong> ", risk_s, conf_s).unwrap();
        let pattern_s = match ea.root_cause_pattern {
            crate::exercise_ammonia::RootCausePattern::PrimaryUreaCycleWeakness => "Primary urea-cycle weakness",
            crate::exercise_ammonia::RootCausePattern::MitochondrialAtpStress => "Mitochondrial ATP stress",
            crate::exercise_ammonia::RootCausePattern::AminoAcidOveruse => "Amino-acid overuse / protein catabolism",
            crate::exercise_ammonia::RootCausePattern::PurineAmpDeamination => "Purine-cycle / AMP deamination",
            crate::exercise_ammonia::RootCausePattern::RedoxDetoxBottleneck => "Redox / detox bottleneck",
            crate::exercise_ammonia::RootCausePattern::InflammatoryMastCellAmplified => "Inflammatory / mast-cell amplified",
            crate::exercise_ammonia::RootCausePattern::Mixed => "Mixed pattern",
        };
        out.push_str(&escape(pattern_s)); out.push_str("</p>\n<p>"); out.push_str(&escape(&ea.root_cause_reasoning)); out.push_str("</p>\n</div>\n");
        out.push_str("<div class=\"recommendation-box\" style=\"border-left-color: #0c5460;\"><h4>CPS1 / urea cycle check</h4>\n<p>"); out.push_str(&escape(&ea.cps1_check_note)); out.push_str("</p>\n</div>\n");
        out.push_str("<h3>Pathway breakdown</h3>\n");
        for pr in &ea.pathway_reports {
            out.push_str("<div class=\"condition-card\">\n<h4>"); out.push_str(&escape(&pr.pathway_name)); out.push_str("</h4>\n");
            write!(out, "<p><strong>Baseline function:</strong> {}% &nbsp; <strong>Genes checked:</strong> {}</p>\n", pr.baseline_function_pct, escape(&pr.genes_checked.join(", "))).unwrap();
            out.push_str("<p><strong>Mechanism:</strong> "); out.push_str(&escape(&pr.mechanism_note)); out.push_str("</p>\n");
            out.push_str("<p><strong>Consequence during hard exercise:</strong> "); out.push_str(&escape(&pr.consequence_during_exercise)); out.push_str("</p>\n");
            out.push_str("<p><strong>Likely symptom expression:</strong> "); out.push_str(&escape(&pr.likely_symptom_expression)); out.push_str("</p>\n");
            out.push_str("<p><strong>Waste / byproduct accumulation:</strong> "); out.push_str(&escape(&pr.waste_accumulation)); out.push_str("</p>\n");
            out.push_str("<p><strong>Cleanup adequate?</strong> "); out.push_str(&escape(&pr.cleanup_adequate)); out.push_str("</p>\n");
            if !pr.findings.is_empty() {
                out.push_str("<p><strong>Impacted genes / variants:</strong></p>\n");
                for f in &pr.findings {
                    out.push_str("<div class=\"gene-variant\">\n");
                    out.push_str(&variant_finding_display_html(&f.variant, &f.note));
                    out.push_str("</div>\n");
                }
            } else {
                out.push_str("<p>No variants in this pathway in this set.</p>\n");
            }
            out.push_str("</div>\n");
        }
        out.push_str("<h3>Cascade analysis</h3>\n<ul style=\"margin: 1em 0; line-height: 1.8;\">\n");
        for nar in &ea.cascade_narratives {
            out.push_str("<li>"); out.push_str(&escape(nar)); out.push_str("</li>\n");
        }
        out.push_str("</ul>\n");
        out.push_str("<h3>Interventions (ranked by estimated effectiveness)</h3>\n<table style=\"border-collapse: collapse; width: 100%; margin: 1em 0;\"><thead><tr style=\"background: #28a745; color: white;\"><th style=\"padding: 8px; border: 1px solid #ddd;\">Intervention</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Effectiveness (1–10)</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Reasoning</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Precautions</th></tr></thead><tbody>\n");
        for int in &ea.interventions {
            out.push_str("<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&int.name));
            write!(out, "</td><td style=\"padding: 8px; border: 1px solid #ddd;\">{}</td><td style=\"padding: 8px; border: 1px solid #ddd;\">", int.effectiveness_1_to_10).unwrap();
            out.push_str(&escape(&int.reasoning));
            out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">");
            out.push_str(&escape(int.precautions.as_deref().unwrap_or("—")));
            out.push_str("</td></tr>\n");
        }
        out.push_str("</tbody></table>\n");
        out.push_str("<h3>User-facing interpretation</h3>\n<div class=\"recommendation-box\">\n<p><strong>Why ammonia may appear during hard exercise:</strong> "); out.push_str(&escape(&ea.user_interpretation.why_ammonia_during_exercise)); out.push_str("</p>\n<p><strong>Production overload vs disposal weakness:</strong> "); out.push_str(&escape(&ea.user_interpretation.production_vs_disposal)); out.push_str("</p>\n<p><strong>Mast-cell / inflammatory contribution:</strong> "); out.push_str(&escape(&ea.user_interpretation.mast_cell_inflammatory_contribution)); out.push_str("</p>\n<p><strong>Interventions most likely to reduce:</strong> "); out.push_str(&escape(&ea.user_interpretation.interventions_most_likely_to_reduce)); out.push_str("</p>\n</div>\n");
        out.push_str("<p><strong>Symptom-inference:</strong> "); out.push_str(&escape(&ea.symptom_inference_note)); out.push_str("</p>\n");
        out.push_str("<div class=\"recommendation-box\" style=\"border-left-color: #0c5460;\"><h4>User-relevant context</h4>\n<p>"); out.push_str(&escape(&ea.user_context_note)); out.push_str("</p>\n</div>\n");
        out.push_str("<h3>Summary</h3>\n<table style=\"border-collapse: collapse; width: 100%; margin: 1em 0;\"><thead><tr style=\"background: #dee2e6;\"><th style=\"padding: 8px; border: 1px solid #ddd;\">Item</th><th style=\"padding: 8px; border: 1px solid #ddd;\">Result</th></tr></thead><tbody>\n");
        write!(out, "<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">Risk level</td><td style=\"padding: 8px; border: 1px solid #ddd;\">{}</td></tr>\n", escape(risk_s)).unwrap();
        write!(out, "<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">Root-cause pattern</td><td style=\"padding: 8px; border: 1px solid #ddd;\">{}</td></tr>\n", escape(pattern_s)).unwrap();
        out.push_str("<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">Pathways analyzed</td><td style=\"padding: 8px; border: 1px solid #ddd;\">Urea cycle, amino acid catabolism, mitochondrial, purine/AMP, redox/sulfur, electrolyte/cofactor</td></tr>\n");
        out.push_str("</tbody></table>\n</div>\n\n");
    }

    if let Some(sa) = survival_analysis {
        out.push_str("<!-- Survival analysis -->\n<div id=\"survival\" class=\"content-section\">\n<h2>Severe-phenotype genes: possible reasons for mild or survivable outcome</h2>\n<p><strong>Confidence / evidence:</strong> Gene-level detection only; variant pathogenicity and clinical significance are not assessed. Educational.</p>\n<p>"); out.push_str(&escape(&sa.summary)); out.push_str("</p>\n");
        for gene in &sa.genes_with_severe_phenotype {
            if let Some(ref r) = sa.by_gene.get(gene) {
                out.push_str("<div class=\"condition-card\">\n<h3>"); out.push_str(&escape(&r.gene)); out.push_str(" — "); out.push_str(&escape(&r.condition)); out.push_str("</h3>\n<p>"); out.push_str(&escape(&r.typical_severity)); out.push_str("</p>\n<h4>Possible reasons for mild or survival</h4>\n<ul>");
                for reason in &r.reasons_for_mild_or_survival {
                    out.push_str("<li><strong>"); out.push_str(&escape(&reason.reason)); out.push_str(":</strong> "); out.push_str(&escape(&reason.explanation)); out.push_str("</li>");
                }
                out.push_str("</ul>\n</div>\n");
            }
        }
        out.push_str("</div>\n\n");
    }

    if let Some(variants) = variants_with_clinvar {
        let with_clinvar: Vec<_> = variants.iter().filter(|v| v.clinvar.is_some()).collect();
        let clinvar_first = variants.first().map(|v| format!("{}:{}", v.chromosome, v.position)).unwrap_or_else(|| "none".to_string());
        tap("09_clinvar", &format!("variants_len={} clinvar_count={} first={}", variants.len(), with_clinvar.len(), clinvar_first));
        if !with_clinvar.is_empty() {
            out.push_str("<!-- ClinVar reports -->\n<div id=\"clinvar\" class=\"content-section\">\n<h2>ClinVar reports</h2>\n<p><strong>Confidence / evidence:</strong> As provided by pipeline/MDNG; review status and classification per entry. Informational only; not for clinical diagnosis.</p>\n<p>Variants with ClinVar annotation (from pipeline/MDNG).</p>\n");
            for v in with_clinvar {
                let c = v.clinvar.as_ref().unwrap();
                out.push_str("<div class=\"condition-card\">\n<h3>");
                out.push_str(&escape(v.rsid.as_deref().unwrap_or("—")));
                out.push_str("</h3>\n<p><strong>Position:</strong> "); out.push_str(&escape(&v.chromosome)); out.push_str(":"); out.push_str(&v.position.to_string());
                if let (Some(ref r), Some(ref a)) = (&v.ref_allele, &v.alt_allele) {
                    write!(out, " | Ref: {} → Alt: {}", escape(r), escape(a)).unwrap();
                }
                out.push_str("</p>\n<p><strong>Classification:</strong> "); out.push_str(&escape(&c.classification));
                if !c.review_status.is_empty() {
                    out.push_str(" | <strong>Review:</strong> "); out.push_str(&escape(&c.review_status));
                }
                if !c.conditions.is_empty() {
                    out.push_str("</p>\n<p><strong>Conditions:</strong> "); out.push_str(&escape(&c.conditions.join("; ")));
                }
                if let Some(ref acc) = c.accession {
                    out.push_str(" | <strong>Accession:</strong> "); out.push_str(&escape(acc));
                }
                out.push_str("</p>\n");
                out.push_str(&variant_zygosity_confidence_html(v));
                out.push_str("</div>\n");
            }
            out.push_str("</div>\n\n");
        }
    }

    if let Some(star_list) = star_alleles {
        out.push_str("<!-- Star alleles (cross-check) -->\n<div id=\"staralleles\" class=\"content-section\">\n<h2>Star alleles: what they are and why they matter</h2>\n");
        out.push_str("<div class=\"recommendation-box\" style=\"border-left-color: #667eea;\">\n<h3 style=\"margin-top: 0;\">In plain language</h3>\n");
        out.push_str("<p><strong>Star alleles</strong> are simply named versions of genes that affect how your body processes certain medicines. Everyone has two copies of each of these genes (one from each parent). Depending on which versions you have, you may break down a drug <em>faster</em>, <em>slower</em>, or about <em>average</em>. That can influence the right dose or the choice of drug — nothing to worry about on its own; it’s information your doctor can use alongside your symptoms and history.</p>\n");
        out.push_str("<p>This section shows your <strong>inferred</strong> result for four genes (CYP2C19, CYP2D6, CYP2C9, CYP3A4) that many prescribers and guidelines use. <strong>Diplotype</strong> means your two copies (e.g. *1/*2). <strong>Effect</strong> is the usual interpretation: reference (normal), reduced function, loss of function, or <strong>increased function</strong>. <strong>Increased function is not the same as loss of function</strong> — it means the enzyme is more active (e.g. CYP2C19 *17); drug implications differ and are explained per drug in the Drug interactions tab. If your doctor isn’t familiar with star alleles, they can use the “Effect” line and the Drug interactions tab; guidelines (e.g. CPIC, PharmGKB) are available when needed.</p>\n");
        out.push_str("<p><strong>Confidence:</strong> These results are inferred from the variants in this dataset only. They are not a substitute for accredited pharmacogenomic testing when your clinician needs a definitive result (e.g. before a critical drug). Use this as a calm starting point for discussion.</p>\n</div>\n");
        let legend = star_allele_legend();
        out.push_str("<h3>Reference: alleles we check for</h3>\n<p>Below are the specific variants (rsIDs) we use to infer each star allele. <strong>*1</strong> means the reference (typical) version; other alleles are named *2, *3, *10, etc., and have a known effect (e.g. reduced function).</p>\n<table style=\"border-collapse: collapse; margin: 1em 0; width: 100%; max-width: 600px;\"><thead><tr style=\"background: #667eea; color: white;\"><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Gene</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Allele</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Defining variant (rsID)</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Effect</th></tr></thead><tbody>");
        for e in &legend {
            out.push_str("<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&e.gene));
            out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&e.allele));
            out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&e.rsid));
            out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&e.effect));
            out.push_str("</td></tr>");
        }
        out.push_str("</tbody></table>\n<h3>Your inferred results</h3>\n<p>For each gene below, <strong>diplotype</strong> is your two copies; <strong>effect</strong> is the usual interpretation. <strong>Reference = normal</strong> (no dose adjustment typically needed for that enzyme). <strong>Loss or reduced function</strong> = enzyme less active; discuss with prescriber (dose or alternative may apply). <strong>Increased function</strong> (e.g. CYP2C19 *17) = enzyme more active — the opposite of loss of function; implications are different and are spelled out per drug in the Drug interactions tab. Share this with your prescriber if you are starting or adjusting a medicine.</p>\n");
        for s in star_list {
            let is_reference = s.effect_labels.is_empty() || s.effect_labels.iter().all(|e| e.contains("reference"));
            out.push_str("<div class=\"condition-card\">\n<h3>"); out.push_str(&escape(&s.gene));
            if is_reference {
                out.push_str(" <span style=\"font-weight: normal; color: #28a745;\">— Normal (reference)</span>");
            } else {
                out.push_str(" <span style=\"font-weight: normal; color: #856404;\">— Discuss with clinician</span>");
            }
            out.push_str("</h3>\n<p><strong>Diplotype:</strong> "); out.push_str(&escape(&s.diplotype));
            if !s.alleles.is_empty() {
                out.push_str(" (detected alleles: "); out.push_str(&escape(&s.alleles.join(", "))); out.push_str(")");
            }
            if !s.effect_labels.is_empty() {
                out.push_str("</p>\n<p><strong>Effect:</strong> "); out.push_str(&escape(&s.effect_labels.join("; ")));
            } else {
                out.push_str("</p>\n<p><strong>Effect:</strong> reference (no variant detected in this set that changes function)");
            }
            out.push_str("</p>\n</div>\n");
        }
        if let Some(verification) = star_allele_verification {
            if !verification.is_empty() {
                out.push_str("<h3>Verification vs official star allele finder</h3>\n<p>When an official PGx tool (e.g. PharmCAT, StellarPGx) was run on the same sample, its calls are shown below for comparison. Agreement supports confidence in both; disagreement may reflect different allele panels or copy-number handling — discuss with your prescriber if important for a drug decision.</p>\n");
                out.push_str("<table style=\"border-collapse: collapse; margin: 1em 0; width: 100%; max-width: 700px;\"><thead><tr style=\"background: #667eea; color: white;\"><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Gene</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">This report</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Official finder</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Match</th></tr></thead><tbody>");
                for v in verification {
                    let match_str = if v.matches { "✓ Yes" } else { "— Consider external verification" };
                    let match_style = if v.matches { "color: #28a745;" } else { "color: #856404;" };
                    out.push_str("<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&v.gene));
                    out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&v.our_diplotype));
                    out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&v.official_diplotype));
                    out.push_str(" <span style=\"font-size: 0.9em; opacity: 0.85;\">("); out.push_str(&escape(&v.official_source)); out.push_str(")</span>");
                    out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd; "); out.push_str(match_style); out.push_str("\">"); out.push_str(match_str);
                    out.push_str("</td></tr>");
                }
                out.push_str("</tbody></table>\n");
            }
        }
        out.push_str("<h3>Example: venlafaxine</h3>\n<p>Venlafaxine is metabolized mainly by <strong>CYP2D6</strong>, with smaller roles for <strong>CYP2C19</strong> and <strong>CYP3A4</strong>. If CYP2D6 is reduced or loss of function, drug levels can be higher and the active metabolite lower; your doctor may consider dose or an alternative. The Drug interactions tab summarizes this for many drugs. Always discuss with your prescriber; this report is for education and shared decision‑making.</p>\n");

        // Pharmacopoeia: drug–enzyme interactions for all curated drugs
        let pharm = pharmacopoeia::run_pharmacopoeia_check(star_list);
        out.push_str("</div>\n\n<!-- Pharmacopoeia -->\n<div id=\"pharmacopoeia\" class=\"content-section\">\n<h2>Drug–enzyme interactions (pharmacopoeia)</h2>\n<p><strong>Confidence / evidence:</strong> Based on star allele inference in this set only; no copy-number or structural variants. For clinical use, confirm with accredited pharmacogenomic testing.</p>\n<p>Curated drugs and their main metabolizing enzymes, with a note for your inferred phenotype. Discuss any dose or drug choice with a prescriber.</p>\n<div class=\"recommendation-box\" style=\"border-left-color: #0c5460;\"><h4>If you have acute myeloid leukemia (AML) or other cancer</h4>\n<p>This report shows <strong>CYP</strong> phenotypes (Star alleles tab) that can affect some chemotherapy drugs: <strong>anthracyclines</strong> (daunorubicin, idarubicin, doxorubicin) and <strong>etoposide</strong> are metabolized by CYP3A4; your result there may inform dose or monitoring. <strong>DPYD</strong> (fluoropyrimidines like 5-FU) and <strong>TPMT</strong> (thiopurines like 6-mercaptopurine) are not in this report’s star-allele panel — ask your <strong>oncology team</strong> whether DPYD/TPMT testing is recommended for you. This report is for education and discussion only; never change or stop chemo based on this alone.</p>\n</div>\n<table style=\"border-collapse: collapse; margin: 1em 0; width: 100%;\"><thead><tr style=\"background: #667eea; color: white;\"><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Drug</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Class</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Summary for you</th></tr></thead><tbody>");
        for r in &pharm {
            out.push_str("<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&r.drug_name));
            out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(r.drug_class.as_deref().unwrap_or("—")));
            out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&r.summary));
            out.push_str("</td></tr>");
        }
        out.push_str("</tbody></table>\n<h3>Per-drug enzyme details</h3>\n");
        for r in &pharm {
            out.push_str("<div class=\"condition-card\"><h4>"); out.push_str(&escape(&r.drug_name));
            if let Some(ref c) = r.drug_class {
                out.push_str(" <span style=\"font-weight: normal; color: #666;\">("); out.push_str(&escape(c)); out.push_str(")</span>");
            }
            out.push_str("</h4>\n<p>"); out.push_str(&escape(&r.summary)); out.push_str("</p>\n");
            if !r.enzyme_notes.is_empty() {
                out.push_str("<ul style=\"margin: 0.5em 0; line-height: 1.5;\">");
                for n in &r.enzyme_notes {
                    let role_s = match n.role { EnzymeRole::Primary => "primary", EnzymeRole::Secondary => "secondary", EnzymeRole::Minor => "minor" };
                    out.push_str("<li><strong>"); out.push_str(&escape(&n.gene)); out.push_str("</strong> ("); out.push_str(role_s);
                    out.push_str("): "); out.push_str(&escape(&n.user_effect)); out.push_str(" — "); out.push_str(&escape(&n.recommendation));
                    out.push_str("</li>");
                }
                out.push_str("</ul>");
            }
            out.push_str("</div>\n");
        }
        out.push_str("</div>\n\n");
    }

    if let Some(parity) = sequencing_parity {
        out.push_str("<!-- Sequencing.com parity -->\n<div id=\"parity\" class=\"content-section\">\n<h2>Sequencing.com parity</h2>\n<p><strong>Confidence / evidence:</strong> rsID presence only; genotype and phenotype are not interpreted. Use to verify coverage vs. commercial report targets.</p>\n<p>We check the same variant targets that appear in Sequencing.com reports (HENRY_QUALITY_CROSSREF, LISA_QUALITY_CROSSREF). You should see at least what they found.</p>\n");
        write!(out, "<p>We found <strong>{} of the {}</strong> target rsIDs in your variant set (the other {} are not present in this run).</p>\n", parity.found_count, parity.total, parity.total.saturating_sub(parity.found_count)).unwrap();
        if !parity.found.is_empty() {
            out.push_str("<h3>Found (on par)</h3>\n<table style=\"border-collapse: collapse; margin: 1em 0;\"><thead><tr style=\"background: #667eea; color: white;\"><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">rsID</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Gene</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Condition</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Priority</th></tr></thead><tbody>");
            for t in &parity.found {
                out.push_str("<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&t.rsid));
                out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&t.gene));
                out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&t.condition));
                out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&t.priority));
                out.push_str("</td></tr>");
            }
            out.push_str("</tbody></table>\n");
        }
        if !parity.missing.is_empty() {
            out.push_str("<h3>Not in this variant set (check coverage or pipeline)</h3>\n<table style=\"border-collapse: collapse; margin: 1em 0;\"><thead><tr style=\"background: #dee2e6; color: #333;\"><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">rsID</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Gene</th><th style=\"padding: 8px; text-align: left; border: 1px solid #ddd;\">Condition</th></tr></thead><tbody>");
            for t in &parity.missing {
                out.push_str("<tr><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&t.rsid));
                out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&t.gene));
                out.push_str("</td><td style=\"padding: 8px; border: 1px solid #ddd;\">"); out.push_str(&escape(&t.condition));
                out.push_str("</td></tr>");
            }
            out.push_str("</tbody></table>\n");
        }
        out.push_str("</div>\n\n");
        out.push_str("<!-- Beyond Sequencing.com -->\n<div id=\"beyond\" class=\"content-section\">\n<h2>Beyond what Sequencing.com reports</h2>\n<p>We aim to be <strong>better</strong> than commercial reports. In addition to the targets above, this report includes:</p>\n<ul style=\"margin: 1em 0; line-height: 1.8;\">\n<li><strong>MCAS and mast cell–related</strong> (KIT, TPSAB1, SLK/Theodore's), with stabilizers and supplement guidance</li>\n<li><strong>Integrated cascade</strong> (CGRP/mast cell pathway scores, clearance needs)</li>\n<li><strong>Survival analysis</strong> (severe-phenotype genes and reasons for mild outcome)</li>\n<li><strong>Star alleles</strong> (CYP2C19, CYP2D6, CYP2C9, CYP3A4) for pharmacogenomics, with cross-check note</li>\n<li><strong>Drug–enzyme interactions (pharmacopoeia)</strong> — curated drugs vs your inferred CYP phenotypes</li>\n<li><strong>ClinVar</strong> when available (from pipeline/MDNG)</li>\n<li><strong>Disorders</strong> (psychiatric, autoimmune, neurological, metabolic) — susceptibility genes, not just rare genetic diseases</li>\n<li><strong>Supplements</strong> for weak areas (methylation, sulfur, etc.)</li>\n<li><strong>Your own extraction</strong> when you run FASTQ → pipeline → MDNG: variants in genomic order with full annotation</li>\n</ul>\n<p>Cross-check star alleles with StellarPGx or the pipeline when available. For clinical use, confirm with accredited testing.</p>\n</div>\n\n");
    }

    // Disclaimer
    out.push_str("<!-- Disclaimer -->\n<div id=\"disclaimer\" class=\"content-section\">\n<h2>Disclaimer</h2>\n<p>For research and educational use only. Not for clinical diagnosis. Gene associations are from published literature (e.g. PMC8540348, GeneReviews, OMIM); presence of variants does not establish diagnosis or indicate causation. Consult a qualified clinician for medical decisions.</p>\n<p><strong>Privacy:</strong> This report was produced entirely on your computer. No genetic data (VCF, variant list, or report) is sent to the internet or any third party. All processing is local.</p>\n</div>\n\n");

    out.push_str("</div>\n<button class=\"print-button\" onclick=\"window.print()\">🖨️ Print Report</button>\n\n<script>\nfunction showSection(sectionId) {\n  document.querySelectorAll('.content-section').forEach(function(s) { s.classList.remove('active'); });\n  document.querySelectorAll('.nav-tab').forEach(function(t) { t.classList.remove('active'); });\n  var el = document.getElementById(sectionId);\n  if (el) el.classList.add('active');\n  if (event && event.target) event.target.classList.add('active');\n}\n</script>\n</body>\n</html>");
    out
}

fn section_reports_inflammation(out: &mut String, reports: &[InflammationReport]) {
    // Worst first: reports with findings, then no findings
    for r in reports.iter().filter(|r| !r.findings.is_empty()).chain(reports.iter().filter(|r| r.findings.is_empty())) {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.condition_name));
        out.push_str("</h3>\n<p>Genes checked: ");
        out.push_str(&escape(&r.genes_checked.join(", ")));
        out.push_str("</p>\n<p>");
        out.push_str(&escape(&r.disclaimer));
        out.push_str("</p>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\">\n");
            out.push_str(&variant_finding_display_html(&f.variant, &f.note));
            out.push_str("</div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("</div>\n");
    }
}
