# External tools to compare our results against

Besides Sequencing.com, you can compare pipeline and report output against these open-source tools. Run them on the same VCF (or same sample) and cross-check.

---

## 1. PharmCAT (pharmacogenomics – star alleles & drug recommendations)

**What it does:** Takes a VCF, infers pharmacogene star alleles (CPIC/PharmVar style), and produces diplotype calls plus drug recommendations (CPIC, DPWG, FDA labels).

**Good for:** Comparing our **Star alleles** section (CYP2C19, CYP2D6, CYP2C9). PharmCAT covers more genes and is the standard for PGx reporting.

- **Site:** https://pharmcat.org/  
- **Docs / run:** https://pharmcat.clinpgx.org/using/Running-PharmCAT/  
- **GitHub:** https://github.com/PharmGKB/PharmCAT (MPL-2.0)

**Typical use:**
- Download the PharmCAT JAR (or use Docker).
- Run the **VCF Preprocessor** on your VCF, then the **Named Allele Matcher** (and optionally Phenotyper + Reporter).
- Compare PharmCAT’s diplotype output (e.g. CYP2C19 *1/*2) with our report’s **Star alleles** tab and with StellarPGx if you run it.

**Note:** CYP2D6 often needs copy-number/structural-variant calling; PharmCAT documents optional “outside call” files for genes they don’t call from VCF.

---

**Verification in our report:** You can feed the official finder's star allele calls into the genetic conditions report so it shows a **Verification vs official star allele finder** table (agreement or mismatch per gene). Export PharmCAT's (or StellarPGx's) diplotypes into a JSON file in this format:

```json
{
  "CYP2C19": { "diplotype": "*1/*2", "source": "PharmCAT" },
  "CYP2D6":  { "diplotype": "*1/*1", "source": "PharmCAT" },
  "CYP2C9":  { "diplotype": "*1/*1", "source": "PharmCAT" },
  "CYP3A4":  { "diplotype": "*1/*1", "source": "PharmCAT" }
}
```

Then set `OFFICIAL_STAR_ALLELE_JSON` to the path to that file when running the report (e.g. in your batch script or env). The report will compare our inferred diplotypes to the official finder and show ✓ or "Consider external verification" per gene.

---

## 2. StellarPGx (star alleles from sequencing)

**What it does:** Nextflow pipeline for calling star alleles in CYP genes from **BAM/CRAM** (not VCF-only). Uses genome graphs and coverage; very strong concordance for CYP2D6.

**Good for:** Cross-checking **star allele** calls when you have alignment files. Our report already suggests “Cross-check with StellarPGx or pipeline star allele output.”

- **GitHub:** https://github.com/SBIMB/StellarPGx (MIT)  
- **Input:** CRAM/BAM + reference (WGS). Not a simple VCF-in tool.

Use StellarPGx when you have BAM/CRAM; use PharmCAT when you only have VCF.

---

## 3. bcftools (VCF stats and overlap)

**What it does:** Standard CLI for VCF/BCF: variant counts, types (SNP/indel), per-sample counts, and overlap between files.

**Good for:** Sanity checks: “Does our converted variant set have similar counts to the lab VCF?” and “How many variants are unique to each pipeline?”

- **Install:** `brew install bcftools` (macOS) or from samtools/bcftools.
- **Variant summary:**  
  `bcftools stats your.vcf.gz`  
  (or `bcftools stats -s - file.vcf.gz` for per-sample)
- **Overlap / difference:**  
  `bcftools isec -p output_dir file1.vcf.gz file2.vcf.gz`  
  (produces files for variants only in file1, only in file2, or in both)

Use this to compare **variant-level** overlap between our pipeline’s VCF (or vcf-to-variants input) and another lab’s VCF, or between two pipelines.

---

## 4. VIP-Report (VCF → clinical-style report)

**What it does:** Generates clinical-style reports from VCF (HTML, custom templates, pedigree, HPO phenotype). Java 21.

**Good for:** Comparing **report structure and content** (which variants/genes appear, how they’re grouped) rather than exact wording. Different focus than our conditions/cascade/survival report.

- **GitHub:** https://github.com/molgenis/vip-report (LGPL-3.0)  
- **Run:** Java 21 + their CLI; see repo for “how to run” and example outputs.

---

## 5. Variant Effect Predictor (VEP) – annotation only

**What it does:** Annotates variants (consequence, gene, transcript, etc.). No “report” in the sense of conditions or drug recommendations.

**Good for:** Checking that **variant positions and alleles** in our set match what a standard annotator sees (same chr:pos:ref:alt), or pulling gene/consequence for your own comparison tables.

- **Web:** https://www.ensembl.org/info/docs/tools/vep/index.html  
- **CLI:** `vep -i input.vcf -o output.vcf --vcf` (plus cache, etc.)

---

## Suggested comparison workflow

| Goal | Tool | What to compare |
|------|------|------------------|
| Match or beat Sequencing.com targets | Our report | **Sequencing.com parity** tab (X/Y rsIDs found). |
| Star allele / PGx | **PharmCAT** (and StellarPGx if you have BAM) | Diplotypes vs our **Star alleles** tab. |
| Raw variant set vs another VCF | **bcftools stats** + **bcftools isec** | Counts and overlap vs our vcf-to-variants or pipeline VCF. |
| Report style / content | **VIP-Report** | Which variants/genes appear and how; we add MCAS, cascade, survival, parity, “Beyond”. |

We stay **on par** with Sequencing.com (parity tab) and **better** by adding MCAS, cascade, survival, ClinVar, supplements, and your own extraction; use PharmCAT and bcftools (and optionally StellarPGx, VIP-Report, VEP) as external checks.
