//! Pharmacopoeia: drug–enzyme interactions and recommendations from star allele phenotypes.
//! For research and educational use only; not for clinical prescribing.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::star_alleles::StarAlleleGeneResult;

/// Role of an enzyme in a drug's metabolism.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnzymeRole {
    Primary,
    Secondary,
    Minor,
}

/// One drug's metabolism: which enzymes and their role.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugMetabolism {
    pub drug_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drug_class: Option<String>,
    /// (gene symbol, role)
    pub enzymes: Vec<(String, EnzymeRole)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Per-enzyme note for one drug given the user's phenotype.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugEnzymeNote {
    pub drug_name: String,
    pub gene: String,
    pub role: EnzymeRole,
    pub user_effect: String,
    pub recommendation: String,
}

/// Full interaction summary for one drug.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrugInteractionReport {
    pub drug_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drug_class: Option<String>,
    pub enzyme_notes: Vec<DrugEnzymeNote>,
    pub summary: String,
}

fn role_str(r: EnzymeRole) -> &'static str {
    match r {
        EnzymeRole::Primary => "primary",
        EnzymeRole::Secondary => "secondary",
        EnzymeRole::Minor => "minor",
    }
}

/// Curated list of drugs with CYP metabolism (for interaction check).
pub fn curated_drugs() -> Vec<DrugMetabolism> {
    vec![
        DrugMetabolism {
            drug_name: "Venlafaxine".to_string(),
            drug_class: Some("SNRI".to_string()),
            enzymes: vec![
                ("CYP2D6".to_string(), EnzymeRole::Primary),
                ("CYP2C19".to_string(), EnzymeRole::Secondary),
                ("CYP3A4".to_string(), EnzymeRole::Minor),
            ],
            notes: Some("O-demethylation to ODV (CYP2D6); N-demethylation (CYP2C19).".to_string()),
        },
        DrugMetabolism {
            drug_name: "Clopidogrel".to_string(),
            drug_class: Some("Antiplatelet".to_string()),
            enzymes: vec![
                ("CYP2C19".to_string(), EnzymeRole::Primary),
                ("CYP3A4".to_string(), EnzymeRole::Secondary),
            ],
            notes: Some("CYP2C19 activates prodrug; loss of function reduces efficacy.".to_string()),
        },
        DrugMetabolism {
            drug_name: "Warfarin".to_string(),
            drug_class: Some("Anticoagulant".to_string()),
            enzymes: vec![
                ("CYP2C9".to_string(), EnzymeRole::Primary),
                ("CYP3A4".to_string(), EnzymeRole::Minor),
            ],
            notes: Some("CYP2C9 *2/*3: reduced clearance; lower dose often required.".to_string()),
        },
        DrugMetabolism {
            drug_name: "Omeprazole".to_string(),
            drug_class: Some("PPI".to_string()),
            enzymes: vec![
                ("CYP2C19".to_string(), EnzymeRole::Primary),
                ("CYP3A4".to_string(), EnzymeRole::Secondary),
            ],
            notes: None,
        },
        DrugMetabolism {
            drug_name: "Citalopram".to_string(),
            drug_class: Some("SSRI".to_string()),
            enzymes: vec![
                ("CYP2C19".to_string(), EnzymeRole::Primary),
                ("CYP2D6".to_string(), EnzymeRole::Secondary),
            ],
            notes: None,
        },
        DrugMetabolism {
            drug_name: "Escitalopram".to_string(),
            drug_class: Some("SSRI".to_string()),
            enzymes: vec![
                ("CYP2C19".to_string(), EnzymeRole::Primary),
                ("CYP2D6".to_string(), EnzymeRole::Secondary),
            ],
            notes: None,
        },
        DrugMetabolism {
            drug_name: "Sertraline".to_string(),
            drug_class: Some("SSRI".to_string()),
            enzymes: vec![
                ("CYP2B6".to_string(), EnzymeRole::Minor),
                ("CYP2C19".to_string(), EnzymeRole::Primary),
                ("CYP2D6".to_string(), EnzymeRole::Minor),
                ("CYP3A4".to_string(), EnzymeRole::Secondary),
            ],
            notes: None,
        },
        DrugMetabolism {
            drug_name: "Amitriptyline".to_string(),
            drug_class: Some("TCA".to_string()),
            enzymes: vec![
                ("CYP2C19".to_string(), EnzymeRole::Primary),
                ("CYP2D6".to_string(), EnzymeRole::Primary),
            ],
            notes: Some("Dual CYP2C19/CYP2D6 metabolism; phenotype affects levels.".to_string()),
        },
        DrugMetabolism {
            drug_name: "Codeine".to_string(),
            drug_class: Some("Opioid".to_string()),
            enzymes: vec![("CYP2D6".to_string(), EnzymeRole::Primary)],
            notes: Some("Prodrug; CYP2D6 converts to morphine. Poor metabolizers get less effect.".to_string()),
        },
        DrugMetabolism {
            drug_name: "Tramadol".to_string(),
            drug_class: Some("Opioid".to_string()),
            enzymes: vec![
                ("CYP2D6".to_string(), EnzymeRole::Primary),
                ("CYP3A4".to_string(), EnzymeRole::Secondary),
            ],
            notes: None,
        },
        DrugMetabolism {
            drug_name: "Diazepam".to_string(),
            drug_class: Some("Benzodiazepine".to_string()),
            enzymes: vec![
                ("CYP2C19".to_string(), EnzymeRole::Primary),
                ("CYP3A4".to_string(), EnzymeRole::Secondary),
            ],
            notes: None,
        },
        DrugMetabolism {
            drug_name: "Pantoprazole".to_string(),
            drug_class: Some("PPI".to_string()),
            enzymes: vec![("CYP2C19".to_string(), EnzymeRole::Primary)],
            notes: None,
        },
        DrugMetabolism {
            drug_name: "Ketotifen".to_string(),
            drug_class: Some("Mast cell stabilizer / H1".to_string()),
            enzymes: vec![
                ("CYP3A4".to_string(), EnzymeRole::Primary),
                ("CYP2D6".to_string(), EnzymeRole::Minor),
            ],
            notes: Some("Mast cell stabilizer; CYP3A4 main clearance.".to_string()),
        },
        DrugMetabolism {
            drug_name: "Cromolyn sodium".to_string(),
            drug_class: Some("Mast cell stabilizer".to_string()),
            enzymes: vec![],
            notes: Some("Minimal hepatic metabolism; largely excreted unchanged.".to_string()),
        },
        // AML and other chemotherapy (CYP phenotypes from Star alleles; DPYD/TPMT: ask oncology team for testing)
        DrugMetabolism {
            drug_name: "Daunorubicin".to_string(),
            drug_class: Some("Anthracycline (AML/leukemia chemo)".to_string()),
            enzymes: vec![
                ("CYP3A4".to_string(), EnzymeRole::Primary),
                ("CYP2D6".to_string(), EnzymeRole::Minor),
            ],
            notes: Some("Used in AML induction. CYP3A4 affects clearance; discuss with oncology team.".to_string()),
        },
        DrugMetabolism {
            drug_name: "Doxorubicin".to_string(),
            drug_class: Some("Anthracycline (chemo)".to_string()),
            enzymes: vec![
                ("CYP3A4".to_string(), EnzymeRole::Primary),
                ("CYP2D6".to_string(), EnzymeRole::Minor),
            ],
            notes: None,
        },
        DrugMetabolism {
            drug_name: "Idarubicin".to_string(),
            drug_class: Some("Anthracycline (AML chemo)".to_string()),
            enzymes: vec![("CYP3A4".to_string(), EnzymeRole::Primary)],
            notes: Some("AML induction. CYP3A4 reduced function may increase exposure.".to_string()),
        },
        DrugMetabolism {
            drug_name: "Etoposide".to_string(),
            drug_class: Some("Chemotherapy (AML/lymphoma)".to_string()),
            enzymes: vec![("CYP3A4".to_string(), EnzymeRole::Primary)],
            notes: Some("CYP3A4 loss/reduced function can raise etoposide levels; discuss dose with oncology team.".to_string()),
        },
        DrugMetabolism {
            drug_name: "Cytarabine".to_string(),
            drug_class: Some("AML chemotherapy".to_string()),
            enzymes: vec![],
            notes: Some("Main AML backbone. Metabolized by cytidine deaminase; no major CYP. DPYD is for fluoropyrimidines (5-FU), not cytarabine. Discuss any concerns with oncology team.".to_string()),
        },
        DrugMetabolism {
            drug_name: "6-Mercaptopurine".to_string(),
            drug_class: Some("Thiopurine (ALL/AML maintenance)".to_string()),
            enzymes: vec![("TPMT".to_string(), EnzymeRole::Primary)],
            notes: Some("TPMT not in this report’s star alleles. Ask oncology team for TPMT/DPYD testing before thiopurines or fluoropyrimidines.".to_string()),
        },
        DrugMetabolism {
            drug_name: "Fluorouracil (5-FU) / Capecitabine".to_string(),
            drug_class: Some("Fluoropyrimidine (chemo)".to_string()),
            enzymes: vec![("DPYD".to_string(), EnzymeRole::Primary)],
            notes: Some("DPYD not in this report’s star alleles. DPD deficiency increases severe toxicity risk. Ask oncology team for DPYD testing before fluoropyrimidines.".to_string()),
        },
    ]
}

/// Build gene -> effect summary from star allele results (e.g. "loss of function", "reduced function", "reference").
fn phenotype_by_gene(star_results: &[StarAlleleGeneResult]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for r in star_results {
        let effect = if r.effect_labels.is_empty() {
            "reference".to_string()
        } else {
            r.effect_labels.join("; ")
        };
        map.insert(r.gene.clone(), effect);
    }
    map
}

/// For one drug, produce enzyme-level notes and a short summary given the user's star allele results.
pub fn check_drug_against_phenotypes(
    drug: &DrugMetabolism,
    star_results: &[StarAlleleGeneResult],
) -> DrugInteractionReport {
    let phenotype = phenotype_by_gene(star_results);
    let mut enzyme_notes = Vec::new();

    let primary_cyp2d6_reference = drug.drug_name == "Venlafaxine"
        && drug.enzymes.iter().any(|(g, r)| *r == EnzymeRole::Primary && g == "CYP2D6")
        && phenotype.get("CYP2D6").map(|e| e.contains("reference")).unwrap_or(true);

    for (gene, role) in &drug.enzymes {
        let effect = phenotype
            .get(gene)
            .map(String::as_str)
            .unwrap_or("not genotyped");
        let recommendation = recommendation_for_effect(
            effect,
            *role,
            drug.drug_name.as_str(),
            gene,
            primary_cyp2d6_reference,
        );
        enzyme_notes.push(DrugEnzymeNote {
            drug_name: drug.drug_name.clone(),
            gene: gene.clone(),
            role: *role,
            user_effect: effect.to_string(),
            recommendation,
        });
    }

    let summary = if enzyme_notes.is_empty() {
        format!(
            "{}: {}",
            drug.drug_name,
            drug.notes.as_deref().unwrap_or("No CYP phenotype check for this drug.")
        )
    } else {
        let alerts: Vec<String> = enzyme_notes
            .iter()
            .filter(|n| {
                !n.user_effect.contains("reference") && n.user_effect != "not genotyped"
            })
            .map(|n| format!("{} ({})", n.gene, n.user_effect))
            .collect();
        let only_increased = alerts.iter().all(|a| a.contains("increased function"));
        let primary_is_reference: bool = drug.enzymes.iter()
            .filter(|(_, r)| *r == EnzymeRole::Primary)
            .any(|(gene, _)| {
                phenotype.get(gene).map(|e| e.contains("reference")).unwrap_or(true)
            });
        let primary_is_cyp2d6 = drug.enzymes.iter()
            .any(|(g, r)| *r == EnzymeRole::Primary && g == "CYP2D6");

        if alerts.is_empty() {
            if primary_is_cyp2d6 && primary_is_reference {
                format!(
                    "{}: CYP2D6 (primary enzyme) not confidently genotyped in this set — WGS often misses or mis-calls CYP2D6. Consider pharmacogenomic testing for dose guidance.",
                    drug.drug_name
                )
            } else if primary_is_reference {
                format!(
                    "{}: No reduced/loss-of-function phenotypes detected for relevant enzymes in this set. Primary enzyme(s) reported as reference; consider PGx testing if clinically indicated.",
                    drug.drug_name
                )
            } else {
                format!(
                    "{}: No reduced/loss-of-function phenotypes detected for relevant enzymes.",
                    drug.drug_name
                )
            }
        } else if only_increased {
            format!(
                "{}: {} — increased function (enzyme more active; opposite of loss of function). Implications differ per drug; see details below. Discuss with prescriber.",
                drug.drug_name,
                alerts.join(", ")
            )
        } else {
            format!(
                "{}: Check {} — consider dose adjustment or alternative; discuss with prescriber.",
                drug.drug_name,
                alerts.join(", ")
            )
        }
    };

    DrugInteractionReport {
        drug_name: drug.drug_name.clone(),
        drug_class: drug.drug_class.clone(),
        enzyme_notes,
        summary,
    }
}

fn recommendation_for_effect(
    effect: &str,
    role: EnzymeRole,
    drug_name: &str,
    gene: &str,
    venlafaxine_cyp2d6_uncertain: bool,
) -> String {
    let is_increased = effect.contains("increased function");
    let is_loss_or_reduced = effect.contains("loss of function") || effect.contains("reduced function");
    let is_concerning = is_loss_or_reduced || is_increased;

    if effect == "not genotyped" {
        return format!("{} not assessed in this set; confirm with PGx testing if relevant.", gene);
    }
    if !is_concerning {
        let base = format!(
            "{}: {} ({}); no dose adjustment typically needed for this enzyme.",
            gene, effect, role_str(role)
        );
        if venlafaxine_cyp2d6_uncertain && (role == EnzymeRole::Secondary || role == EnzymeRole::Minor) {
            return format!("{} Main uncertainty for venlafaxine is CYP2D6 (see summary).", base);
        }
        return base;
    }

    // Increased function = enzyme more active (opposite of loss of function). Drug implications differ.
    if is_increased {
        match role {
            EnzymeRole::Primary => format!(
                "{} {}: increased function (enzyme more active — not the same as loss of function). For some drugs this means faster metabolism or prodrug activation; discuss with prescriber.",
                gene, role_str(role)
            ),
            EnzymeRole::Secondary => format!(
                "{} {}: increased function — enzyme more active; may contribute to lower drug levels. Discuss with prescriber if concerned.",
                gene, role_str(role)
            ),
            EnzymeRole::Minor => format!(
                "{} {}: increased function — minor contribution for {}.",
                gene, role_str(role), drug_name
            ),
        }
    } else {
        // Loss or reduced function
        match role {
            EnzymeRole::Primary => format!(
                "{} {}: {} — consider dose adjustment or alternative; discuss with prescriber.",
                gene, role_str(role), effect
            ),
            EnzymeRole::Secondary => format!(
                "{} {}: {} — may contribute to exposure; discuss with prescriber if concerned.",
                gene, role_str(role), effect
            ),
            EnzymeRole::Minor => format!(
                "{} {}: {} — minor contribution for {}.",
                gene, role_str(role), effect, drug_name
            ),
        }
    }
}

/// Run pharmacopoeia check for all curated drugs against the user's star allele results.
pub fn run_pharmacopoeia_check(
    star_results: &[StarAlleleGeneResult],
) -> Vec<DrugInteractionReport> {
    curated_drugs()
        .into_iter()
        .map(|drug| check_drug_against_phenotypes(&drug, star_results))
        .collect()
}
