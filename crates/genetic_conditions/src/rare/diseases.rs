//! Rare genetic diseases: Fragile X, myotonic dystrophy, Prader–Willi/Angelman, Rett, CHARGE, Noonan, Bardet–Biedl, ciliopathies. References: OMIM, GeneReviews.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RareDiseaseRef {
    pub name: String,
    pub description: String,
    pub genes: Vec<String>,
    pub notes: Vec<String>,
    pub references: Vec<String>,
}

pub fn list_rare_genetic_diseases() -> Vec<RareDiseaseRef> {
    vec![
        RareDiseaseRef {
            name: "Fragile X syndrome".to_string(),
            description: "X-linked; intellectual disability, behavioral, physical features. CGG repeat expansion in FMR1; premutation carriers at risk for FXTAS.".to_string(),
            genes: vec!["FMR1".to_string()],
            notes: vec!["CGG repeat in 5' UTR: full mutation (>200) causes syndrome; premutation (55–200) carrier.".to_string(), "Repeat expansion not called from standard SNV/indel VCF; FMR1 gene region still relevant.".to_string()],
            references: vec!["OMIM 300624".to_string(), "GeneReviews: FMR1 Disorders".to_string()],
        },
        RareDiseaseRef {
            name: "Myotonic dystrophy type 1 (DM1)".to_string(),
            description: "Autosomal dominant; myotonia, weakness, cardiac, cataract, insulin resistance. CTG repeat in DMPK.".to_string(),
            genes: vec!["DMPK".to_string()],
            notes: vec!["CTG repeat in 3' UTR; expansion (typically >50) causes disease.".to_string()],
            references: vec!["OMIM 160900".to_string(), "GeneReviews: Myotonic Dystrophy Type 1".to_string()],
        },
        RareDiseaseRef {
            name: "Myotonic dystrophy type 2 (DM2)".to_string(),
            description: "Autosomal dominant; proximal weakness, myotonia, less severe than DM1. CCTG repeat in CNBP.".to_string(),
            genes: vec!["CNBP".to_string()],
            notes: vec!["CCTG repeat in intron 1 of CNBP (ZNF9).".to_string()],
            references: vec!["OMIM 602668".to_string(), "GeneReviews: Myotonic Dystrophy Type 2".to_string()],
        },
        RareDiseaseRef {
            name: "Prader–Willi syndrome".to_string(),
            description: "Imprinting; hypotonia, feeding difficulty, then hyperphagia, obesity, intellectual disability. Paternal 15q11–q13 deletion or maternal UPD or imprinting defect.".to_string(),
            genes: vec!["SNRPN".to_string(), "NDN".to_string(), "MAGEL2".to_string(), "MKRN3".to_string()],
            notes: vec!["15q11.2–q13; deletion (paternal), maternal UPD, or imprinting defect. SNRPN key P-W critical region.".to_string()],
            references: vec!["OMIM 176270".to_string(), "GeneReviews: Prader-Willi Syndrome".to_string()],
        },
        RareDiseaseRef {
            name: "Angelman syndrome".to_string(),
            description: "Imprinting; severe intellectual disability, ataxia, happy demeanor, epilepsy. Maternal 15q11–q13 deletion, UBE3A mutation, or paternal UPD.".to_string(),
            genes: vec!["UBE3A".to_string(), "GABRB3".to_string(), "ATP10A".to_string()],
            notes: vec!["15q11.2–q13; maternal deletion or UBE3A mutation; paternal UPD. UBE3A primary Angelman gene.".to_string()],
            references: vec!["OMIM 105830".to_string(), "GeneReviews: Angelman Syndrome".to_string()],
        },
        RareDiseaseRef {
            name: "Rett syndrome".to_string(),
            description: "X-linked dominant; mostly females. Regression, loss of hand use, seizures, breathing dysregulation. MECP2 mutations.".to_string(),
            genes: vec!["MECP2".to_string()],
            notes: vec!["Most cases de novo; males severe. CDKL5 and FOXG1 in variant Rett phenotypes.".to_string()],
            references: vec!["OMIM 312750".to_string(), "GeneReviews: Rett Syndrome".to_string()],
        },
        RareDiseaseRef {
            name: "CHARGE syndrome".to_string(),
            description: "Coloboma, heart defect, atresia choanae, retardation (developmental), genital, ear. CHD7 haploinsufficiency.".to_string(),
            genes: vec!["CHD7".to_string()],
            notes: vec!["Majority CHD7; autosomal dominant; many de novo.".to_string()],
            references: vec!["OMIM 214800".to_string(), "GeneReviews: CHARGE Syndrome".to_string()],
        },
        RareDiseaseRef {
            name: "Noonan syndrome".to_string(),
            description: "RASopathy; characteristic facies, short stature, heart defect (e.g. PV stenosis), bleeding tendency. PTPN11 most common; multiple RAS-MAPK genes.".to_string(),
            genes: vec!["PTPN11".to_string(), "SOS1".to_string(), "RAF1".to_string(), "KRAS".to_string(), "NRAS".to_string(), "BRAF".to_string(), "MAP2K1".to_string(), "LZTR1".to_string(), "RIT1".to_string(), "RRAS".to_string(), "SOS2".to_string(), "RASA2".to_string(), "MAP3K8".to_string()],
            notes: vec!["RAS-MAPK pathway; genotype–phenotype (e.g. PTPN11 and leukemia risk).".to_string()],
            references: vec!["OMIM 163950".to_string(), "GeneReviews: Noonan Syndrome".to_string()],
        },
        RareDiseaseRef {
            name: "Bardet–Biedl syndrome".to_string(),
            description: "Ciliopathy; rod-cone dystrophy, polydactyly, obesity, renal, learning disability, hypogonadism. Multiple BBS genes; often oligogenic.".to_string(),
            genes: vec!["BBS1".to_string(), "BBS2".to_string(), "BBS4".to_string(), "BBS5".to_string(), "BBS7".to_string(), "BBS9".to_string(), "BBS10".to_string(), "BBS12".to_string(), "ARL6".to_string(), "MKKS".to_string(), "MKS1".to_string(), "CEP290".to_string(), "WDPCP".to_string(), "IFT27".to_string(), "LZTFL1".to_string(), "BBIP1".to_string(), "IFT74".to_string(), "C8ORF37".to_string()],
            notes: vec!["BBS1/BBS10 common; triallelic inheritance in some families.".to_string()],
            references: vec!["OMIM 209900".to_string(), "GeneReviews: Bardet-Biedl Syndrome".to_string()],
        },
        RareDiseaseRef {
            name: "Joubert syndrome and related disorders".to_string(),
            description: "Ciliopathy; molar tooth sign, hypotonia, ataxia, developmental delay, variable renal/retinal. Many genes.".to_string(),
            genes: vec!["AHI1".to_string(), "CEP290".to_string(), "TMEM67".to_string(), "NPHP1".to_string(), "RPGRIP1L".to_string(), "CC2D2A".to_string(), "ARL13B".to_string(), "INPP5E".to_string(), "OFD1".to_string(), "TCTN1".to_string(), "TCTN2".to_string(), "TCTN3".to_string(), "KIF7".to_string(), "CSPP1".to_string(), "MKS1".to_string(), "B9D1".to_string(), "B9D2".to_string(), "TMEM216".to_string(), "C5ORF42".to_string(), "TMEM138".to_string(), "TMEM231".to_string(), "KIAA0586".to_string(), "CEP104".to_string(), "C2CD3".to_string()],
            notes: vec!["Molar tooth sign on MRI; >35 genes; genetic heterogeneity.".to_string()],
            references: vec!["OMIM 213300".to_string(), "GeneReviews: Joubert Syndrome".to_string()],
        },
        RareDiseaseRef {
            name: "Nephronophthisis".to_string(),
            description: "Ciliopathy; juvenile form leading cause of ESRD in children. Cysts, fibrosis; overlap with Joubert/Senior–Løken.".to_string(),
            genes: vec!["NPHP1".to_string(), "INVS".to_string(), "NPHP3".to_string(), "NPHP4".to_string(), "IQCB1".to_string(), "CEP290".to_string(), "GLIS2".to_string(), "RPGRIP1L".to_string(), "NEK8".to_string(), "SDCCAG8".to_string(), "TMEM67".to_string(), "TTC21B".to_string(), "WDR19".to_string(), "CEP164".to_string(), "ZNF423".to_string(), "ANKS6".to_string(), "MAPKBP1".to_string(), "DCDC2".to_string(), "MKS1".to_string(), "IFT172".to_string(), "CEP83".to_string(), "CSPP1".to_string()],
            notes: vec!["NPHP1 deletion common; recessive; retinal dystrophy in some (Senior–Løken).".to_string()],
            references: vec!["OMIM 256100".to_string(), "GeneReviews: Nephronophthisis".to_string()],
        },
    ]
}
