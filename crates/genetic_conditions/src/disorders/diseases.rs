//! Disorders: psychiatric, autoimmune, neurological, metabolic (susceptibility genes).
//! Polygenic/complex; variants indicate susceptibility or association, not diagnosis. References: GWAS, GeneReviews, OMIM.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisorderRef {
    pub name: String,
    pub category: String,
    pub description: String,
    pub genes: Vec<String>,
    pub notes: Vec<String>,
    pub references: Vec<String>,
}

pub fn list_disorders() -> Vec<DisorderRef> {
    vec![
        // Psychiatric
        DisorderRef {
            name: "Major depressive disorder (susceptibility)".to_string(),
            category: "Psychiatric".to_string(),
            description: "Polygenic; mood, anhedonia, sleep, appetite. Serotonin and stress-pathway genes.".to_string(),
            genes: vec!["SLC6A4".to_string(), "MTHFR".to_string(), "COMT".to_string(), "BDNF".to_string(), "CACNA1C".to_string(), "HTR2A".to_string()],
            notes: vec!["Susceptibility only; environment and epigenetics matter. SLC6A4 (5-HTT) length variant; MTHFR/folate pathway.".to_string()],
            references: vec!["GWAS; GeneReviews: Depression".to_string()],
        },
        DisorderRef {
            name: "Anxiety disorders (susceptibility)".to_string(),
            category: "Psychiatric".to_string(),
            description: "Polygenic; GAD, panic, phobia. Serotonin and catecholamine pathways.".to_string(),
            genes: vec!["SLC6A4".to_string(), "COMT".to_string(), "GAD1".to_string(), "BDNF".to_string()],
            notes: vec!["Association only; not diagnostic.".to_string()],
            references: vec!["GWAS; psychiatric genetics".to_string()],
        },
        DisorderRef {
            name: "Bipolar disorder (susceptibility)".to_string(),
            category: "Psychiatric".to_string(),
            description: "Polygenic; mood episodes, mania/hypomania. Calcium and neurotransmitter pathways.".to_string(),
            genes: vec!["CACNA1C".to_string(), "ANK3".to_string(), "ODZ4".to_string(), "SYNE1".to_string(), "MTHFR".to_string()],
            notes: vec!["Susceptibility; lithium response may vary by genotype.".to_string()],
            references: vec!["GWAS; PGC; GeneReviews".to_string()],
        },
        DisorderRef {
            name: "Schizophrenia (susceptibility)".to_string(),
            category: "Psychiatric".to_string(),
            description: "Polygenic; psychosis, cognition. HLA, glutamate, dopamine.".to_string(),
            genes: vec!["DISC1".to_string(), "NRGN".to_string(), "DRD2".to_string(), "GRIN2A".to_string(), "CACNA1C".to_string(), "HLA-DRB1".to_string()],
            notes: vec!["Many loci; HLA and neurotransmitter genes.".to_string()],
            references: vec!["GWAS; PGC; GeneReviews".to_string()],
        },
        DisorderRef {
            name: "ADHD (susceptibility)".to_string(),
            category: "Psychiatric".to_string(),
            description: "Polygenic; inattention, hyperactivity, impulsivity. Dopamine and adhesion.".to_string(),
            genes: vec!["DRD4".to_string(), "SLC6A3".to_string(), "ADGRL3".to_string(), "SNAP25".to_string(), "LPHN3".to_string()],
            notes: vec!["Association only; DRD4 length variant often cited.".to_string()],
            references: vec!["GWAS; psychiatric genetics".to_string()],
        },
        // Autoimmune
        DisorderRef {
            name: "Systemic lupus erythematosus (SLE) (susceptibility)".to_string(),
            category: "Autoimmune".to_string(),
            description: "Autoimmune; rash, joint, kidney, fatigue. HLA and interferon pathway.".to_string(),
            genes: vec!["HLA-DRB1".to_string(), "IRF5".to_string(), "STAT4".to_string(), "BLK".to_string(), "ITGAM".to_string(), "TNFAIP3".to_string()],
            notes: vec!["Strong HLA association; IRF5/STAT4 type I IFN.".to_string()],
            references: vec!["GWAS; GeneReviews: SLE".to_string()],
        },
        DisorderRef {
            name: "Rheumatoid arthritis (susceptibility)".to_string(),
            category: "Autoimmune".to_string(),
            description: "Autoimmune; symmetric arthritis, joint damage. HLA and PTPN22.".to_string(),
            genes: vec!["HLA-DRB1".to_string(), "PTPN22".to_string(), "STAT4".to_string(), "TRAF1".to_string(), "CTLA4".to_string()],
            notes: vec!["Shared epitope (HLA-DRB1); PTPN22 risk variant.".to_string()],
            references: vec!["GWAS; GeneReviews".to_string()],
        },
        DisorderRef {
            name: "Inflammatory bowel disease / Crohn (susceptibility)".to_string(),
            category: "Autoimmune".to_string(),
            description: "Chronic gut inflammation. NOD2, autophagy, IL-23.".to_string(),
            genes: vec!["NOD2".to_string(), "IL23R".to_string(), "ATG16L1".to_string(), "IRGM".to_string(), "STAT3".to_string()],
            notes: vec!["NOD2 frameshift; IL23R protective variant.".to_string()],
            references: vec!["GWAS; GeneReviews: IBD".to_string()],
        },
        DisorderRef {
            name: "Multiple sclerosis (susceptibility)".to_string(),
            category: "Autoimmune".to_string(),
            description: "CNS demyelination; HLA and immune genes.".to_string(),
            genes: vec!["HLA-DRB1".to_string(), "IL2RA".to_string(), "CLEC16A".to_string(), "IL7R".to_string(), "CD58".to_string()],
            notes: vec!["HLA-DRB1*15:01 strong risk.".to_string()],
            references: vec!["GWAS; GeneReviews: MS".to_string()],
        },
        // Neurological
        DisorderRef {
            name: "Migraine (susceptibility)".to_string(),
            category: "Neurological".to_string(),
            description: "Recurrent headache; vascular and neuronal. MTHFR and ion channels.".to_string(),
            genes: vec!["MTHFR".to_string(), "PRDM16".to_string(), "TRPM8".to_string(), "LRP1".to_string(), "CACNA1A".to_string()],
            notes: vec!["MTHFR C677T association; CACNA1A in familial hemiplegic migraine.".to_string()],
            references: vec!["GWAS; GeneReviews: Migraine".to_string()],
        },
        DisorderRef {
            name: "Epilepsy (susceptibility)".to_string(),
            category: "Neurological".to_string(),
            description: "Seizure susceptibility; ion channels and GABA. Many genes; some monogenic subtypes.".to_string(),
            genes: vec!["SCN1A".to_string(), "SCN2A".to_string(), "GABRA1".to_string(), "KCNQ2".to_string(), "LGI1".to_string(), "DEPDC5".to_string()],
            notes: vec!["Susceptibility; SCN1A/2A in Dravet and related; not diagnostic alone.".to_string()],
            references: vec!["GeneReviews: Epilepsy overview".to_string()],
        },
        // Metabolic
        DisorderRef {
            name: "Type 2 diabetes (susceptibility)".to_string(),
            category: "Metabolic".to_string(),
            description: "Insulin resistance and beta-cell function. TCF7L2, PPARG, KCNJ11.".to_string(),
            genes: vec!["TCF7L2".to_string(), "PPARG".to_string(), "KCNJ11".to_string(), "SLC30A8".to_string(), "IGF2BP2".to_string(), "CDKAL1".to_string()],
            notes: vec!["Strong TCF7L2 association; lifestyle major factor.".to_string()],
            references: vec!["GWAS; GeneReviews: T2D".to_string()],
        },
        DisorderRef {
            name: "Obesity (susceptibility)".to_string(),
            category: "Metabolic".to_string(),
            description: "BMI and adiposity; FTO and melanocortin pathway.".to_string(),
            genes: vec!["FTO".to_string(), "MC4R".to_string(), "TMEM18".to_string(), "BDNF".to_string(), "SH2B1".to_string()],
            notes: vec!["FTO robust GWAS hit; MC4R in severe early-onset.".to_string()],
            references: vec!["GWAS; GeneReviews".to_string()],
        },
    ]
}
