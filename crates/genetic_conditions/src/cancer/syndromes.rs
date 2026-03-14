//! Hereditary cancer syndromes: genes and brief descriptions for genetic-only screening.
//! References: GeneReviews, NCCN, ACMG. For research/educational use only; not clinical diagnosis.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CancerSyndromeRef {
    pub name: String,
    pub description: String,
    pub genes: Vec<String>,
    pub screening_notes: Vec<String>,
    pub references: Vec<String>,
}

/// Hereditary cancer syndromes commonly included in genetic cancer screening panels.
pub fn list_hereditary_cancer_syndromes() -> Vec<CancerSyndromeRef> {
    vec![
        CancerSyndromeRef {
            name: "Hereditary breast and ovarian cancer (HBOC)".to_string(),
            description: "Increased risk for breast, ovarian, and other cancers. BRCA1 and BRCA2; other genes (PALB2, ATM, CHEK2, etc.) on extended panels.".to_string(),
            genes: vec![
                "BRCA1".to_string(),
                "BRCA2".to_string(),
                "PALB2".to_string(),
                "ATM".to_string(),
                "CHEK2".to_string(),
                "BARD1".to_string(),
                "RAD51C".to_string(),
                "RAD51D".to_string(),
            ],
            screening_notes: vec![
                "Guidelines: NCCN, USPSTF for high-risk; confirm with genetic counseling and accredited testing.".to_string(),
                "Pathogenic variants drive risk; VUS require follow-up.".to_string(),
            ],
            references: vec![
                "GeneReviews: BRCA1/BRCA2 Hereditary Breast and Ovarian Cancer".to_string(),
                "NCCN Guidelines for Genetic/Familial High-Risk Assessment: Breast, Ovarian, and Pancreatic.".to_string(),
            ],
        },
        CancerSyndromeRef {
            name: "Lynch syndrome (hereditary nonpolyposis colorectal cancer)".to_string(),
            description: "Increased risk for colorectal, endometrial, and other cancers. Mismatch repair genes.".to_string(),
            genes: vec![
                "MLH1".to_string(),
                "MSH2".to_string(),
                "MSH6".to_string(),
                "PMS2".to_string(),
                "EPCAM".to_string(),
            ],
            screening_notes: vec![
                "MMR deficiency; tumor MSI/dMMR or IHC can support. Confirm with genetic counseling.".to_string(),
            ],
            references: vec![
                "GeneReviews: Lynch Syndrome".to_string(),
                "NCCN Guidelines: Genetic/Familial High-Risk Assessment: Colorectal.".to_string(),
            ],
        },
        CancerSyndromeRef {
            name: "Li–Fraumeni syndrome".to_string(),
            description: "TP53; broad cancer spectrum (sarcomas, breast, brain, adrenocortical, leukemia).".to_string(),
            genes: vec!["TP53".to_string()],
            screening_notes: vec![
                "Highly penetrant; surveillance per NCCN. Confirm with accredited testing.".to_string(),
            ],
            references: vec!["GeneReviews: Li-Fraumeni Syndrome".to_string(), "NCCN TP53 Guidelines.".to_string()],
        },
        CancerSyndromeRef {
            name: "PTEN hamartoma tumor syndrome (Cowden syndrome)".to_string(),
            description: "PTEN; increased risk for breast, thyroid, endometrial, colorectal; benign hamartomas.".to_string(),
            genes: vec!["PTEN".to_string()],
            screening_notes: vec!["Clinical criteria (Cowden) plus molecular. Confirm with genetic counseling.".to_string()],
            references: vec!["GeneReviews: PTEN Hamartoma Tumor Syndrome".to_string()],
        },
        CancerSyndromeRef {
            name: "Familial adenomatous polyposis (FAP)".to_string(),
            description: "APC; hundreds to thousands of colorectal adenomas; high risk for colorectal cancer if untreated.".to_string(),
            genes: vec!["APC".to_string()],
            screening_notes: vec!["Attenuated FAP with lesser polyp burden. Confirm with genetic counseling.".to_string()],
            references: vec!["GeneReviews: Familial Adenomatous Polyposis".to_string()],
        },
        CancerSyndromeRef {
            name: "Hereditary diffuse gastric cancer".to_string(),
            description: "CDH1; diffuse gastric cancer and lobular breast cancer risk.".to_string(),
            genes: vec!["CDH1".to_string()],
            screening_notes: vec!["Prophylactic gastrectomy considered in carriers. Confirm with genetic counseling.".to_string()],
            references: vec!["GeneReviews: Hereditary Diffuse Gastric Cancer".to_string()],
        },
        CancerSyndromeRef {
            name: "Von Hippel–Lindau syndrome".to_string(),
            description: "VHL; hemangioblastomas, renal cell carcinoma, pheochromocytoma, pancreatic lesions.".to_string(),
            genes: vec!["VHL".to_string()],
            screening_notes: vec!["Surveillance per VHL guidelines. Confirm with genetic counseling.".to_string()],
            references: vec!["GeneReviews: Von Hippel-Lindau Syndrome".to_string()],
        },
        CancerSyndromeRef {
            name: "Multiple endocrine neoplasia type 2 (MEN2)".to_string(),
            description: "RET; medullary thyroid carcinoma, pheochromocytoma, parathyroid. MEN2A and MEN2B.".to_string(),
            genes: vec!["RET".to_string()],
            screening_notes: vec!["RET genotype correlates with phenotype; prophylactic thyroidectomy in some. Confirm with genetic counseling.".to_string()],
            references: vec!["GeneReviews: Multiple Endocrine Neoplasia Type 2".to_string()],
        },
    ]
}
