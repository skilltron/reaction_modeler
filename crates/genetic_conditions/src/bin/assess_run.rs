//! Run the full genetic-conditions pipeline on one or two variant JSON files and compare reports.
//! Uses all modules (conditions, cascade, survival) to assess the run and compare VCF/MDNG report outputs.
//!
//! Usage: assess-run <variants.json> [variants_b.json]
//!   One path: print full assessment for that run.
//!   Two paths: print assessments for both and a comparison (which shows more data, more findings, higher scores).

use genetic_conditions::{
    cascade,
    check_variants_against_all,
    gene_annotation,
    survival,
};
use std::env;
use std::fs;
use std::process;

fn count_findings(report: &genetic_conditions::AllConditionsReport) -> (usize, usize, usize, usize, usize, usize, usize) {
    let immune: usize = report.immune.iter().map(|r| r.findings.len()).sum();
    let inflammation: usize = report.inflammation.iter().map(|r| r.findings.len()).sum();
    let exposure: usize = report.exposure.iter().map(|r| r.findings.len()).sum();
    let sulfur: usize = report.sulfur.iter().map(|r| r.findings.len()).sum();
    let rare: usize = report.rare.iter().map(|r| r.findings.len()).sum();
    let cancer: usize = report.cancer.iter().map(|r| r.findings.len()).sum();
    let disorders: usize = report.disorders.iter().map(|r| r.findings.len()).sum();
    (immune, inflammation, exposure, sulfur, rare, cancer, disorders)
}

struct RunAssessment {
    label: String,
    variant_count: usize,
    immune: usize,
    inflammation: usize,
    exposure: usize,
    sulfur: usize,
    rare: usize,
    cancer: usize,
    disorders: usize,
    total_findings: usize,
    cascade_composite: u8,
    survival_genes: Vec<String>,
}

fn assess(path: &str, label: &str) -> Result<RunAssessment, Box<dyn std::error::Error>> {
    let raw = fs::read_to_string(path)?;
    let variants = gene_annotation::annotate_variants_with_genes(serde_json::from_str(&raw)?);
    let report = check_variants_against_all(&variants);
    let (immune, inflammation, exposure, sulfur, rare, cancer, disorders) = count_findings(&report);
    let total_findings = immune + inflammation + exposure + sulfur + rare + cancer + disorders;
    let cascade_report = cascade::compute_cascade_from_report(&report);
    let survival_analysis = survival::analyze_survival(&variants);

    Ok(RunAssessment {
        label: label.to_string(),
        variant_count: variants.len(),
        immune,
        inflammation,
        exposure,
        sulfur,
        rare,
        cancer,
        disorders,
        total_findings,
        cascade_composite: cascade_report.scores.composite_cgrp_runaway_cascade,
        survival_genes: survival_analysis.genes_with_severe_phenotype,
    })
}

fn print_assessment(a: &RunAssessment) {
    println!("--- {} ---", a.label);
    println!("  Variants:           {}", a.variant_count);
    println!("  Findings:");
    println!("    immune:           {}", a.immune);
    println!("    inflammation:     {}", a.inflammation);
    println!("    exposure:         {}", a.exposure);
    println!("    sulfur:           {}", a.sulfur);
    println!("    rare:             {}", a.rare);
    println!("    cancer:           {}", a.cancer);
    println!("    disorders:        {}", a.disorders);
    println!("  Total findings:     {}", a.total_findings);
    println!("  Cascade composite:  {}", a.cascade_composite);
    println!(
        "  Survival genes:    {}",
        if a.survival_genes.is_empty() {
            "(none)".to_string()
        } else {
            a.survival_genes.join(", ")
        }
    );
    println!();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        eprintln!("Usage: assess-run <variants.json> [variants_b.json]");
        eprintln!("  One path:  full assessment (conditions, cascade, survival).");
        eprintln!("  Two paths: assess both and compare (e.g. MDNG vs VCF report).");
        process::exit(1);
    }

    let path_a = &args[1];
    let label_a = path_a
        .rsplit('/')
        .next()
        .unwrap_or(path_a)
        .replace(".json", "");
    let run_a = assess(path_a, &label_a)?;
    print_assessment(&run_a);

    if args.len() == 3 {
        let path_b = &args[2];
        let label_b = path_b
            .rsplit('/')
            .next()
            .unwrap_or(path_b)
            .replace(".json", "");
        let run_b = assess(path_b, &label_b)?;
        print_assessment(&run_b);

        println!("========== COMPARISON (A = {}, B = {}) ==========", label_a, label_b);
        println!("  Variants:        {} vs {}  -> {}", run_a.variant_count, run_b.variant_count, cmp(run_a.variant_count, run_b.variant_count, "A", "B"));
        println!("  Total findings:  {} vs {}  -> {}", run_a.total_findings, run_b.total_findings, cmp(run_a.total_findings, run_b.total_findings, "A", "B"));
        println!("  Immune:          {} vs {}  -> {}", run_a.immune, run_b.immune, cmp(run_a.immune, run_b.immune, "A", "B"));
        println!("  Inflammation:    {} vs {}  -> {}", run_a.inflammation, run_b.inflammation, cmp(run_a.inflammation, run_b.inflammation, "A", "B"));
        println!("  Exposure:        {} vs {}  -> {}", run_a.exposure, run_b.exposure, cmp(run_a.exposure, run_b.exposure, "A", "B"));
        println!("  Sulfur:          {} vs {}  -> {}", run_a.sulfur, run_b.sulfur, cmp(run_a.sulfur, run_b.sulfur, "A", "B"));
        println!("  Rare:            {} vs {}  -> {}", run_a.rare, run_b.rare, cmp(run_a.rare, run_b.rare, "A", "B"));
        println!("  Disorders:       {} vs {}  -> {}", run_a.disorders, run_b.disorders, cmp(run_a.disorders, run_b.disorders, "A", "B"));
        println!("  Cascade score:   {} vs {}  -> {}", run_a.cascade_composite, run_b.cascade_composite, cmp(run_a.cascade_composite as usize, run_b.cascade_composite as usize, "A", "B"));
        println!("  Survival genes:  {} vs {}  -> {}", run_a.survival_genes.len(), run_b.survival_genes.len(), cmp(run_a.survival_genes.len(), run_b.survival_genes.len(), "A", "B"));
        println!();
        println!("Assessment: Run with more variants and/or more condition findings is the richer input for the report.");
    }
    Ok(())
}

fn cmp(na: usize, nb: usize, label_a: &str, label_b: &str) -> String {
    if na > nb {
        format!("{} has more", label_a)
    } else if nb > na {
        format!("{} has more", label_b)
    } else {
        "tie".to_string()
    }
}

