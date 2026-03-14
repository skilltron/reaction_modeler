//! MCAS / Mast Cell Instability integrated genetics analysis.
//! Level 3: pathway breakdown, risk score, cascade narrative, intervention ranking, symptom/mediator inference.
//! For research and educational use only; not for clinical diagnosis.

use crate::variant_input::{RegionType, VariantInput};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

// --- Pathway definitions (genes per pathway group) ---

/// One pathway group for mast-cell instability analysis.
#[derive(Debug, Clone)]
pub struct McasPathwayRef {
    pub id: &'static str,
    pub name: &'static str,
    pub genes: Vec<&'static str>,
    pub mechanism_note: &'static str,
    pub promotes: Vec<&'static str>, // e.g. "degranulation", "histamine excess"
}

fn pathway_refs() -> Vec<McasPathwayRef> {
    vec![
        McasPathwayRef {
            id: "signaling",
            name: "Mast cell activation / degranulation signaling",
            genes: vec![
                "KIT", "TPSAB1", "FCER1A", "FCER1G", "SYK", "LYN", "LAT", "PLCG1", "PLCG2",
                "CACNA1C", "CACNA1H", "ORAI1", "STIM1", "MAPK1", "MAPK3", "NFKB1", "NFKB2",
                "PTGS1", "PTGS2", "ALOX5", "LTC4S",
            ],
            mechanism_note: "KIT/c-KIT, IgE receptor (FCER1), SYK/LYN/LAT/PLCG cascade; calcium (CACNA, ORAI/STIM); MAPK/NF-kB; prostaglandin (PTGS) and leukotriene (ALOX5, LTC4S) production.",
            promotes: vec!["degranulation", "calcium-trigger sensitivity", "prostaglandin excess", "leukotriene excess"],
        },
        McasPathwayRef {
            id: "histamine",
            name: "Histamine production / breakdown / clearance",
            genes: vec![
                "HDC", "HNMT", "AOC1", "ABP1", "DAO", "MTHFR", "MTR", "MTRR", "BHMT", "COMT",
                "MAOA", "MAOB", "ALDH2",
            ],
            mechanism_note: "HDC (synthesis); HNMT (intracellular), AOC1/DAO (extracellular) breakdown; methylation (MTHFR, MTR, MTRR, BHMT, COMT); MAO/ALDH for amine load.",
            promotes: vec!["histamine excess", "poor histamine clearance", "waste / byproduct accumulation"],
        },
        McasPathwayRef {
            id: "cytokine",
            name: "Cytokine / allergic skew / immune bias",
            genes: vec![
                "IL4", "IL13", "IL4R", "IL13RA1", "STAT6", "GATA3", "TNF", "TNFRSF1A", "IL6", "IL6R",
                "TLR4", "TLR2", "CD14",
            ],
            mechanism_note: "IL4/IL13/IL4R/STAT6 Th2 skew; TNF/IL6 inflammatory amplification; TLR innate immune overactivation.",
            promotes: vec!["neuroinflammation", "inflammatory amplification"],
        },
        McasPathwayRef {
            id: "oxidative_sulfur",
            name: "Oxidative stress / detox / sulfur handling",
            genes: vec![
                "GSTP1", "GSTM1", "GSTT1", "GSTA1", "SOD1", "SOD2", "CAT", "GPX1", "GPX4",
                "SUOX", "CBS", "CTH", "MTHFR", "MTR", "MTRR", "GSS", "GCLC", "GCLM",
            ],
            mechanism_note: "GST/SOD/catalase/GPX; glutathione (GSS, GCLC, GCLM); sulfur/sulfite (SUOX, CBS, CTH). Cross-talk with sulfur/mast cell pathway.",
            promotes: vec!["waste / byproduct accumulation", "amplified mast-cell instability"],
        },
        McasPathwayRef {
            id: "neuroimmune",
            name: "Neuroimmune and mediator effects",
            genes: vec![
                "CALCA", "RAMP1", "CRLR", "TRPV1", "TRPV4", "PTGER2", "PTGER4", "TBXAS1",
                "HPGDS", "TPSAB1", "TPSB2",
            ],
            mechanism_note: "CGRP (CALCA, RAMP1, CRLR); TRPV calcium-mediated neurogenic inflammation; prostaglandin D2 (HPGDS), thromboxane (TBXAS1); tryptase (TPSAB1, TPSB2).",
            promotes: vec!["neuroinflammation", "calcium-trigger sensitivity", "prostaglandin excess", "mast-cell contribution to pain/flushing/migraine"],
        },
    ]
}

// --- Finding and pathway report ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McasPathwayFinding {
    pub gene: String,
    pub variant: VariantInput,
    pub reference_allele: Option<String>,
    pub alternate_allele: Option<String>,
    pub region_type: Option<RegionType>,
    pub note: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McasPathwayReport {
    pub pathway_id: String,
    pub pathway_name: String,
    pub genes_checked: Vec<String>,
    pub findings: Vec<McasPathwayFinding>,
    /// Baseline function % (100 = no impact; heuristic from finding count).
    pub baseline_function_pct: u8,
    pub mechanism_note: String,
    pub expected_consequence: String,
    pub likely_symptom_expression: String,
    pub promotes: Vec<String>,
}

/// Risk level for MCAS / mast cell instability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McasRiskLevel {
    Low,
    Moderate,
    High,
    VeryHigh,
}

/// Confidence in risk score based on variant coverage and pathway coherence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum McasConfidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McasIntervention {
    pub name: String,
    pub effectiveness_1_to_10: u8,
    pub reasoning: String,
    pub timing_note: Option<String>,
    pub precautions: Option<String>,
}

/// Inferred dominant mediator pattern from pathway/symptom logic.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediatorPattern {
    HistamineDominant,
    ProstaglandinDominant,
    LeukotrieneDominant,
    MixedMediator,
    CalciumTriggerNeuroimmune,
    Unclear,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymptomMediatorInference {
    pub dominant_pattern: MediatorPattern,
    pub reasoning: String,
    pub user_pattern_note: String,
}

/// Full MCAS / Mast Cell Instability integrated report.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McasIntegratedReport {
    pub risk_level: McasRiskLevel,
    pub risk_confidence: McasConfidence,
    pub pathway_reports: Vec<McasPathwayReport>,
    pub cascade_narratives: Vec<String>,
    pub interventions: Vec<McasIntervention>,
    pub symptom_inference: SymptomMediatorInference,
    pub user_context_note: String,
}

const USER_CONTEXT_NOTE: &str = "User-relevant pattern: mast-cell stabilizers have reduced knee pain and gut pain; possible calcium-trigger sensitivity; possible CGRP/neuroinflammatory overlap; ammonia smell with hard exercise may indicate nitrogen waste handling stress that could worsen inflammatory burden.";

/// Run full MCAS integrated analysis on variants.
pub fn run_mcas_integrated_analysis(variants: &[VariantInput]) -> McasIntegratedReport {
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
            findings.push(McasPathwayFinding {
                gene: gene.clone(),
                variant: v.clone(),
                reference_allele: v.ref_allele.clone(),
                alternate_allele: v.alt_allele.clone(),
                region_type: v.region_type,
                note: format!(
                    "Variant in {} ({}); may affect {} pathway.{}",
                    gene,
                    p.name,
                    p.id,
                    region_note
                ),
            });
        }
        total_finding_count += findings.len();
        let impact = (findings.len() * 15).min(100);
        let baseline_pct = 100u8.saturating_sub(impact as u8);
        let (expected_consequence, likely_symptom) = consequence_and_symptoms(p.id, findings.len());
        pathway_reports.push(McasPathwayReport {
            pathway_id: p.id.to_string(),
            pathway_name: p.name.to_string(),
            genes_checked: p.genes.iter().map(|s| (*s).to_string()).collect(),
            findings,
            baseline_function_pct: baseline_pct,
            mechanism_note: p.mechanism_note.to_string(),
            expected_consequence: expected_consequence,
            likely_symptom_expression: likely_symptom,
            promotes: p.promotes.iter().map(|s| (*s).to_string()).collect(),
        });
    }

    let (risk_level, risk_confidence) = compute_risk(total_finding_count, &pathway_reports);
    let cascade_narratives = build_cascade_narratives(&pathway_reports);
    let interventions = build_interventions(&pathway_reports, risk_level);
    let symptom_inference = infer_mediator_pattern(&pathway_reports);

    McasIntegratedReport {
        risk_level,
        risk_confidence,
        pathway_reports,
        cascade_narratives,
        interventions,
        symptom_inference,
        user_context_note: USER_CONTEXT_NOTE.to_string(),
    }
}

fn consequence_and_symptoms(pathway_id: &str, finding_count: usize) -> (String, String) {
    let (conseq, symptoms) = match (pathway_id, finding_count) {
        (_, 0) => ("No variants detected in this pathway in this set.".to_string(), "—".to_string()),
        ("signaling", n) => (
            format!("Mast cell signaling may be dysregulated; {} variant(s) in KIT/FCER1/SYK/calcium/MAPK/prostaglandin/leukotriene nodes.", n),
            "Easier degranulation; calcium-trigger sensitivity; flushing, gut pain, migraine tendency.".to_string(),
        ),
        ("histamine", n) => (
            format!("Histamine production or clearance may be affected; {} variant(s) in HDC/HNMT/DAO/methylation nodes.", n),
            "Histamine excess or prolonged half-life; flushing, itch, gut pain, headache, nasal congestion.".to_string(),
        ),
        ("cytokine", n) => (
            format!("Th2 or inflammatory skew possible; {} variant(s) in IL4/IL13/TNF/IL6/TLR nodes.", n),
            "Allergic or inflammatory amplification; sinus, skin, gut, neuroinflammation.".to_string(),
        ),
        ("oxidative_sulfur", n) => (
            format!("Oxidative stress or sulfur handling may be strained; {} variant(s) in GST/SOD/GPX/SUOX/glutathione nodes.", n),
            "Redox or sulfite burden can amplify mast cell instability; chemical sensitivity, headache.".to_string(),
        ),
        ("neuroimmune", n) => (
            format!("Neuroimmune or mediator nodes affected; {} variant(s) in CGRP/TRPV/prostaglandin D2/tryptase-related nodes.", n),
            "CGRP/neurogenic inflammation; joint pain, gut pain, flushing, burning skin, migraine.".to_string(),
        ),
        _ => (
            "Pathway impact from variant count.".to_string(),
            "See pathway-specific literature.".to_string(),
        ),
    };
    (conseq, symptoms)
}

fn compute_risk(total_findings: usize, pathway_reports: &[McasPathwayReport]) -> (McasRiskLevel, McasConfidence) {
    let impacted_pathways = pathway_reports.iter().filter(|r| !r.findings.is_empty()).count();
    let confidence = if total_findings >= 3 && impacted_pathways >= 2 {
        McasConfidence::High
    } else if total_findings >= 1 || impacted_pathways >= 1 {
        McasConfidence::Medium
    } else {
        McasConfidence::Low
    };
    let level = match (total_findings, impacted_pathways) {
        (0, 0) => McasRiskLevel::Low,
        (1..=2, 1) => McasRiskLevel::Low,
        (1..=4, 2) | (3..=4, 1) => McasRiskLevel::Moderate,
        (5..=8, _) | (_, 3) => McasRiskLevel::High,
        _ => McasRiskLevel::VeryHigh,
    };
    (level, confidence)
}

fn build_cascade_narratives(pathway_reports: &[McasPathwayReport]) -> Vec<String> {
    let mut out = Vec::new();
    let histamine = pathway_reports.iter().find(|r| r.pathway_id == "histamine");
    let signaling = pathway_reports.iter().find(|r| r.pathway_id == "signaling");
    let oxidative = pathway_reports.iter().find(|r| r.pathway_id == "oxidative_sulfur");
    let neuro = pathway_reports.iter().find(|r| r.pathway_id == "neuroimmune");

    if histamine.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("Impaired DAO/HNMT or methylation support → higher histamine burden and prolonged mediator half-life.".to_string());
    }
    if signaling.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("Calcium or KIT/FCER1/SYK dysregulation → easier mast-cell triggering and degranulation.".to_string());
    }
    if histamine.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("Methylation weakness (MTHFR/MTR/MTRR/COMT) → slower amine clearance and potential histamine/prostaglandin accumulation.".to_string());
    }
    if oxidative.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("Oxidative stress or sulfur burden → amplified mast-cell instability and mediator release.".to_string());
    }
    if neuro.map(|r| !r.findings.is_empty()).unwrap_or(false) || signaling.map(|r| !r.findings.is_empty()).unwrap_or(false) {
        out.push("Mast-cell activation → CGRP / pain / gut / migraine worsening and neuroimmune amplification.".to_string());
    }
    let any_findings = pathway_reports.iter().any(|r| !r.findings.is_empty());
    if any_findings {
        out.push("Downstream waste buildup from broken processes; cleanup pathways (DAO, HNMT, glutathione, sulfite) may also be impaired.".to_string());
    }
    if out.is_empty() {
        out.push("No variant-driven cascade identified in this set.".to_string());
    }
    out
}

fn build_interventions(pathway_reports: &[McasPathwayReport], _risk_level: McasRiskLevel) -> Vec<McasIntervention> {
    let histamine_impact = pathway_reports.iter().find(|r| r.pathway_id == "histamine").map(|r| !r.findings.is_empty()).unwrap_or(false);
    let signaling_impact = pathway_reports.iter().find(|r| r.pathway_id == "signaling").map(|r| !r.findings.is_empty()).unwrap_or(false);
    let oxidative_impact = pathway_reports.iter().find(|r| r.pathway_id == "oxidative_sulfur").map(|r| !r.findings.is_empty()).unwrap_or(false);

    let mut list = vec![
        McasIntervention {
            name: "Quercetin".to_string(),
            effectiveness_1_to_10: 8,
            reasoning: "Mast cell membrane stabiliser; reduces degranulation. Works with vitamin C.".to_string(),
            timing_note: Some("With meals; consistent use for stabilisation.".to_string()),
            precautions: Some("Discuss dose with clinician.".to_string()),
        },
        McasIntervention {
            name: "Luteolin".to_string(),
            effectiveness_1_to_10: 7,
            reasoning: "Mast cell stabiliser; anti-inflammatory; crosses blood–brain barrier.".to_string(),
            timing_note: None,
            precautions: None,
        },
        McasIntervention {
            name: "Apigenin".to_string(),
            effectiveness_1_to_10: 5,
            reasoning: "Flavonoid; mast cell stabiliser in vitro; anti-inflammatory; effect in MCAS is likely modest; often paired with luteolin.".to_string(),
            timing_note: None,
            precautions: Some("Lower evidence than quercetin/PEA; consider as optional add-on.".to_string()),
        },
        McasIntervention {
            name: "PEA (palmitoylethanolamide)".to_string(),
            effectiveness_1_to_10: 7,
            reasoning: "Endocannabinoid-like; mast cell stabilisation and anti-inflammatory; supports neuropathic pain; used in MCAS.".to_string(),
            timing_note: Some("Typical 300–600 mg twice daily; titrate with clinician.".to_string()),
            precautions: None,
        },
        McasIntervention {
            name: "Vitamin C".to_string(),
            effectiveness_1_to_10: 7,
            reasoning: "Antioxidant; supports histamine breakdown; complements quercetin.".to_string(),
            timing_note: None,
            precautions: None,
        },
        McasIntervention {
            name: "Cromolyn sodium".to_string(),
            effectiveness_1_to_10: 9,
            reasoning: "Anchor mast cell stabiliser; reduces degranulation. Prescription.".to_string(),
            timing_note: Some("Oral often 100–200 mg QID; titrate with clinician.".to_string()),
            precautions: Some("Prescription only; clinician-directed.".to_string()),
        },
        McasIntervention {
            name: "Ketotifen".to_string(),
            effectiveness_1_to_10: 8,
            reasoning: "Mast cell stabiliser; H1 blocker; systemic and CNS coverage.".to_string(),
            timing_note: Some("Typically 0.5–2 mg daily; start low.".to_string()),
            precautions: Some("Prescription only.".to_string()),
        },
        McasIntervention {
            name: "DAO support".to_string(),
            effectiveness_1_to_10: if histamine_impact { 8 } else { 6 },
            reasoning: if histamine_impact {
                "Genetically supported: AOC1/DAO or histamine-pathway variants; enzyme support may reduce dietary histamine load.".to_string()
            } else {
                "Support extracellular histamine breakdown when dietary histamine is a trigger.".to_string()
            },
            timing_note: Some("With high-histamine meals.".to_string()),
            precautions: None,
        },
        McasIntervention {
            name: "Magnesium".to_string(),
            effectiveness_1_to_10: if signaling_impact { 7 } else { 5 },
            reasoning: if signaling_impact {
                "Calcium-trigger sensitivity suggested by signaling-pathway variants; magnesium may modulate calcium excitability.".to_string()
            } else {
                "General support for calcium/membrane stability.".to_string()
            },
            timing_note: None,
            precautions: None,
        },
        McasIntervention {
            name: "Low-histamine / mast-cell-friendly diet".to_string(),
            effectiveness_1_to_10: 7,
            reasoning: "Reduces exogenous histamine and common triggers.".to_string(),
            timing_note: None,
            precautions: None,
        },
        McasIntervention {
            name: "Methylation support (B12, methylfolate, B6)".to_string(),
            effectiveness_1_to_10: if histamine_impact { 7 } else { 4 },
            reasoning: if histamine_impact {
                "Genetically justified when MTHFR/MTR/MTRR/COMT variants present; supports amine clearance.".to_string()
            } else {
                "Consider only when methylation genes support it.".to_string()
            },
            timing_note: Some("Discuss form and dose with clinician.".to_string()),
            precautions: Some("Do not overmethylate; monitor.".to_string()),
        },
        McasIntervention {
            name: "Glutathione / NAC".to_string(),
            effectiveness_1_to_10: if oxidative_impact { 6 } else { 5 },
            reasoning: if oxidative_impact {
                "Oxidative/sulfur pathway impact; glutathione support may help but caution if sulfur sensitivity.".to_string()
            } else {
                "Antioxidant support; use with caution if sulfur or mast-cell sensitivity.".to_string()
            },
            timing_note: None,
            precautions: Some("Caution if sulfur intolerance or SUOX/CBS issues.".to_string()),
        },
        McasIntervention {
            name: "Prostaglandin / leukotriene support".to_string(),
            effectiveness_1_to_10: if signaling_impact { 6 } else { 4 },
            reasoning: if signaling_impact {
                "PTGS/ALOX5/LTC4S pathway variants may justify targeted support (e.g. omega-3, avoid excess omega-6).".to_string()
            } else {
                "Consider when mediator pattern suggests PG/LT dominance.".to_string()
            },
            timing_note: None,
            precautions: None,
        },
        McasIntervention {
            name: "Ammi visnaga / khella (chromone)".to_string(),
            effectiveness_1_to_10: 5,
            reasoning: "Chromone logic may be mechanistically relevant for some mast-cell/calcium effects.".to_string(),
            timing_note: None,
            precautions: Some("Safety and phototoxicity uncertainty; do not overstate; discuss with clinician.".to_string()),
        },
    ];

    // Sort by effectiveness descending
    list.sort_by(|a, b| b.effectiveness_1_to_10.cmp(&a.effectiveness_1_to_10));
    list
}

fn infer_mediator_pattern(pathway_reports: &[McasPathwayReport]) -> SymptomMediatorInference {
    let histamine = pathway_reports.iter().find(|r| r.pathway_id == "histamine").map(|r| r.findings.len()).unwrap_or(0);
    let signaling = pathway_reports.iter().find(|r| r.pathway_id == "signaling").map(|r| r.findings.len()).unwrap_or(0);
    let neuro = pathway_reports.iter().find(|r| r.pathway_id == "neuroimmune").map(|r| r.findings.len()).unwrap_or(0);
    let cytokine = pathway_reports.iter().find(|r| r.pathway_id == "cytokine").map(|r| r.findings.len()).unwrap_or(0);

    let (pattern, reasoning) = if histamine > 0 && signaling == 0 && neuro == 0 {
        (MediatorPattern::HistamineDominant, "Histamine-pathway variants (HDC/HNMT/DAO/methylation) suggest histamine-dominant burden.".to_string())
    } else if signaling > 0 && histamine == 0 && neuro == 0 {
        (MediatorPattern::ProstaglandinDominant, "Signaling-pathway variants (PTGS/ALOX5) may favour prostaglandin/leukotriene production.".to_string())
    } else if signaling > 0 && neuro > 0 {
        (MediatorPattern::CalciumTriggerNeuroimmune, "Signaling and neuroimmune (CGRP/TRPV) variants suggest calcium-trigger and neuroimmune amplification.".to_string())
    } else if histamine > 0 && (signaling > 0 || neuro > 0) {
        (MediatorPattern::MixedMediator, "Histamine plus signaling or neuroimmune variants suggest mixed mediator pattern.".to_string())
    } else if cytokine > 0 && (histamine > 0 || signaling > 0) {
        (MediatorPattern::MixedMediator, "Cytokine skew with histamine or signaling impact suggests mixed inflammatory and mediator load.".to_string())
    } else if histamine > 0 {
        (MediatorPattern::HistamineDominant, "Histamine clearance or production variants dominate.".to_string())
    } else if signaling > 0 {
        (MediatorPattern::LeukotrieneDominant, "Signaling-pathway variants may increase leukotriene/prostaglandin output.".to_string())
    } else {
        (MediatorPattern::Unclear, "Insufficient variant data in this set to infer dominant mediator pattern.".to_string())
    };

    SymptomMediatorInference {
        dominant_pattern: pattern,
        reasoning,
        user_pattern_note: "If you have: flushing, burning skin, itch, gut pain, diarrhea, reflux, migraines, sinus symptoms, chemical sensitivity, heat or exercise intolerance, ammonia smell with hard exercise, or knee/connective-tissue pain improved by mast-cell stabilisers — these can help narrow whether histamine, prostaglandin, leukotriene, or calcium-trigger/neuroimmune pattern fits best. Share with your clinician.".to_string(),
    }
}
