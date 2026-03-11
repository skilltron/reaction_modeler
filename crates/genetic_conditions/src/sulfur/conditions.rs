//! Sulfur metabolism disorders. References: OMIM, GeneReviews.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SulfurMetabolismRef {
    pub name: String,
    pub description: String,
    pub genes: Vec<String>,
    pub notes: Vec<String>,
    pub references: Vec<String>,
}

pub fn list_sulfur_metabolism_conditions() -> Vec<SulfurMetabolismRef> {
    vec![
        SulfurMetabolismRef {
            name: "Homocystinuria (CBS deficiency)".to_string(),
            description: "Cystathionine beta-synthase deficiency; elevated homocysteine, thrombosis, lens dislocation, osteoporosis. Pyridoxine-responsive in some.".to_string(),
            genes: vec!["CBS".to_string()],
            notes: vec!["Transsulfuration: homocysteine to cystathionine. B6 responsive in ~50%.".to_string(), "Biomarker: plasma/serum homocysteine; urine homocysteine.".to_string()],
            references: vec!["OMIM 236200".to_string(), "GeneReviews: Homocystinuria due to CBS deficiency".to_string()],
        },
        SulfurMetabolismRef {
            name: "Homocystinuria (remethylation defects)".to_string(),
            description: "Defects in homocysteine remethylation; elevated homocysteine, megaloblastic anemia, neurologic. MTHFR, MTR, MTRR.".to_string(),
            genes: vec!["MTHFR".to_string(), "MTR".to_string(), "MTRR".to_string()],
            notes: vec!["MTHFR 677C>T common variant: mild homocysteine elevation; severe MTHFR deficiency rare.".to_string(), "MTR/MTRR: cobalamin (B12) dependent.".to_string()],
            references: vec!["OMIM 236250".to_string(), "OMIM 250940 (MTHFR)".to_string(), "GeneReviews: Homocystinuria".to_string()],
        },
        SulfurMetabolismRef {
            name: "Sulfite oxidase deficiency".to_string(),
            description: "Isolated SUOX deficiency; sulfite cannot be converted to sulfate. Severe neuro deterioration, seizures, dislocated lenses. Sulfites accumulate (diet, endogenous).".to_string(),
            genes: vec!["SUOX".to_string()],
            notes: vec!["Sulfite (SO3) → sulfate (SO4). Avoid sulfite-containing foods (wine, dried fruit, some drugs).".to_string(), "Urine sulfite positive; low sulfate. Often neonatal/early infant onset.".to_string()],
            references: vec!["OMIM 272300".to_string(), "GeneReviews: Isolated sulfite oxidase deficiency".to_string()],
        },
        SulfurMetabolismRef {
            name: "Molybdenum cofactor deficiency".to_string(),
            description: "Defect in molybdenum cofactor biosynthesis; same biochemical block as isolated sulfite oxidase deficiency. Severe neuro, seizures, lens dislocation.".to_string(),
            genes: vec!["MOCS1".to_string(), "MOCS2".to_string(), "GPHN".to_string()],
            notes: vec!["MOCS1/MOCS2: molybdenum cofactor synthesis. GPHN: gephyrin.".to_string(), "Phenotype overlaps sulfite oxidase deficiency; genetic testing distinguishes.".to_string()],
            references: vec!["OMIM 252150".to_string(), "GeneReviews: Molybdenum cofactor deficiency".to_string()],
        },
        SulfurMetabolismRef {
            name: "Cystathioninuria".to_string(),
            description: "CTH or CBS-related; elevated cystathionine in urine. Often benign; some neurologic reports.".to_string(),
            genes: vec!["CTH".to_string(), "CBS".to_string()],
            notes: vec!["Cystathionine breakdown. Rare; clinical significance variable.".to_string()],
            references: vec!["OMIM 219500".to_string()],
        },
    ]
}
