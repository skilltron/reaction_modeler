//! HTML report generator: same style as Enhanced psychiatric report, with expanded MCAS and related conditions.
//! For research and educational use only; not for clinical diagnosis.

use crate::cascade;
use crate::inflammation::{self, InflammationReport};
use crate::supplements;
use crate::survival;
use crate::AllConditionsReport;
use std::fmt::Write;

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

/// Build full HTML report including expanded MCAS and MCAS-related conditions (PMC8540348, SLK/Theodore's).
/// Optionally include integrated cascade report and survival analysis (severe-phenotype genes).
pub fn all_conditions_to_html(
    report: &AllConditionsReport,
    report_title: &str,
    report_date: &str,
    cascade_report: Option<&cascade::IntegratedCascadeReport>,
    survival_analysis: Option<&survival::SurvivalAnalysis>,
) -> String {
    let mcas_refs = inflammation::mcas_mastocytosis_ref();
    let slk_ref = inflammation::slk_theodores_ref();

    let mut out = String::with_capacity(64 * 1024);
    out.push_str("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n<meta charset=\"UTF-8\">\n<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n<title>");
    out.push_str(&escape(report_title));
    out.push_str("</title>\n<style>");
    out.push_str(REPORT_CSS);
    out.push_str("</style>\n</head>\n<body>\n<div class=\"container\">\n<header>\n<h1>Genetic Conditions Report</h1>\n<p style=\"opacity: 0.9;\">");
    out.push_str(&escape(report_title));
    out.push_str("</p>\n<div class=\"patient-info\">\n<div class=\"info-card\"><strong>Report date:</strong> ");
    out.push_str(&escape(report_date));
    out.push_str("</div>\n<div class=\"info-card\"><strong>Disclaimer:</strong> For research and educational use only. Not for clinical diagnosis.</div>\n</div>\n</header>\n\n<div class=\"nav-tabs\">\n");
    write!(out, r#"<div class="nav-tab active" onclick="showSection('mcas')">🩺 MCAS &amp; related</div>"#).unwrap();
    out.push_str("\n<div class=\"nav-tab\" onclick=\"showSection('inflammation')\">Inflammation findings</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('immune')\">Immune</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('exposure')\">Exposure</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('sulfur')\">Sulfur</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('rare')\">Rare</div>\n");
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('supplements')\">Supplements</div>\n");
    if cascade_report.is_some() {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('cascade')\">Cascade</div>\n");
    }
    if survival_analysis.is_some() {
        out.push_str("<div class=\"nav-tab\" onclick=\"showSection('survival')\">Survival</div>\n");
    }
    out.push_str("<div class=\"nav-tab\" onclick=\"showSection('disclaimer')\">Disclaimer</div>\n</div>\n\n");

    // MCAS & related (active)
    out.push_str("<!-- MCAS & related -->\n<div id=\"mcas\" class=\"content-section active\">\n<h2>MCAS and mast cell–related conditions (expanded reference)</h2>\n");
    out.push_str("<p>Reference: PMC8540348 (Mastocytosis and Mast Cell Activation Disorders), GeneReviews, and related literature.</p>\n");

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
    out.push_str("</p>\n</div>\n");
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

    // Inflammation findings (variants in KIT, TPSAB1)
    let has_inflammation_findings = report.inflammation.iter().any(|r| !r.findings.is_empty());
    if has_inflammation_findings {
        out.push_str("<h3>Variant findings in MCAS/mastocytosis genes (KIT, TPSAB1)</h3>\n");
        for r in &report.inflammation {
            for f in &r.findings {
                out.push_str("<div class=\"gene-variant\">\n<h4>");
                out.push_str(&escape(&f.gene));
                out.push_str(" — ");
                out.push_str(&escape(&r.condition_name));
                out.push_str("</h4>\n<p>");
                out.push_str(&escape(&f.note));
                if let (Some(ref_a), Some(alt_a)) = (&f.reference_allele, &f.alternate_allele) {
                    write!(out, " Ref: {} → Alt: {}.", escape(ref_a), escape(alt_a)).unwrap();
                }
                if let Some(rt) = &f.region_type {
                    write!(out, " Region: {}.", escape(rt.as_str())).unwrap();
                }
                out.push_str("</p>\n</div>\n");
            }
        }
    } else {
        out.push_str("<p class=\"alert alert-info\">No variants in KIT or TPSAB1 in the provided variant set.</p>\n");
    }
    out.push_str("</div>\n\n");

    // Inflammation tab (all inflammation reports)
    out.push_str("<!-- Inflammation findings -->\n<div id=\"inflammation\" class=\"content-section\">\n<h2>Inflammation (MCAS / mastocytosis) reports</h2>\n");
    section_reports_inflammation(&mut out, &report.inflammation);
    out.push_str("</div>\n\n");

    // Immune
    out.push_str("<!-- Immune -->\n<div id=\"immune\" class=\"content-section\">\n<h2>Immune disease reports</h2>\n");
    for r in &report.immune {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.disease_name));
        out.push_str("</h3>\n<p>Genes checked: ");
        out.push_str(&escape(&r.genes_checked.join(", ")));
        out.push_str("</p>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\"><p>");
            out.push_str(&escape(&f.note));
            out.push_str("</p></div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n\n");

    // Exposure
    out.push_str("<!-- Exposure -->\n<div id=\"exposure\" class=\"content-section\">\n<h2>Chemical exposure reports</h2>\n");
    for r in &report.exposure {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.chemical_name));
        out.push_str("</h3>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\"><p>");
            out.push_str(&escape(&f.note));
            out.push_str("</p></div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n\n");

    // Sulfur
    out.push_str("<!-- Sulfur -->\n<div id=\"sulfur\" class=\"content-section\">\n<h2>Sulfur metabolism reports</h2>\n");
    for r in &report.sulfur {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.condition_name));
        out.push_str("</h3>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\"><p>");
            out.push_str(&escape(&f.note));
            out.push_str("</p></div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n\n");

    // Rare
    out.push_str("<!-- Rare -->\n<div id=\"rare\" class=\"content-section\">\n<h2>Rare disease reports</h2>\n");
    for r in &report.rare {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.disease_name));
        out.push_str("</h3>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\"><p>");
            out.push_str(&escape(&f.note));
            out.push_str("</p></div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("</div>\n");
    }
    out.push_str("</div>\n\n");

    // Supplements for weak areas (≤8)
    let supps = supplements::supplements_for_weak_areas();
    out.push_str("<!-- Supplements -->\n<div id=\"supplements\" class=\"content-section\">\n<h2>Supplement support for weak areas (≤8)</h2>\n<p>These commonly support methylation, MCAS/inflammation, homocysteine, and immune pathways. For research and educational use only; not clinical advice. Dosing and suitability must be determined by a clinician.</p>\n");
    for s in &supps {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&s.name));
        out.push_str("</h3>\n<p><strong>Weak areas:</strong> ");
        out.push_str(&escape(&s.weak_areas.join(", ")));
        out.push_str("</p>\n<p>");
        out.push_str(&escape(&s.rationale));
        out.push_str("</p>\n<p><strong>Typical note:</strong> ");
        out.push_str(&escape(&s.typical_note));
        out.push_str("</p>\n</div>\n");
    }
    out.push_str("</div>\n\n");

    if let Some(cr) = cascade_report {
        out.push_str("<!-- Integrated Cascade -->\n<div id=\"cascade\" class=\"content-section\">\n<h2>Integrated cascade summary</h2>\n<p>Genotype-driven scores and pathway ranking. Phenotype can refine. For research only.</p>\n");
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
        out.push_str("</ol>\n<h3>Suspected buildup / clearance</h3>\n");
        for b in &cr.suspected_buildups {
            out.push_str("<div class=\"condition-card\"><h4>"); out.push_str(&escape(&b.category)); out.push_str("</h4>\n<p>"); out.push_str(&escape(&b.why_may_accumulate)); out.push_str("</p>\n<p><strong>Clearance:</strong> "); out.push_str(&escape(&b.clearance_category)); out.push_str("</p></div>\n");
        }
        out.push_str("</div>\n\n");
    }

    if let Some(sa) = survival_analysis {
        out.push_str("<!-- Survival analysis -->\n<div id=\"survival\" class=\"content-section\">\n<h2>Severe-phenotype genes: possible reasons for mild or survivable outcome</h2>\n<p>"); out.push_str(&escape(&sa.summary)); out.push_str("</p>\n");
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

    // Disclaimer
    out.push_str("<!-- Disclaimer -->\n<div id=\"disclaimer\" class=\"content-section\">\n<h2>Disclaimer</h2>\n<p>For research and educational use only. Not for clinical diagnosis. Gene associations are from published literature (e.g. PMC8540348, GeneReviews, OMIM); presence of variants does not establish diagnosis or indicate causation. Consult a qualified clinician for medical decisions.</p>\n</div>\n\n");

    out.push_str("</div>\n<button class=\"print-button\" onclick=\"window.print()\">🖨️ Print Report</button>\n\n<script>\nfunction showSection(sectionId) {\n  document.querySelectorAll('.content-section').forEach(function(s) { s.classList.remove('active'); });\n  document.querySelectorAll('.nav-tab').forEach(function(t) { t.classList.remove('active'); });\n  var el = document.getElementById(sectionId);\n  if (el) el.classList.add('active');\n  if (event && event.target) event.target.classList.add('active');\n}\n</script>\n</body>\n</html>");
    out
}

fn section_reports_inflammation(out: &mut String, reports: &[InflammationReport]) {
    for r in reports {
        out.push_str("<div class=\"condition-card\">\n<h3>");
        out.push_str(&escape(&r.condition_name));
        out.push_str("</h3>\n<p>Genes checked: ");
        out.push_str(&escape(&r.genes_checked.join(", ")));
        out.push_str("</p>\n<p>");
        out.push_str(&escape(&r.disclaimer));
        out.push_str("</p>\n");
        for f in &r.findings {
            out.push_str("<div class=\"gene-variant\"><p>");
            out.push_str(&escape(&f.note));
            out.push_str("</p></div>\n");
        }
        if r.findings.is_empty() {
            out.push_str("<p>No matching variants.</p>\n");
        }
        out.push_str("</div>\n");
    }
}
