//! Supplement support for weak areas (≤8). For research/educational use only; not clinical advice.
//! Covers methylation, MCAS/inflammation, homocysteine/sulfur, and common deficiencies.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplementSupport {
    pub name: String,
    /// Weak area(s) this supports (e.g. methylation, MCAS, homocysteine).
    pub weak_areas: Vec<String>,
    /// Gene/pathway effects this supplement counteracts — so you know where the failure is.
    pub gene_effects_counteracted: Vec<String>,
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
            gene_effects_counteracted: vec![
                "MTHFR (C677T/A1298C): reduced conversion of folate to 5-MTHF — bypasses the enzyme.".to_string(),
                "Elevated homocysteine when MTHFR is slow — supports remethylation to methionine.".to_string(),
            ],
            rationale: "Active folate; bypasses MTHFR. Supports methylation cycle, homocysteine clearance, neurotransmitter synthesis. Often low in MTHFR variants.".to_string(),
            typical_note: "400–800 mcg daily; start low. Not a substitute for medical care.".to_string(),
        },
        SupplementSupport {
            name: "B12 (methylcobalamin)".to_string(),
            weak_areas: vec!["methylation".to_string(), "homocysteine".to_string()],
            gene_effects_counteracted: vec![
                "MTR / MTRR: methionine synthase and its reductase — methyl B12 is the cofactor.".to_string(),
                "MTHFR downstream: homocysteine stays high when remethylation is impaired — B12 supports the step.".to_string(),
            ],
            rationale: "Cofactor for methionine synthase; lowers homocysteine. Methyl form avoids conversion issues. Supports energy and nervous system.".to_string(),
            typical_note: "1000–5000 mcg oral or sublingual; dose is clinician-directed.".to_string(),
        },
        SupplementSupport {
            name: "B6 (P-5-P)".to_string(),
            weak_areas: vec!["homocysteine".to_string(), "sulfur metabolism".to_string()],
            gene_effects_counteracted: vec![
                "CBS / CTH: transsulfuration — P-5-P is cofactor; supports when flux is high or enzyme is stressed.".to_string(),
                "SUOX / sulfite: downstream sulfur handling — B6 supports the pathway into sulfite and sulfate.".to_string(),
            ],
            rationale: "Cofactor for CBS and other transsulfuration steps. Supports homocysteine clearance and sulfur metabolism. P-5-P is active form.".to_string(),
            typical_note: "Typical range 25–50 mg; avoid very high long-term. Clinician-directed.".to_string(),
        },
        SupplementSupport {
            name: "Quercetin".to_string(),
            weak_areas: vec!["MCAS / mast cell".to_string(), "inflammation".to_string()],
            gene_effects_counteracted: vec![
                "KIT / TPSAB1: mast cell activation or tryptase — stabilizes membrane and reduces degranulation.".to_string(),
                "DAO / HNMT: when histamine clearance is low — reduces histamine load from mast cells.".to_string(),
            ],
            rationale: "Mast cell stabilizer and antioxidant. May reduce reactivity and histamine load; often used alongside vitamin C in MCAS support.".to_string(),
            typical_note: "500–1000 mg; often with vitamin C. Not a substitute for prescription stabilizers.".to_string(),
        },
        SupplementSupport {
            name: "Vitamin C".to_string(),
            weak_areas: vec!["MCAS / antioxidant".to_string(), "immune".to_string()],
            gene_effects_counteracted: vec![
                "Mast cell / histamine: supports DAO and histamine breakdown; antioxidant stabilizes membranes.".to_string(),
                "General oxidative stress: when SOD/GPX or mitochondrial burden is high — scavenges ROS.".to_string(),
            ],
            rationale: "Antioxidant; supports mast cell membrane stability and histamine breakdown. Commonly low; supports immune and connective tissue.".to_string(),
            typical_note: "500–2000 mg divided; buffered or liposomal if sensitive. Not for clinical diagnosis.".to_string(),
        },
        SupplementSupport {
            name: "Omega-3 (EPA/DHA)".to_string(),
            weak_areas: vec!["inflammation".to_string(), "immune".to_string()],
            gene_effects_counteracted: vec![
                "Prostaglandin / inflammatory balance: shifts eicosanoid balance when COX/LOX or cytokine drive is high.".to_string(),
                "Mast cell / histamine context: anti-inflammatory; may lower reactivity and mediator burden.".to_string(),
            ],
            rationale: "Anti-inflammatory; supports cell membranes and inflammatory balance. Often insufficient in diet.".to_string(),
            typical_note: "1–2 g EPA+DHA daily from quality fish oil. Discuss with clinician.".to_string(),
        },
        SupplementSupport {
            name: "Vitamin D".to_string(),
            weak_areas: vec!["immune".to_string(), "mood / bone".to_string()],
            gene_effects_counteracted: vec![
                "VDR variants: when vitamin D receptor or metabolism is affected — adequate level compensates.".to_string(),
                "Immune dysregulation: low D worsens inflammation and fatigue — repletion supports regulation.".to_string(),
            ],
            rationale: "Frequently low; supports immune regulation and mood. Deficiency can worsen inflammation and fatigue.".to_string(),
            typical_note: "Dose by level (e.g. 1000–4000 IU); test and recheck. Clinician-directed.".to_string(),
        },
        SupplementSupport {
            name: "Magnesium".to_string(),
            weak_areas: vec!["nervous system".to_string(), "methylation cofactor".to_string()],
            gene_effects_counteracted: vec![
                "Methylation enzymes: Mg is cofactor for many; supports when MTHFR/MTR or stress pathway is loaded.".to_string(),
                "Nervous system / calcium: when calcium excitability or trigeminal sensitivity is high — Mg can buffer.".to_string(),
            ],
            rationale: "Cofactor for many enzymes; often low. Supports muscle, nerve, and stress response; relevant when methylation or CNS is a concern.".to_string(),
            typical_note: "Glycinate or citrate 200–400 mg; start low. Avoid in kidney disease without guidance.".to_string(),
        },
        SupplementSupport {
            name: "PEA (palmitoylethanolamide)".to_string(),
            weak_areas: vec!["MCAS / mast cell".to_string(), "neuropathic pain".to_string()],
            gene_effects_counteracted: vec![
                "Mast cell activation: supports mast cell stabilisation and reduces degranulation/burden in MCAS context.".to_string(),
                "Neuroinflammatory / pain signalling: may reduce neuropathic pain and inflammatory amplification.".to_string(),
            ],
            rationale: "Endocannabinoid-like compound; supports mast cell stabilisation and reduces inflammatory and neuropathic pain; often used in MCAS and chronic pain.".to_string(),
            typical_note: "Typical 300–600 mg twice daily; clinician-directed. Not a substitute for prescription stabilizers.".to_string(),
        },
        SupplementSupport {
            name: "Apigenin".to_string(),
            weak_areas: vec!["MCAS / mast cell".to_string(), "antioxidant".to_string()],
            gene_effects_counteracted: vec![
                "Mast cell activation: in vitro mast cell stabiliser; may modestly reduce degranulation when used with other flavonoids.".to_string(),
                "Oxidative / inflammatory stress: antioxidant support alongside quercetin/luteolin.".to_string(),
            ],
            rationale: "Flavonoid; mast cell stabiliser in vitro; anti-inflammatory; effect in MCAS is likely modest and best as an add-on with quercetin/luteolin.".to_string(),
            typical_note: "Supplement; typical doses vary (e.g. 25–50 mg apigenin or as part of combination formulas). Consider as optional add-on; clinician-directed.".to_string(),
        },
    ]
}
