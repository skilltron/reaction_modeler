//! Superior Limbic Keratoconjunctivitis (SLK) / Theodore's syndrome. References: Theodore (1963), ocular surface literature.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlkRef {
    pub name: String,
    pub description: String,
    pub key_terms: Vec<String>,
    pub associations: Vec<String>,
    pub genes: Vec<String>,
    pub references: Vec<String>,
}

pub fn slk_theodores_ref() -> SlkRef {
    SlkRef {
        name: "Superior limbic keratoconjunctivitis (SLK) / Theodore's syndrome".to_string(),
        description: "Chronic ocular surface disease: inflammation of superior bulbar and tarsal conjunctiva, with corneal involvement. Symptoms: burning, foreign-body sensation, redness, filamentary keratitis.".to_string(),
        key_terms: vec![
            "superior limbic keratoconjunctivitis".to_string(), "SLK".to_string(), "Theodore's syndrome".to_string(),
            "superior bulbar conjunctiva".to_string(), "superior tarsal conjunctiva".to_string(), "filamentary keratitis".to_string(), "ocular surface disease".to_string(),
        ],
        associations: vec!["thyroid disease".to_string(), "contact lens use".to_string(), "mechanical laxity".to_string(), "dry eye".to_string()],
        genes: vec![],
        references: vec!["Theodore (1963); SLK classification.".to_string(), "Ocular surface inflammation and SLK (literature).".to_string()],
    }
}
