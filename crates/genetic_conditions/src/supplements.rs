//! Supplement support for weak areas (≤8). For research/educational use only; not clinical advice.
//! Covers methylation, MCAS/inflammation, homocysteine/sulfur, and common deficiencies.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplementSupport {
    pub name: String,
    /// Weak area(s) this supports (e.g. methylation, MCAS, homocysteine).
    pub weak_areas: Vec<String>,
    pub rationale: String,
    /// Short typical-use note (e.g. form, range). Not prescribing.
    pub typical_note: String,
}

/// Up to 8 supplements that commonly support weak areas (methylation, MCAS, homocysteine, immune).
/// Order prioritizes areas often relevant when genetics point to these pathways.
pub fn supplements_for_weak_areas() -> Vec<SupplementSupport> {
    vec![
        SupplementSupport {
            name: "Methylfolate (5-MTHF)".to_string(),
            weak_areas: vec!["methylation".to_string(), "MTHFR pathway".to_string()],
            rationale: "Active folate; bypasses MTHFR. Supports methylation cycle, homocysteine clearance, neurotransmitter synthesis. Often low in MTHFR variants.".to_string(),
            typical_note: "400–800 mcg daily; start low. Not a substitute for medical care.".to_string(),
        },
        SupplementSupport {
            name: "B12 (methylcobalamin)".to_string(),
            weak_areas: vec!["methylation".to_string(), "homocysteine".to_string()],
            rationale: "Cofactor for methionine synthase; lowers homocysteine. Methyl form avoids conversion issues. Supports energy and nervous system.".to_string(),
            typical_note: "1000–5000 mcg oral or sublingual; dose is clinician-directed.".to_string(),
        },
        SupplementSupport {
            name: "B6 (P-5-P)".to_string(),
            weak_areas: vec!["homocysteine".to_string(), "sulfur metabolism".to_string()],
            rationale: "Cofactor for CBS and other transsulfuration steps. Supports homocysteine clearance and sulfur metabolism. P-5-P is active form.".to_string(),
            typical_note: "Typical range 25–50 mg; avoid very high long-term. Clinician-directed.".to_string(),
        },
        SupplementSupport {
            name: "Quercetin".to_string(),
            weak_areas: vec!["MCAS / mast cell".to_string(), "inflammation".to_string()],
            rationale: "Mast cell stabilizer and antioxidant. May reduce reactivity and histamine load; often used alongside vitamin C in MCAS support.".to_string(),
            typical_note: "500–1000 mg; often with vitamin C. Not a substitute for prescription stabilizers.".to_string(),
        },
        SupplementSupport {
            name: "Vitamin C".to_string(),
            weak_areas: vec!["MCAS / antioxidant".to_string(), "immune".to_string()],
            rationale: "Antioxidant; supports mast cell membrane stability and histamine breakdown. Commonly low; supports immune and connective tissue.".to_string(),
            typical_note: "500–2000 mg divided; buffered or liposomal if sensitive. Not for clinical diagnosis.".to_string(),
        },
        SupplementSupport {
            name: "Omega-3 (EPA/DHA)".to_string(),
            weak_areas: vec!["inflammation".to_string(), "immune".to_string()],
            rationale: "Anti-inflammatory; supports cell membranes and inflammatory balance. Often insufficient in diet.".to_string(),
            typical_note: "1–2 g EPA+DHA daily from quality fish oil. Discuss with clinician.".to_string(),
        },
        SupplementSupport {
            name: "Vitamin D".to_string(),
            weak_areas: vec!["immune".to_string(), "mood / bone".to_string()],
            rationale: "Frequently low; supports immune regulation and mood. Deficiency can worsen inflammation and fatigue.".to_string(),
            typical_note: "Dose by level (e.g. 1000–4000 IU); test and recheck. Clinician-directed.".to_string(),
        },
        SupplementSupport {
            name: "Magnesium".to_string(),
            weak_areas: vec!["nervous system".to_string(), "methylation cofactor".to_string()],
            rationale: "Cofactor for many enzymes; often low. Supports muscle, nerve, and stress response; relevant when methylation or CNS is a concern.".to_string(),
            typical_note: "Glycinate or citrate 200–400 mg; start low. Avoid in kidney disease without guidance.".to_string(),
        },
    ]
}
