//! Rare genetic diseases: Fragile X, myotonic dystrophy, Prader–Willi/Angelman, Rett, CHARGE, Noonan, Bardet–Biedl, ciliopathies.

mod diseases;
mod report;

pub use diseases::{list_rare_genetic_diseases, RareDiseaseRef};
pub use report::{
    check_variants_against_rare_diseases, RareDiseaseFinding, RareDiseaseReport,
};
