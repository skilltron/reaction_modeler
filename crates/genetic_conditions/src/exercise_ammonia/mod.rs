//! Exercise Ammonia / Nitrogen Waste Handling integrated genetics analysis.
//! Level 3: urea cycle, amino acid catabolism, mitochondrial stress, purine/AMP deamination,
//! redox/sulfur, electrolyte/cofactor, mast-cell/inflammatory cross-talk.
//! For research and educational use only; not for clinical diagnosis.

use crate::variant_input::{RegionType, VariantInput};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// --- Pathway definitions ---

#[derive(Debug, Clone)]
pub struct ExerciseAmmoniaPathwayRef {
    pub id: &'static str,
    pub name: &'static str,
    pub genes: Vec<&'static str>,
    pub mechanism_note: &'static str,
    pub waste_or_cleanup: &'static str,
}

fn pathway_refs() -> Vec<ExerciseAmmoniaPathwayRef> {
    vec![
        ExerciseAmmoniaPathwayRef {
            id: "urea_cycle",
            name: "Urea cycle / ammonia disposal",
            genes: vec!["CPS1", "OTC", "ASS1", "ASL", "ARG1", "ARG2", "NAGS", "SLC25A15", "ORNT1"],
            mechanism_note: "CPS1 (carbamoyl phosphate); OTC (ornithine transcarbamylase); ASS1, ASL (argininosuccinate); ARG1/ARG2 (arginase); NAGS; SLC25A15 (ORNT1 mitochondrial ornithine). Ammonia → urea throughput.",
            waste_or_cleanup: "Ammonia buffering and urea-cycle throughput; bottleneck step determines clearance capacity.",
        },
        ExerciseAmmoniaPathwayRef {
            id: "amino_acid_catabolism",
            name: "Amino acid catabolism / nitrogen load",
            genes: vec![
                "BCKDHA", "BCKDHB", "DBT", "BCAT2", "GLUD1", "GLUL", "GLS", "GOT1", "GOT2", "GPT", "GPT2",
                "ALDH4A1", "GLDC", "GCSH", "PSAT1", "PHGDH", "SHMT1", "SHMT2",
            ],
            mechanism_note: "BCAA catabolism (BCKDH, BCAT2); glutamate/glutamine (GLUD1, GLUL, GLS); transamination (GOT, GPT); glycine/serine (GLDC, GCSH, SHMT, PHGDH). Nitrogen from protein/amino acid use during exercise.",
            waste_or_cleanup: "Nitrogen produced from amino acid breakdown; poor handling increases ammonia load.",
        },
        ExerciseAmmoniaPathwayRef {
            id: "mitochondrial",
            name: "Mitochondrial energy stress",
            genes: vec![
                "NDUFS1", "NDUFS2", "NDUFV1", "SDHA", "SDHB", "UQCRC1", "COX4I1", "ATP5A1", "ATP5F1",
                "CPT1A", "CPT2", "SLC25A20", "PDHA1", "PDHB", "DLD", "LDHA", "LDHB", "PC", "ACADVL",
            ],
            mechanism_note: "ETC (NDUFS, SDH, UQCRC, COX); ATP synthase; carnitine/beta-oxidation (CPT1A, CPT2, SLC25A20, ACADVL); pyruvate (PDHA1, PDHB, PC); lactate (LDHA, LDHB). Poor ATP → AMP breakdown, fallback fuel, excess waste.",
            waste_or_cleanup: "Mitochondrial inefficiency increases metabolic stress and AMP deamination tendency.",
        },
        ExerciseAmmoniaPathwayRef {
            id: "purine_amp",
            name: "Purine nucleotide cycle / AMP deamination",
            genes: vec!["AMPD1", "AMPD2", "AMPD3", "ADA", "ADSL", "ATIC", "IMPDH1", "IMPDH2"],
            mechanism_note: "AMPD1/2/3 (AMP deaminase); adenosine deaminase (ADA); purine synthesis (ADSL, ATIC, IMPDH). ATP depletion → AMP → IMP + ammonia during high exertion.",
            waste_or_cleanup: "AMP deamination produces ammonia; high ATP stress drives disproportionate ammonia release.",
        },
        ExerciseAmmoniaPathwayRef {
            id: "redox_sulfur",
            name: "Redox / detox / sulfur handling",
            genes: vec![
                "GSS", "GCLC", "GCLM", "GPX1", "GPX4", "SOD1", "SOD2", "CAT", "GSTP1", "GSTM1", "GSTT1",
                "SUOX", "CBS", "CTH", "MTHFR", "MTR", "MTRR",
            ],
            mechanism_note: "Glutathione (GSS, GCLC, GCLM); SOD/catalase/GPX; GST; sulfur (SUOX, CBS, CTH). Oxidative stress reduces tolerance to nitrogen burden.",
            waste_or_cleanup: "Cleanup capacity; poor redox/sulfur handling worsens ammonia and metabolic waste tolerance.",
        },
        ExerciseAmmoniaPathwayRef {
            id: "electrolyte_cofactor",
            name: "Electrolyte / mineral cofactor support",
            genes: vec![
                "TRPM6", "TRPM7", "SLC41A1", "TPK1", "SLC19A2", "SLC19A3", "MTHFR", "MTR", "MTRR",
                "NNT", "SLC25A19", "SLC22A5", "OCTN2", "GAMT", "GATM", "ARG1", "ASS1", "ASL",
            ],
            mechanism_note: "Magnesium (TRPM6/7, SLC41A1); B1 (TPK1, SLC19A2/3); B2/B3 (NNT); B6/folate (MTHFR, MTR, MTRR); carnitine (SLC22A5/OCTN2); creatine (GAMT, GATM). Urea-cycle cofactors (ARG1, ASS1, ASL).",
            waste_or_cleanup: "Cofactor support for urea cycle and mitochondrial function; only when genetically justified.",
        },
    ]
}

// --- Finding and pathway report ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseAmmoniaFinding {
    pub gene: String,
    pub variant: VariantInput,
    pub reference_allele: Option<String>,
    pub alternate_allele: Option<String>,
    pub region_type: Option<RegionType>,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseAmmoniaPathwayReport {
    pub pathway_id: String,
    pub pathway_name: String,
    pub genes_checked: Vec<String>,
    pub findings: Vec<ExerciseAmmoniaFinding>,
    pub baseline_function_pct: u8,
    pub mechanism_note: String,
    pub consequence_during_exercise: String,
    pub likely_symptom_expression: String,
    pub waste_accumulation: String,
    pub cleanup_adequate: String,
}

/// Risk level for exercise ammonia / nitrogen clearance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseAmmoniaRiskLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseAmmoniaConfidence {
    Low,
    Medium,
    High,
}

/// Root-cause pattern classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootCausePattern {
    PrimaryUreaCycleWeakness,
    MitochondrialAtpStress,
    AminoAcidOveruse,
    PurineAmpDeamination,
    RedoxDetoxBottleneck,
    InflammatoryMastCellAmplified,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseAmmoniaIntervention {
    pub name: String,
    pub effectiveness_1_to_10: u8,
    pub reasoning: String,
    pub timing_note: Option<String>,
    pub precautions: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInterpretation {
    pub why_ammonia_during_exercise: String,
    pub production_vs_disposal: String,
    pub mast_cell_inflammatory_contribution: String,
    pub interventions_most_likely_to_reduce: String,
}

const USER_CONTEXT_NOTE: &str = "User-relevant context: ammonia smell during hard exercise; mast-cell stabilizers have reduced knee pain and gut pain; possible mitochondrial involvement; possible calcium-trigger / CGRP / neuroinflammatory overlap. Cascade issues, waste buildup from broken processes, and cleanup limitations are explicitly analyzed.";

/// Full Exercise Ammonia / Nitrogen Waste Handling report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseAmmoniaReport {
    pub risk_level: ExerciseAmmoniaRiskLevel,
    pub risk_confidence: ExerciseAmmoniaConfidence,
    pub root_cause_pattern: RootCausePattern,
    pub root_cause_reasoning: String,
    pub pathway_reports: Vec<ExerciseAmmoniaPathwayReport>,
    pub cascade_narratives: Vec<String>,
    pub interventions: Vec<ExerciseAmmoniaIntervention>,
    pub user_interpretation: UserInterpretation,
    pub symptom_inference_note: String,
    pub user_context_note: String,
    /// CPS1 deficiency / CPS1 check explicitly called out (previously requested).
    pub cps1_check_note: String,
}

/// Run full Exercise Ammonia / Nitrogen Waste Handling analysis.
pub fn run_exercise_ammonia_analysis(
    variants: &[VariantInput],
    inflammation_finding_count: usize,
) -> ExerciseAmmoniaReport {
    let pathways = pathway_refs();
    let mut pathway_reports = Vec::with_capacity(pathways.len());
    let mut total_finding_count: usize = 0;

    for p in &pathways {
        let gene_set: HashSet<String> = p.genes.iter().map(|s| (*s).to_uppercase()).collect();
        let mut seen = HashSet::new();
        let mut findings = Vec::new();
        for v in variants {
            let gene = match &v.gene {
                Some(g) => g.to_uppercase(),
                None => continue,
            };
            if !gene_set.contains(&gene) {
                continue;
            }
            let key = v.dedup_key();
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);
            let region_note = v.region_type.map(|r| format!(" Region: {}.", r.as_str())).unwrap_or_default();
            findings.push(ExerciseAmmoniaFinding {
                gene: gene.clone(),
                variant: v.clone(),
                reference_allele: v.ref_allele.clone(),
                alternate_allele: v.alt_allele.clone(),
                region_type: v.region_type,
                note: format!(
                    "Variant in {} ({}); may affect nitrogen waste handling during exercise.{}",
                    gene,
                    p.name,
                    region_note
                ),
            });
        }
        total_finding_count += findings.len();
        let impact = (findings.len() * 18).min(100);
        let baseline_pct = 100u8.saturating_sub(impact as u8);
        let (consequence, symptoms, waste, cleanup) = consequence_and_cleanup(p.id, findings.len());
        pathway_reports.push(ExerciseAmmoniaPathwayReport {
            pathway_id: p.id.to_string(),
            pathway_name: p.name.to_string(),
            genes_checked: p.genes.iter().map(|s| (*s).to_string()).collect(),
            findings,
            baseline_function_pct: baseline_pct,
            mechanism_note: p.mechanism_note.to_string(),
            consequence_during_exercise: consequence,
            likely_symptom_expression: symptoms,
            waste_accumulation: waste,
            cleanup_adequate: cleanup,
        });
    }

    let (risk_level, risk_confidence) = compute_risk(total_finding_count, &pathway_reports);
    let (root_cause_pattern, root_cause_reasoning) =
        classify_root_cause(&pathway_reports, inflammation_finding_count);
    let cascade_narratives = build_cascade_narratives(&pathway_reports, inflammation_finding_count);
    let interventions = build_interventions(&pathway_reports, root_cause_pattern);
    let user_interpretation = build_user_interpretation(&pathway_reports, root_cause_pattern, &interventions);
    let cps1_note = build_cps1_check_note(&pathway_reports);

    ExerciseAmmoniaReport {
        risk_level,
        risk_confidence,
        root_cause_pattern,
        root_cause_reasoning,
        pathway_reports: pathway_reports.clone(),
        cascade_narratives,
        interventions,
        user_interpretation,
        symptom_inference_note: symptom_inference_note(),
        user_context_note: USER_CONTEXT_NOTE.to_string(),
        cps1_check_note: cps1_note,
    }
}

fn consequence_and_cleanup(
    pathway_id: &str,
    finding_count: usize,
) -> (String, String, String, String) {
    if finding_count == 0 {
        return (
            "No variants in this pathway in this set.".to_string(),
            "—".to_string(),
            "—".to_string(),
            "Insufficient data to assess.".to_string(),
        );
    }
    match pathway_id {
        "urea_cycle" => (
            format!(
                "Urea cycle throughput may be impaired; {} variant(s) in CPS1/OTC/ASS1/ASL/ARG/NAGS. Ammonia from exercise may accumulate if disposal is the bottleneck.",
                finding_count
            ),
            "Ammonia smell during hard exercise; exercise intolerance; possible hyperammonaemia under load.".to_string(),
            "Ammonia may persist longer; urea cycle is primary disposal route.".to_string(),
            "Cleanup capacity may be reduced; urea cycle is likely bottleneck.".to_string(),
        ),
        "amino_acid_catabolism" => (
            format!(
                "Amino acid catabolism or nitrogen handling may be affected; {} variant(s). Protein/amino acids may be used more aggressively as fuel during hard exertion, increasing nitrogen burden.",
                finding_count
            ),
            "Rapid fatigue; muscle burn; possible ammonia smell when using amino acids for fuel.".to_string(),
            "Nitrogen from amino acid breakdown may exceed disposal capacity.".to_string(),
            "Depends on urea cycle and redox; if both impaired, cleanup inadequate.".to_string(),
        ),
        "mitochondrial" => (
            format!(
                "Mitochondrial efficiency may be reduced; {} variant(s). Poor ATP generation may force AMP breakdown and fallback to less efficient fuel use, increasing ammonia and lactate.",
                finding_count
            ),
            "Early fatigue; muscle burn; heat intolerance; post-exertional malaise; lactate/ammonia mixed pattern.".to_string(),
            "AMP deamination and lactate may rise; mitochondrial stress increases metabolic waste.".to_string(),
            "Mitochondrial support may improve ATP and reduce AMP-driven ammonia.".to_string(),
        ),
        "purine_amp" => (
            format!(
                "AMP deamination or purine cycle may be affected; {} variant(s). ATP depletion during high exertion could drive disproportionate ammonia release via AMPD.",
                finding_count
            ),
            "Ammonia smell during high-intensity work; rapid fatigue when ATP demand exceeds supply.".to_string(),
            "IMP and ammonia from AMP deamination during adenylate energy stress.".to_string(),
            "Ribose or nucleotide support only if mechanistically justified; improve ATP capacity to reduce AMP stress.".to_string(),
        ),
        "redox_sulfur" => (
            format!(
                "Redox or sulfur handling may be strained; {} variant(s). Oxidative stress may reduce tolerance to nitrogen and metabolic waste.",
                finding_count
            ),
            "Chemical sensitivity; worse recovery; ammonia smell may be more noticeable when redox is overwhelmed.".to_string(),
            "Glutathione/sulfur burden; oxidative stress worsens waste tolerance.".to_string(),
            "Antioxidant support with sulfur sensitivity caution; cleanup may be inadequate under load.".to_string(),
        ),
        "electrolyte_cofactor" => (
            format!(
                "Cofactor support for urea cycle or mitochondria may be affected; {} variant(s). Mg, B vitamins, carnitine, or arginine/citrulline support may be justified.",
                finding_count
            ),
            "Cramping; fatigue; poor recovery; cofactor-dependent steps may be rate-limiting.".to_string(),
            "Indirect: cofactor deficiency can worsen urea cycle and mitochondrial function.".to_string(),
            "Supplement only when genetically justified; avoid over-supplementation.".to_string(),
        ),
        _ => (
            "Pathway impact from variant count.".to_string(),
            "See pathway-specific literature.".to_string(),
            "—".to_string(),
            "—".to_string(),
        ),
    }
}

fn compute_risk(
    total_findings: usize,
    pathway_reports: &[ExerciseAmmoniaPathwayReport],
) -> (ExerciseAmmoniaRiskLevel, ExerciseAmmoniaConfidence) {
    let impacted = pathway_reports.iter().filter(|r| !r.findings.is_empty()).count();
    let confidence = if total_findings >= 3 && impacted >= 2 {
        ExerciseAmmoniaConfidence::High
    } else if total_findings >= 1 || impacted >= 1 {
        ExerciseAmmoniaConfidence::Medium
    } else {
        ExerciseAmmoniaConfidence::Low
    };
    let level = match (total_findings, impacted) {
        (0, 0) => ExerciseAmmoniaRiskLevel::Low,
        (1..=2, 1) => ExerciseAmmoniaRiskLevel::Low,
        (1..=4, 2) | (3..=4, 1) => ExerciseAmmoniaRiskLevel::Moderate,
        (5..=9, _) | (_, 3) => ExerciseAmmoniaRiskLevel::High,
        _ => ExerciseAmmoniaRiskLevel::VeryHigh,
    };
    (level, confidence)
}

fn classify_root_cause(
    pathway_reports: &[ExerciseAmmoniaPathwayReport],
    inflammation_finding_count: usize,
) -> (RootCausePattern, String) {
    let urea = pathway_reports.iter().find(|r| r.pathway_id == "urea_cycle").map(|r| r.findings.len()).unwrap_or(0);
    let mito = pathway_reports.iter().find(|r| r.pathway_id == "mitochondrial").map(|r| r.findings.len()).unwrap_or(0);
    let aa = pathway_reports.iter().find(|r| r.pathway_id == "amino_acid_catabolism").map(|r| r.findings.len()).unwrap_or(0);
    let purine = pathway_reports.iter().find(|r| r.pathway_id == "purine_amp").map(|r| r.findings.len()).unwrap_or(0);
    let redox = pathway_reports.iter().find(|r| r.pathway_id == "redox_sulfur").map(|r| r.findings.len()).unwrap_or(0);

    if urea > 0 && mito == 0 && aa == 0 && inflammation_finding_count == 0 {
        (
            RootCausePattern::PrimaryUreaCycleWeakness,
            "Urea cycle variants (e.g. CPS1, OTC, ASS1, ASL, ARG1) suggest primary ammonia disposal weakness.".to_string(),
        )
    } else if mito > 0 && urea == 0 && purine == 0 {
        (
            RootCausePattern::MitochondrialAtpStress,
            "Mitochondrial variants suggest ATP stress pattern; AMP breakdown and fallback fuel use may drive ammonia.".to_string(),
        )
    } else if aa > 0 && urea == 0 && mito == 0 {
        (
            RootCausePattern::AminoAcidOveruse,
            "Amino acid catabolism variants suggest protein/amino acid overuse as fuel during exertion, increasing nitrogen load.".to_string(),
        )
    } else if purine > 0 && urea == 0 {
        (
            RootCausePattern::PurineAmpDeamination,
            "Purine/AMP deamination pathway variants suggest adenylate energy stress driving ammonia release during high exertion.".to_string(),
        )
    } else if redox > 0 && urea == 0 && mito == 0 && inflammation_finding_count == 0 {
        (
            RootCausePattern::RedoxDetoxBottleneck,
            "Redox/sulfur pathway variants suggest detox bottleneck; oxidative stress may reduce tolerance to nitrogen burden.".to_string(),
        )
    } else if inflammation_finding_count > 0 && (urea > 0 || mito > 0 || redox > 0) {
        (
            RootCausePattern::InflammatoryMastCellAmplified,
            "Mast-cell/inflammation findings plus nitrogen-pathway variants suggest inflammatory amplification of exercise intolerance and ammonia stress.".to_string(),
        )
    } else if urea > 0 || mito > 0 || aa > 0 || purine > 0 || redox > 0 {
        (
            RootCausePattern::Mixed,
            "Multiple pathway impacts suggest mixed pattern: production overload and/or disposal weakness, with possible inflammatory amplification.".to_string(),
        )
    } else {
        (
            RootCausePattern::Mixed,
            "Insufficient variant data to classify; consider phenotype and symptom pattern.".to_string(),
        )
    }
}

fn build_cascade_narratives(
    pathway_reports: &[ExerciseAmmoniaPathwayReport],
    inflammation_finding_count: usize,
) -> Vec<String> {
    let mut out = Vec::new();
    let urea = pathway_reports.iter().find(|r| r.pathway_id == "urea_cycle");
    let mito = pathway_reports.iter().find(|r| r.pathway_id == "mitochondrial");
    let aa = pathway_reports.iter().find(|r| r.pathway_id == "amino_acid_catabolism");
    let purine = pathway_reports.iter().find(|r| r.pathway_id == "purine_amp");
    let redox = pathway_reports.iter().find(|r| r.pathway_id == "redox_sulfur");

    if mito.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("Mitochondrial weakness → ATP stress → AMP deamination → ammonia rise during hard exercise.".to_string());
    }
    if urea.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("Poor urea cycle throughput (e.g. CPS1, OTC, ASS1, ASL, ARG1) → ammonia persistence → exercise intolerance and ammonia smell.".to_string());
    }
    if aa.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("Amino acid use for fuel → increased nitrogen burden → disposal may be overwhelmed.".to_string());
    }
    if purine.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("AMP deamination (AMPD) during adenylate stress → IMP + ammonia → smell during high-intensity work.".to_string());
    }
    if redox.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("Redox weakness → worse tolerance to metabolic waste and nitrogen burden.".to_string());
    }
    if inflammation_finding_count > 0 {
        out.push("Inflammation / mast-cell activation → reduced recovery and lower threshold for overload; mast-cell stabilizers reducing knee/gut pain is biologically plausible.".to_string());
    }
    if urea.map(|r| !r.findings.is_empty()).unwrap_or(false) || mito.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("Gut barrier or liver-related support may be relevant where inferable; systemic cleanup can be impaired.".to_string());
    }
    if out.is_empty() {
        out.push("No variant-driven cascade identified in this set.".to_string());
    }
    out
}

fn build_interventions(
    pathway_reports: &[ExerciseAmmoniaPathwayReport],
    _root_cause: RootCausePattern,
) -> Vec<ExerciseAmmoniaIntervention> {
    let urea = pathway_reports.iter().find(|r| r.pathway_id == "urea_cycle").map(|r| !r.findings.is_empty()).unwrap_or(false);
    let mito = pathway_reports.iter().find(|r| r.pathway_id == "mitochondrial").map(|r| !r.findings.is_empty()).unwrap_or(false);
    let redox = pathway_reports.iter().find(|r| r.pathway_id == "redox_sulfur").map(|r| !r.findings.is_empty()).unwrap_or(false);
    let cofactor = pathway_reports.iter().find(|r| r.pathway_id == "electrolyte_cofactor").map(|r| !r.findings.is_empty()).unwrap_or(false);

    let mut list = vec![
        ExerciseAmmoniaIntervention {
            name: "Magnesium".to_string(),
            effectiveness_1_to_10: if mito || cofactor { 7 } else { 5 },
            reasoning: "Cofactor for ATP, glycolysis, and muscle; may support mitochondrial and electrolyte balance.".to_string(),
            timing_note: Some("Daily; consider around training.".to_string()),
            precautions: Some("Avoid excess; renal function.".to_string()),
        },
        ExerciseAmmoniaIntervention {
            name: "Citrulline".to_string(),
            effectiveness_1_to_10: if urea { 8 } else { 5 },
            reasoning: if urea {
                "Urea cycle substrate; may support ammonia disposal when cycle variants suggest bottleneck.".to_string()
            } else {
                "Urea cycle support; consider when ammonia disposal is suspected weak.".to_string()
            },
            timing_note: Some("Pre- or peri-workout; discuss dose with clinician.".to_string()),
            precautions: Some("Not a substitute for medical management of hyperammonaemia.".to_string()),
        },
        ExerciseAmmoniaIntervention {
            name: "Ornithine".to_string(),
            effectiveness_1_to_10: if urea { 7 } else { 4 },
            reasoning: if urea {
                "Urea cycle intermediate; may support OTC/ASS1/ASL step when genetically indicated.".to_string()
            } else {
                "Urea cycle support when disposal is bottleneck.".to_string()
            },
            timing_note: None,
            precautions: None,
        },
        ExerciseAmmoniaIntervention {
            name: "Arginine".to_string(),
            effectiveness_1_to_10: if urea { 6 } else { 4 },
            reasoning: "Urea cycle; precursor to nitric oxide. Support when arginase or upstream steps are limiting.".to_string(),
            timing_note: None,
            precautions: Some("Caution with herpes; discuss with clinician.".to_string()),
        },
        ExerciseAmmoniaIntervention {
            name: "Carnitine".to_string(),
            effectiveness_1_to_10: if mito { 7 } else { 5 },
            reasoning: if mito {
                "Mitochondrial fatty acid oxidation; support when CPT or beta-oxidation stress is suggested.".to_string()
            } else {
                "Supports fatty acid oxidation and mitochondrial function.".to_string()
            },
            timing_note: None,
            precautions: None,
        },
        ExerciseAmmoniaIntervention {
            name: "B-complex (B1, B2, B3, B6)".to_string(),
            effectiveness_1_to_10: if cofactor || mito { 7 } else { 5 },
            reasoning: if cofactor {
                "Genetically justified when cofactor-pathway variants present; B1/B2/B3/B6 support energy and nitrogen handling.".to_string()
            } else {
                "Support for mitochondrial and cofactor-dependent steps; only when justified.".to_string()
            },
            timing_note: Some("With meals; form and dose clinician-directed.".to_string()),
            precautions: Some("Avoid excessive B6; monitor.".to_string()),
        },
        ExerciseAmmoniaIntervention {
            name: "Ribose".to_string(),
            effectiveness_1_to_10: 4,
            reasoning: "Purine/nucleotide precursor; only if AMP/adenylate stress pattern is clear and mechanistically justified.".to_string(),
            timing_note: None,
            precautions: Some("Evidence mixed; use only when pattern fits.".to_string()),
        },
        ExerciseAmmoniaIntervention {
            name: "Mitochondrial support (CoQ10, PQQ, etc.)".to_string(),
            effectiveness_1_to_10: if mito { 7 } else { 5 },
            reasoning: if mito {
                "Mitochondrial variants suggest support may improve ATP and reduce AMP-driven ammonia.".to_string()
            } else {
                "General mitochondrial support; consider when fatigue or ATP stress is reported.".to_string()
            },
            timing_note: None,
            precautions: None,
        },
        ExerciseAmmoniaIntervention {
            name: "Hydration / electrolyte support".to_string(),
            effectiveness_1_to_10: 6,
            reasoning: "Adequate hydration and electrolytes support renal and systemic waste clearance.".to_string(),
            timing_note: Some("Before, during, and after hard exercise.".to_string()),
            precautions: None,
        },
        ExerciseAmmoniaIntervention {
            name: "Lower-protein pre-workout (if amino acid overuse)".to_string(),
            effectiveness_1_to_10: 5,
            reasoning: "If amino acid catabolism appears to contribute, reducing protein load before high-intensity work may lower nitrogen burden.".to_string(),
            timing_note: Some("Trial with clinician or dietitian.".to_string()),
            precautions: Some("Do not restrict protein overall; timing only.".to_string()),
        },
        ExerciseAmmoniaIntervention {
            name: "Lower-intensity aerobic base building".to_string(),
            effectiveness_1_to_10: 6,
            reasoning: "If high-intensity work is the specific trigger for ammonia smell, building aerobic base may reduce ATP stress and AMP deamination.".to_string(),
            timing_note: None,
            precautions: None,
        },
        ExerciseAmmoniaIntervention {
            name: "Mast-cell stabilizers (if exercise-triggered inflammation)".to_string(),
            effectiveness_1_to_10: 6,
            reasoning: "If inflammatory/mast-cell amplification is likely, stabilizers may reduce exercise-triggered intolerance and improve recovery.".to_string(),
            timing_note: None,
            precautions: Some("Prescription options (cromolyn, ketotifen) are clinician-directed.".to_string()),
        },
        ExerciseAmmoniaIntervention {
            name: "Antioxidant / glutathione support".to_string(),
            effectiveness_1_to_10: if redox { 6 } else { 4 },
            reasoning: if redox {
                "Redox pathway impact; antioxidant support may help with sulfur sensitivity caution.".to_string()
            } else {
                "With sulfur sensitivity caution where appropriate.".to_string()
            },
            timing_note: None,
            precautions: Some("Caution if sulfur intolerance or SUOX/CBS issues.".to_string()),
        },
    ];
    list.sort_by(|a, b| b.effectiveness_1_to_10.cmp(&a.effectiveness_1_to_10));
    list
}

fn build_user_interpretation(
    pathway_reports: &[ExerciseAmmoniaPathwayReport],
    _root_cause: RootCausePattern,
    interventions: &[ExerciseAmmoniaIntervention],
) -> UserInterpretation {
    let urea = pathway_reports.iter().find(|r| r.pathway_id == "urea_cycle").map(|r| !r.findings.is_empty()).unwrap_or(false);
    let mito = pathway_reports.iter().find(|r| r.pathway_id == "mitochondrial").map(|r| !r.findings.is_empty()).unwrap_or(false);
    let why = if urea && !mito {
        "Ammonia during hard exercise is likely driven by impaired disposal through the urea cycle (e.g. CPS1, OTC, ASS1, ASL, ARG1), so ammonia produced by muscle and liver accumulates."
    } else if mito && !urea {
        "Ammonia may be driven by mitochondrial ATP stress: poor ATP generation leads to AMP breakdown (deamination), producing ammonia; fallback to less efficient fuel use increases metabolic waste."
    } else if urea && mito {
        "Both disposal weakness (urea cycle) and production overload (mitochondrial stress → AMP deamination) may contribute; ammonia accumulates when production exceeds clearance."
    } else {
        "Variant data in this set do not strongly point to a single mechanism; production overload, disposal weakness, or both are possible depending on phenotype."
    };
    let prod_vs_disp = if urea && mito {
        "Both: disposal may be weak (urea cycle) and production may be elevated (AMP deamination, amino acid use)."
    } else if urea {
        "Primarily disposal weakness: urea cycle throughput appears to be the bottleneck."
    } else if mito {
        "Primarily production overload or ATP stress driving AMP deamination; disposal may be adequate at rest but overwhelmed during exertion."
    } else {
        "Unclear from genetics alone; consider symptom pattern and response to interventions."
    };
    let mast = "Mast-cell stabilizers reducing knee and gut pain makes biological sense if inflammation or mast-cell activation worsens exercise tolerance and recovery; inflammatory amplification can lower the threshold for metabolic overload and ammonia stress.";
    let top: Vec<&str> = interventions.iter().take(4).map(|i| i.name.as_str()).collect();
    let interventions_list = top.join(", ");
    UserInterpretation {
        why_ammonia_during_exercise: why.to_string(),
        production_vs_disposal: prod_vs_disp.to_string(),
        mast_cell_inflammatory_contribution: mast.to_string(),
        interventions_most_likely_to_reduce: format!("Highest-ranked from this analysis: {}. Discuss timing and dose with your clinician.", interventions_list),
    }
}

fn symptom_inference_note() -> String {
    "Symptoms that can narrow the pattern: ammonia smell during hard exercise; rapid fatigue; muscle burn out of proportion; post-exertional malaise; headaches or migraines after exertion; heat intolerance; flushing; joint pain; gut symptoms after exertion; brain fog after exercise; poor recovery after high-intensity training; benefit from mast-cell stabilizers. Share with your clinician to align with phenotype.".to_string()
}

fn build_cps1_check_note(pathway_reports: &[ExerciseAmmoniaPathwayReport]) -> String {
    let urea = pathway_reports.iter().find(|r| r.pathway_id == "urea_cycle");
    let cps1_findings = urea.map(|r| r.findings.iter().filter(|f| f.gene.eq_ignore_ascii_case("CPS1")).count()).unwrap_or(0);
    if cps1_findings > 0 {
        "CPS1 (carbamoyl phosphate synthetase 1) variant(s) were detected in this set. CPS1 is the first and rate-limiting step of the urea cycle. Deficiency or reduced function can cause hyperammonaemia, especially under metabolic stress (e.g. illness, prolonged exercise). This report does not diagnose CPS1 deficiency; discuss with your clinician and consider dedicated biochemical or genetic testing if clinically indicated.".to_string()
    } else {
        "CPS1 (carbamoyl phosphate synthetase 1) was included in the urea-cycle gene set; no CPS1 variants were found in this variant set. CPS1 deficiency is a known cause of urea cycle disorder and hyperammonaemia; absence of variants here does not rule out deficiency (coverage or variant type may differ).".to_string()
    }
}
