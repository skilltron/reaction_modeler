//! Mast cell activation syndrome (MCAS) and mastocytosis. References: PMC8540348, GeneReviews.
//! Includes MCAS stabilizer options and predicted-benefit combo (including cromolyn sodium).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MastCellDisorderKind {
    SystemicMastocytosis,
    CutaneousMastocytosis,
    Mcas,
    HereditaryAlphaTryptasemia,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MastCellDisorderRef {
    pub kind: MastCellDisorderKind,
    pub name: String,
    pub description: String,
    pub genes: Vec<String>,
    pub biomarkers: Vec<String>,
    pub criteria_notes: Vec<String>,
    pub references: Vec<String>,
}

pub fn mcas_mastocytosis_ref() -> Vec<MastCellDisorderRef> {
    vec![
        MastCellDisorderRef {
            kind: MastCellDisorderKind::SystemicMastocytosis,
            name: "Systemic mastocytosis (SM)".to_string(),
            description: "Clonal mast cell disorder; bone marrow and/or other organ involvement. KIT D816V mutation common; elevated serum tryptase typical.".to_string(),
            genes: vec!["KIT".to_string()],
            biomarkers: vec!["serum tryptase (often persistently elevated)".to_string(), "KIT D816V mutation".to_string(), "CD25+ mast cells".to_string()],
            criteria_notes: vec!["Major: multifocal mast cell infiltrates; minor: KIT D816V, tryptase >20 ng/mL, CD25".to_string()],
            references: vec!["PMC8540348 Mastocytosis and Mast Cell Activation Disorders".to_string(), "GeneReviews: Mastocytosis".to_string()],
        },
        MastCellDisorderRef {
            kind: MastCellDisorderKind::CutaneousMastocytosis,
            name: "Cutaneous mastocytosis (CM)".to_string(),
            description: "Mast cell proliferation limited to skin (e.g. urticaria pigmentosa).".to_string(),
            genes: vec!["KIT".to_string()],
            biomarkers: vec!["skin biopsy".to_string(), "tryptase (may be normal)".to_string()],
            criteria_notes: vec!["No systemic criteria; skin lesions with mast cell infiltrates.".to_string()],
            references: vec!["PMC8540348".to_string()],
        },
        MastCellDisorderRef {
            kind: MastCellDisorderKind::Mcas,
            name: "Mast cell activation syndrome (MCAS)".to_string(),
            description: "Episodic mast cell activation symptoms; tryptase often normal or mildly elevated; no clonal SM criteria.".to_string(),
            genes: vec!["KIT".to_string()],
            biomarkers: vec!["acute tryptase rise during episode (optional)".to_string(), "exclusion of SM and HαT when relevant".to_string()],
            criteria_notes: vec!["Consensus criteria: episodic symptoms, tryptase/mediator rise, response to treatment.".to_string()],
            references: vec!["PMC8540348".to_string()],
        },
        MastCellDisorderRef {
            kind: MastCellDisorderKind::HereditaryAlphaTryptasemia,
            name: "Hereditary alpha-tryptasemia (HαT)".to_string(),
            description: "Copy number gain of TPSAB1; elevated baseline serum tryptase. Can coexist with or mimic MCAS.".to_string(),
            genes: vec!["TPSAB1".to_string()],
            biomarkers: vec!["elevated baseline tryptase".to_string(), "TPSAB1 copy number (genetic testing)".to_string()],
            criteria_notes: vec!["Duplication of alpha-tryptase-encoding copy; not clonal.".to_string()],
            references: vec!["PMC8540348".to_string(), "HαT and tryptase genetics (Lyons et al.)".to_string()],
        },
    ]
}

// --- MCAS stabilizers: predicted benefits and combo recommendation ---

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McasStabilizer {
    pub name: String,
    /// Primary benefit domains (e.g. GI, systemic, CNS).
    pub benefit_domains: Vec<String>,
    /// Short rationale for predicted benefit.
    pub benefit_rationale: String,
    /// Typical dosing note (educational only).
    pub dosing_note: String,
    /// Why it fits in the recommended combo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub combo_rationale: Option<String>,
}

/// Reference list of MCAS stabilizers with predicted benefits. Includes cromolyn sodium.
pub fn mcas_stabilizers_ref() -> Vec<McasStabilizer> {
    vec![
        McasStabilizer {
            name: "Cromolyn sodium".to_string(),
            benefit_domains: vec![
                "GI (abdominal pain, diarrhea, nausea)".to_string(),
                "mucosal/systemic".to_string(),
                "flushing".to_string(),
            ],
            benefit_rationale: "Mast cell stabilizer; reduces degranulation. First-line for GI and systemic MCAS symptoms; oral solution (e.g. Gastrocrom) targets gut mucosa.".to_string(),
            dosing_note: "Oral: often 100–200 mg four times daily, titrated; ampules for oral use. Dosing is patient-specific; clinician must prescribe.".to_string(),
            combo_rationale: Some("Anchor stabilizer; best evidence in MCAS for GI and systemic. Combine with H1/H2 and optionally ketotifen for broad coverage.".to_string()),
        },
        McasStabilizer {
            name: "Ketotifen".to_string(),
            benefit_domains: vec![
                "systemic".to_string(),
                "CNS / blood–brain barrier".to_string(),
                "H1 overlap (reduces histamine load)".to_string(),
            ],
            benefit_rationale: "Mast cell stabilizer with H1 antihistamine effect; crosses blood–brain barrier. May help brain fog, fatigue, and systemic flares.".to_string(),
            dosing_note: "Typically 0.5–2 mg once or twice daily; start low. Prescription; dosing is clinician-directed.".to_string(),
            combo_rationale: Some("Complements cromolyn by covering systemic and CNS; additive to H1/H2.".to_string()),
        },
        McasStabilizer {
            name: "H1 blocker (e.g. cetirizine, fexofenadine)".to_string(),
            benefit_domains: vec![
                "skin (itching, urticaria)".to_string(),
                "rhinitis".to_string(),
                "flushing".to_string(),
            ],
            benefit_rationale: "Reduces histamine-mediated symptoms; standard in MCAS regimens.".to_string(),
            dosing_note: "Standard or higher-than-label dosing may be used in MCAS per specialist.".to_string(),
            combo_rationale: Some("Foundation with H2; works synergistically with cromolyn and ketotifen.".to_string()),
        },
        McasStabilizer {
            name: "H2 blocker (e.g. famotidine)".to_string(),
            benefit_domains: vec![
                "GI (acid, abdominal)".to_string(),
                "flushing".to_string(),
            ],
            benefit_rationale: "Reduces gastric acid and H2-mediated effects; part of standard MCAS dual blockade.".to_string(),
            dosing_note: "Often twice daily; dose and choice are clinician-directed.".to_string(),
            combo_rationale: Some("Dual H1+H2 improves symptom control; standard with cromolyn.".to_string()),
        },
        McasStabilizer {
            name: "Quercetin (with vitamin C)".to_string(),
            benefit_domains: vec!["antioxidant".to_string(), "mast cell stabilizer (in vitro)".to_string()],
            benefit_rationale: "Natural stabilizer and antioxidant; may support reduction in reactivity; evidence less robust than cromolyn/ketotifen.".to_string(),
            dosing_note: "Supplement; typical range 500–1000 mg quercetin; vitamin C often co-used. Not a substitute for prescription stabilizers.".to_string(),
            combo_rationale: Some("Optional add-on to prescription combo; may help some patients.".to_string()),
        },
    ]
}

/// Recommended MCAS stabilizer combo that includes cromolyn sodium for predicted best overall benefit.
/// Returns names in suggested order (cromolyn included).
pub fn mcas_recommended_combo_with_cromolyn() -> Vec<String> {
    vec![
        "Cromolyn sodium".to_string(),
        "H1 blocker (e.g. cetirizine, fexofenadine)".to_string(),
        "H2 blocker (e.g. famotidine)".to_string(),
        "Ketotifen".to_string(),
    ]
}
