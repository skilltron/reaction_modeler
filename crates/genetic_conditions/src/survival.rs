//! Why severe or "lethal" genotypes may show mild or survivable phenotypes.
//! For research and educational use only; not for clinical diagnosis.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// One possible reason why a severe/lethal genotype might not show the classic severe phenotype.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalReason {
    pub reason: String,
    pub explanation: String,
}

/// Gene-associated severe phenotype and reasons for survival / mild outcome.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SevereGeneRef {
    pub gene: String,
    pub condition: String,
    pub typical_severity: String,
    pub reasons_for_mild_or_survival: Vec<SurvivalReason>,
}

/// Known genes with severe/lethal classic phenotypes and explanations for milder outcomes.
/// Osteogenesis imperfecta (OI) has many associated genes; we list the main ones so variants in any can be flagged and explained.
pub fn severe_phenotype_reference() -> Vec<SevereGeneRef> {
    vec![
        // Osteogenesis imperfecta (OI) — type I–XVII; many genes
        SevereGeneRef {
            gene: "COL1A1".to_string(),
            condition: "Osteogenesis imperfecta (OI)".to_string(),
            typical_severity: "Bone fragility; type I mild to type II lethal; gene constantly expressed.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Incomplete penetrance / mild allele".to_string(),
                    explanation: "Some variants cause haploinsufficiency with mild type I OI; only a few fractures over decades (e.g. finger, metatarsal, partial long-bone fracture) despite constantly expressed gene.".to_string(),
                },
                SurvivalReason {
                    reason: "Modifier genes or collagen chaperones".to_string(),
                    explanation: "Other genes affecting collagen folding, secretion, or bone remodelling can attenuate phenotype.".to_string(),
                },
                SurvivalReason {
                    reason: "Minimal trauma / protective behaviour".to_string(),
                    explanation: "Fewer high-impact injuries reduce fracture count; phenotype may still be 'mild OI'.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "COL1A2".to_string(),
            condition: "Osteogenesis imperfecta (OI)".to_string(),
            typical_severity: "Bone fragility; type I–IV; gene constantly expressed.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Mild allele or incomplete penetrance".to_string(),
                    explanation: "Null or missense variants with residual function can give type I–like phenotype; e.g. only a few fractures (finger, metatarsal, partial leg) by mid-life.".to_string(),
                },
                SurvivalReason {
                    reason: "Modifier genes".to_string(),
                    explanation: "Genetic background can soften bone phenotype.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "CRTAP".to_string(),
            condition: "Osteogenesis imperfecta (OI)".to_string(),
            typical_severity: "Type VII OI; severe to lethal; collagen prolyl 3-hydroxylation.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Residual activity or hypomorphic allele".to_string(),
                    explanation: "Some variants allow partial 3-hydroxylation; phenotype can be moderate.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "LEPRE1".to_string(),
            condition: "Osteogenesis imperfecta (OI)".to_string(),
            typical_severity: "Type VIII OI (P3H1); often severe.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Hypomorphic allele".to_string(),
                    explanation: "Residual P3H1 function can attenuate severity.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "FKBP10".to_string(),
            condition: "Osteogenesis imperfecta (OI)".to_string(),
            typical_severity: "Type XI OI; bone fragility, often with Bruck syndrome overlap.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Mild allele".to_string(),
                    explanation: "Some FKBP10 variants are associated with less severe bone phenotype.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "SERPINH1".to_string(),
            condition: "Osteogenesis imperfecta (OI)".to_string(),
            typical_severity: "Type X OI; HSP47 collagen chaperone; can be severe.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Residual chaperone function".to_string(),
                    explanation: "Hypomorphic variants may allow some collagen folding.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "SP7".to_string(),
            condition: "Osteogenesis imperfecta (OI)".to_string(),
            typical_severity: "Type XII OI (osteoblast transcription factor); variable.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Variable expressivity".to_string(),
                    explanation: "SP7-related OI shows a range of severity.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "BMP1".to_string(),
            condition: "Osteogenesis imperfecta (OI)".to_string(),
            typical_severity: "Type XIII OI; procollagen C-proteinase; variable.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Residual processing".to_string(),
                    explanation: "Some BMP1 variants permit partial collagen processing.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "TMEM38B".to_string(),
            condition: "Osteogenesis imperfecta (OI)".to_string(),
            typical_severity: "Type XIV OI; ER calcium channel; variable.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Variable expressivity".to_string(),
                    explanation: "Phenotype ranges from mild to moderate.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "IFITM5".to_string(),
            condition: "Osteogenesis imperfecta (OI)".to_string(),
            typical_severity: "Type V OI; hyperplastic callus; dominant.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Type V often non-lethal".to_string(),
                    explanation: "Type V OI is typically compatible with survival; severity varies.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "MECP2".to_string(),
            condition: "Rett syndrome".to_string(),
            typical_severity: "Severe neurodevelopmental; often early lethality in males.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Mosaicism".to_string(),
                    explanation: "Somatic mosaicism can spare enough cells to allow survival or milder phenotype.".to_string(),
                },
                SurvivalReason {
                    reason: "Mild or late-onset variant".to_string(),
                    explanation: "Some alleles cause milder Rett or variant phenotypes.".to_string(),
                },
            ],
        },
        SevereGeneRef {
            gene: "CHD7".to_string(),
            condition: "CHARGE syndrome".to_string(),
            typical_severity: "Multisystem; can be life-limiting.".to_string(),
            reasons_for_mild_or_survival: vec![
                SurvivalReason {
                    reason: "Haploinsufficiency with variable expressivity".to_string(),
                    explanation: "Phenotype ranges from severe to mild; some individuals have few major malformations.".to_string(),
                },
                SurvivalReason {
                    reason: "Mosaicism".to_string(),
                    explanation: "Somatic mosaicism can attenuate severity.".to_string(),
                },
            ],
        },
    ]
}

/// Result of survival analysis for a set of variants: which severe genes were seen and possible reasons.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurvivalAnalysis {
    pub genes_with_severe_phenotype: Vec<String>,
    pub by_gene: HashMap<String, SevereGeneRef>,
    pub summary: String,
}

/// Analyze variants for genes associated with severe/lethal phenotypes and attach reasons for survival/mild outcome.
pub fn analyze_survival(variants: &[crate::VariantInput]) -> SurvivalAnalysis {
    let refs = severe_phenotype_reference();
    let severe_genes: std::collections::HashSet<String> = refs
        .iter()
        .flat_map(|r| std::iter::once(r.gene.to_uppercase()))
        .collect();

    let mut genes_found = Vec::new();
    let mut by_gene = HashMap::new();
    for v in variants {
        if let Some(ref g) = v.gene {
            let gu = g.to_uppercase();
            if severe_genes.contains(&gu) && !genes_found.contains(&gu) {
                genes_found.push(gu.clone());
                if let Some(r) = refs.iter().find(|r| r.gene.to_uppercase() == gu) {
                    by_gene.insert(gu.clone(), r.clone());
                }
            }
        }
    }

    let summary = if genes_found.is_empty() {
        "No variants in reference severe-phenotype genes in this set.".to_string()
    } else {
        format!(
            "Variants found in genes associated with severe phenotypes: {}. See by-gene reasons for possible explanations of mild or survivable outcome.",
            genes_found.join(", ")
        )
    };

    SurvivalAnalysis {
        genes_with_severe_phenotype: genes_found,
        by_gene,
        summary,
    }
}
