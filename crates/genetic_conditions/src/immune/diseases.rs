//! Immune diseases with well-established genetic cause or susceptibility.
//! References: OMIM, GeneReviews where applicable.

use serde::{Deserialize, Serialize};

/// How well a genetic cause or susceptibility is established.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceLevel {
    Monogenic,
    StrongSusceptibility,
    PolygenicSusceptibility,
}

/// Reference entry for one immune disease.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImmuneDiseaseRef {
    pub name: String,
    pub genetic_cause_indicated: bool,
    pub evidence_level: EvidenceLevel,
    pub description: String,
    pub genes: Vec<String>,
    pub notes: Vec<String>,
    pub references: Vec<String>,
}

pub fn list_immune_diseases() -> Vec<ImmuneDiseaseRef> {
    vec![
        ImmuneDiseaseRef {
            name: "X-linked severe combined immunodeficiency (SCID)".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::Monogenic,
            description: "X-linked SCID; T cell absence, B cell dysfunction.".to_string(),
            genes: vec!["IL2RG".to_string()],
            notes: vec!["IL2RG mutations; X-linked.".to_string()],
            references: vec!["OMIM 300400".to_string(), "GeneReviews: X-linked SCID".to_string()],
        },
        ImmuneDiseaseRef {
            name: "X-linked agammaglobulinemia (XLA)".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::Monogenic,
            description: "Absent B cells and immunoglobulins.".to_string(),
            genes: vec!["BTK".to_string()],
            notes: vec!["BTK mutations; X-linked.".to_string()],
            references: vec!["OMIM 300755".to_string(), "GeneReviews: X-linked agammaglobulinemia".to_string()],
        },
        ImmuneDiseaseRef {
            name: "ADA-deficient SCID".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::Monogenic,
            description: "Adenosine deaminase deficiency; T/B lymphopenia.".to_string(),
            genes: vec!["ADA".to_string()],
            notes: vec!["Autosomal recessive.".to_string()],
            references: vec!["OMIM 102700".to_string(), "GeneReviews: ADA deficiency".to_string()],
        },
        ImmuneDiseaseRef {
            name: "Familial Mediterranean fever (FMF)".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::Monogenic,
            description: "Autoinflammatory; recurrent fever, serositis.".to_string(),
            genes: vec!["MEFV".to_string()],
            notes: vec!["MEFV mutations; autosomal recessive (incomplete penetrance).".to_string()],
            references: vec!["OMIM 249100".to_string(), "GeneReviews: FMF".to_string()],
        },
        ImmuneDiseaseRef {
            name: "Cryopyrin-associated periodic syndromes (CAPS)".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::Monogenic,
            description: "Autoinflammatory; NLRP3 gain-of-function.".to_string(),
            genes: vec!["NLRP3".to_string()],
            notes: vec!["NLRP3 mutations; autosomal dominant.".to_string()],
            references: vec!["OMIM 606416".to_string(), "GeneReviews: CAPS".to_string()],
        },
        ImmuneDiseaseRef {
            name: "IPEX syndrome".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::Monogenic,
            description: "Immune dysregulation, polyendocrinopathy, enteropathy, X-linked.".to_string(),
            genes: vec!["FOXP3".to_string()],
            notes: vec!["FOXP3 mutations; X-linked.".to_string()],
            references: vec!["OMIM 304790".to_string(), "GeneReviews: IPEX".to_string()],
        },
        ImmuneDiseaseRef {
            name: "APECED (Autoimmune polyendocrinopathy-candidiasis-ectodermal dystrophy)".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::Monogenic,
            description: "AIRE deficiency; autoimmune multi-organ.".to_string(),
            genes: vec!["AIRE".to_string()],
            notes: vec!["AIRE mutations; autosomal recessive.".to_string()],
            references: vec!["OMIM 240300".to_string(), "GeneReviews: APECED".to_string()],
        },
        ImmuneDiseaseRef {
            name: "Celiac disease".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::StrongSusceptibility,
            description: "Gluten-sensitive enteropathy; HLA-DQ2/DQ8 strongly associated.".to_string(),
            genes: vec!["HLA-DQA1".to_string(), "HLA-DQB1".to_string()],
            notes: vec![
                "HLA-DQ2.5, DQ8; necessary but not sufficient.".to_string(),
                "Non-HLA genes (e.g. IL2/IL21 region) contribute.".to_string(),
            ],
            references: vec![
                "HLA-DQ in celiac (major susceptibility)".to_string(),
                "GeneReviews: Celiac disease".to_string(),
            ],
        },
        ImmuneDiseaseRef {
            name: "Ankylosing spondylitis".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::StrongSusceptibility,
            description: "Spondyloarthropathy; HLA-B27 strongly associated.".to_string(),
            genes: vec!["HLA-B".to_string()],
            notes: vec!["HLA-B*27 alleles; major risk factor.".to_string()],
            references: vec!["OMIM 106300".to_string(), "HLA-B27 and AS (Nat Rev Rheumatol)".to_string()],
        },
        ImmuneDiseaseRef {
            name: "Type 1 diabetes".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::PolygenicSusceptibility,
            description: "Autoimmune diabetes; HLA class II is major susceptibility.".to_string(),
            genes: vec![
                "HLA-DRB1".to_string(),
                "HLA-DQB1".to_string(),
                "HLA-DQA1".to_string(),
                "INS".to_string(),
                "PTPN22".to_string(),
            ],
            notes: vec!["HLA accounts for ~50% familial risk; INS, PTPN22, others.".to_string()],
            references: vec!["OMIM 222100".to_string(), "T1DM genetics (HLA, INS, PTPN22)".to_string()],
        },
        ImmuneDiseaseRef {
            name: "Systemic lupus erythematosus (SLE)".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::PolygenicSusceptibility,
            description: "Autoimmune; complement deficiencies (monogenic) and HLA/polygenic risk.".to_string(),
            genes: vec![
                "HLA-DRB1".to_string(),
                "C1QA".to_string(),
                "C1QB".to_string(),
                "C1QC".to_string(),
                "C2".to_string(),
                "C4A".to_string(),
                "C4B".to_string(),
                "TREX1".to_string(),
            ],
            notes: vec!["Rare monogenic: C1q/C2/C4, TREX1. Common: HLA, IRF5, STAT4.".to_string()],
            references: vec!["OMIM 152700".to_string(), "GeneReviews: SLE".to_string()],
        },
        ImmuneDiseaseRef {
            name: "Crohn disease (IBD)".to_string(),
            genetic_cause_indicated: true,
            evidence_level: EvidenceLevel::PolygenicSusceptibility,
            description: "Inflammatory bowel disease; NOD2 and other susceptibility genes.".to_string(),
            genes: vec!["NOD2".to_string(), "ATG16L1".to_string(), "IL23R".to_string()],
            notes: vec!["NOD2 variants increase risk; multiple loci.".to_string()],
            references: vec!["OMIM 266600".to_string(), "IBD genetics (NOD2, ATG16L1, IL23R)".to_string()],
        },
    ]
}
