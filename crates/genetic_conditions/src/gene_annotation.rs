//! Annotate VCF-derived variants with gene symbols from rsID and from genomic position so condition checks (cancer, inflammation, etc.) can match.
//! Without this, variants from vcf-to-variants or mdng-to-variants have gene: None and never match.
//! Position-based annotation helps when rsID is missing or not in our map, and reduces sensitivity to reference build (GRCh37 vs GRCh38).

use crate::variant_input::VariantInput;
use crate::sequencing_parity;
use std::collections::HashMap;

/// Build the rsID → gene lookup once (caller can cache if desired).
fn rsid_gene_lookup() -> HashMap<String, String> {
    sequencing_parity::rsid_to_gene_map()
}

/// Normalize chromosome for comparison: "17", "chr17" -> 17; "13", "chr13" -> 13.
fn chrom_number(chrom: &str) -> Option<u32> {
    let s = chrom.trim_start_matches("chr");
    s.parse::<u32>().ok()
}

/// BRCA1 (chr17): span that covers both GRCh37 (41,196,312–41,277,500) and GRCh38 (43,044,295–43,125,364).
const BRCA1_CHR: u32 = 17;
const BRCA1_START: u64 = 41_100_000;
const BRCA1_END: u64 = 43_200_000;

/// BRCA2 (chr13): span that covers GRCh37/GRCh38 chr13 ~32.2–32.5 M.
const BRCA2_CHR: u32 = 13;
const BRCA2_START: u64 = 32_200_000;
const BRCA2_END: u64 = 32_500_000;

/// If variant has no gene but falls in a known cancer-gene span, set gene. Handles chr vs no-chr and both ref builds.
fn gene_from_position(chrom: &str, position: u64) -> Option<&'static str> {
    let ch = chrom_number(chrom)?;
    if ch == BRCA1_CHR && position >= BRCA1_START && position <= BRCA1_END {
        return Some("BRCA1");
    }
    if ch == BRCA2_CHR && position >= BRCA2_START && position <= BRCA2_END {
        return Some("BRCA2");
    }
    None
}

/// Annotate variants with gene when missing: (1) from rsID→gene map; (2) from genomic position (BRCA1/BRCA2) so alignment/build differences don’t miss basics.
/// Returns a new Vec; variants that already have gene are unchanged.
pub fn annotate_variants_with_genes(variants: Vec<VariantInput>) -> Vec<VariantInput> {
    let map = rsid_gene_lookup();
    variants
        .into_iter()
        .map(|mut v| {
            if v.gene.is_none() {
                if let Some(ref rsid) = v.rsid {
                    let key = sequencing_parity::canonical_rsid(rsid);
                    if !key.is_empty() {
                        if let Some(gene) = map.get(&key) {
                            v.gene = Some(gene.clone());
                        }
                    }
                }
                if v.gene.is_none() {
                    if let Some(gene) = gene_from_position(&v.chromosome, v.position) {
                        v.gene = Some(gene.to_string());
                    }
                }
            }
            v
        })
        .collect()
}
