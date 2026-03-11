//! Chemical effect patterns: genes and pathways. References: PMC, PubMed, GeneReviews where applicable.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChemicalName {
    Trichloroethylene,
    Radon,
    Benzene,
    Arsenic,
    IonizingRadiation,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChemicalEffectPattern {
    pub name: String,
    pub description: String,
    pub genes: Vec<String>,
    pub pathways: Vec<String>,
    pub references: Vec<String>,
}

pub fn chemical_effect_pattern(chemical: ChemicalName) -> ChemicalEffectPattern {
    match chemical {
        ChemicalName::Trichloroethylene => trichloroethylene_pattern(),
        ChemicalName::Radon => radon_pattern(),
        ChemicalName::Benzene => benzene_pattern(),
        ChemicalName::Arsenic => arsenic_pattern(),
        ChemicalName::IonizingRadiation => ionizing_radiation_pattern(),
        ChemicalName::Other => ChemicalEffectPattern {
            name: "Other".to_string(),
            description: "No pattern defined.".to_string(),
            genes: vec![],
            pathways: vec![],
            references: vec![],
        },
    }
}

fn trichloroethylene_pattern() -> ChemicalEffectPattern {
    ChemicalEffectPattern {
        name: "Trichloroethylene (TCE)".to_string(),
        description: "Solvent; oxidative stress, DNA methylation changes, cell cycle, apoptosis, PPAR. Genetic susceptibility via metabolism genes.".to_string(),
        genes: vec![
            "TP53".to_string(), "CDKN1A".to_string(), "BAX".to_string(), "BCL2".to_string(), "PPARA".to_string(),
            "DNMT1".to_string(), "DNMT3A".to_string(), "DNMT3B".to_string(), "TET2".to_string(), "UHRF1".to_string(),
            "IHH".to_string(), "PPARG".to_string(), "GSTT1".to_string(), "GSTM1".to_string(), "CYP2E1".to_string(),
        ],
        pathways: vec!["PPAR signaling".to_string(), "Cell cycle regulation".to_string(), "Apoptosis".to_string(), "DNA methylation / epigenetic regulation".to_string(), "Reductive metabolism".to_string()],
        references: vec![
            "PMC2630226 Genetic Signature for Human Risk Assessment: TCE".to_string(),
            "TCE and blood DNA methylation (autoimmune/cancer pathways)".to_string(),
            "TCE-induced gene expression and DNA methylation in mouse liver (PLOS One)".to_string(),
            "PubMed 28962410 Occupational TCE and p53, p21, bax, bcl-2, PPARA".to_string(),
            "PMC2922418 TCE renal carcinoma and reductive metabolism gene variants".to_string(),
        ],
    }
}

fn radon_pattern() -> ChemicalEffectPattern {
    ChemicalEffectPattern {
        name: "Radon".to_string(),
        description: "Radioactive gas; alpha particles, double-strand breaks, oxidative DNA damage. Susceptibility via DNA repair.".to_string(),
        genes: vec![
            "XRCC1".to_string(), "OGG1".to_string(), "APEX1".to_string(), "ERCC1".to_string(), "ERCC2".to_string(),
            "RAD51".to_string(), "XRCC2".to_string(), "XRCC3".to_string(), "BRCA1".to_string(), "BRCA2".to_string(),
            "ATM".to_string(), "TP53".to_string(), "MPZL2".to_string(), "POMC".to_string(), "STAU2".to_string(), "MLNR".to_string(),
        ],
        pathways: vec!["BER".to_string(), "NER".to_string(), "HR/DSB repair".to_string(), "DNA damage response".to_string()],
        references: vec![
            "PMC6375683 Genetic modifiers of radon-induced lung cancer risk (uranium miners)".to_string(),
            "Residential radon, DNA repair polymorphisms (Lung Cancer journal)".to_string(),
            "PubMed 30008631 Radon exposure-induced genetic variations in lung cancers".to_string(),
            "Rare deleterious germline variants and lung cancer risk (ATM, MPZL2, POMC)".to_string(),
        ],
    }
}

fn benzene_pattern() -> ChemicalEffectPattern {
    ChemicalEffectPattern {
        name: "Benzene".to_string(),
        description: "Volatile solvent; hematotoxicity, leukemia risk. Susceptibility via phase I/II metabolism.".to_string(),
        genes: vec!["CYP2E1".to_string(), "GSTM1".to_string(), "GSTT1".to_string(), "NQO1".to_string(), "MPO".to_string()],
        pathways: vec!["Phase I/II metabolism".to_string(), "Oxidative stress".to_string()],
        references: vec![
            "PMC1241108 CYP2E1, MPO, NQO1, GSTM1, GSTT1 and benzene poisoning (EHP)".to_string(),
            "PubMed 34295994 Genetic polymorphisms in benzene-exposed workers".to_string(),
        ],
    }
}

fn arsenic_pattern() -> ChemicalEffectPattern {
    ChemicalEffectPattern {
        name: "Arsenic (inorganic)".to_string(),
        description: "Metalloid; oxidative stress, methylation. Susceptibility via AS3MT, GSTO, MTHFR, DNA repair.".to_string(),
        genes: vec!["AS3MT".to_string(), "GSTO1".to_string(), "GSTO2".to_string(), "MTHFR".to_string(), "ERCC2".to_string(), "XRCC1".to_string(), "NFE2L2".to_string(), "DNMT1".to_string()],
        pathways: vec!["Arsenic metabolism".to_string(), "Glutathione/redox".to_string(), "DNA repair".to_string(), "DNA methylation".to_string()],
        references: vec![
            "PMC2898853 Genetic effects on toxic elements".to_string(),
            "Arsenic metabolism and genetic biomarkers (Sci Direct)".to_string(),
            "PMC10082670 Metals and DNA methylation".to_string(),
        ],
    }
}

fn ionizing_radiation_pattern() -> ChemicalEffectPattern {
    ChemicalEffectPattern {
        name: "Ionizing radiation".to_string(),
        description: "X-ray, gamma, particles; DSB, ROS. Susceptibility via BER, NER, HR, NHEJ.".to_string(),
        genes: vec![
            "XRCC1".to_string(), "OGG1".to_string(), "APEX1".to_string(), "ERCC1".to_string(), "ERCC2".to_string(),
            "RAD51".to_string(), "XRCC2".to_string(), "XRCC3".to_string(), "BRCA1".to_string(), "BRCA2".to_string(),
            "ATM".to_string(), "TP53".to_string(), "NBN".to_string(), "MRE11A".to_string(), "RAD50".to_string(),
            "LIG4".to_string(), "XRCC4".to_string(), "PRKDC".to_string(),
        ],
        pathways: vec!["BER".to_string(), "NER".to_string(), "HR".to_string(), "NHEJ".to_string(), "DNA damage response".to_string()],
        references: vec![
            "Genes affecting ionizing radiation survival (exome + functional screening)".to_string(),
            "Human variation in DNA repair and cancer risk (Front Immunol)".to_string(),
        ],
    }
}
