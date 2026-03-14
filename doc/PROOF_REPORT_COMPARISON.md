# Proof report comparison: Lisa vs Henry

**Conclusion: The minimal pipeline (one file in → one report out) produces different output for each person. The display is driven by the file read.**

## 1. Genotype comparison (23andMe vs Sequencing.com for Lisa)

| rsID      | 23andMe | Sequencing.com | Match |
|----------|---------|----------------|-------|
| rs1801133 | AG      | GA             | Yes (same alleles) |
| rs3131972 | AG      | AG             | Yes |
| rs12124819 | AA     | AA             | Yes |
| rs429358 | CT       | TC             | Yes (same alleles) |
| rs1799945 | CC      | CC             | Yes |

**So the Sequencing.com file in Lisa's folder is the same person as her 23andMe export.** If the full report still looked "wrong", it is not because the Sequencing.com file is Henry's data.

---

## 2. Minimal proof report (one file → one HTML)

**Lisa** (from `ULTIMATE-COMPATIBILITY-LisaHaskell-*.txt` → variants JSON in her folder):

- **File read:** `.../Lisa New Genome Squencing feb 20 2026/.proof_lisa_28372.json`
- **Variant count:** 631,991
- **First variant:** chr 1 pos 69869 rsid rs548049170 genotype TT
- **Last variant:** chr MT pos 16526 rsid i701671 genotype G
- **MTHFR rs1801133:** chr 1 pos 11856378 genotype GA

**Henry** (from small VCF subset `henry_small.vcf` → `henry_small_variants.json`):

- **File read:** `target/henry_small_variants.json`
- **Variant count:** 270 (subset of his VCF)
- **First variant:** chr 1 pos 10013 ref T alt G genotype 1/1
- **Last variant:** chr 1 pos 680774 rs202064409 genotype 0/1
- **MTHFR rs1801133:** not found in this variant set (subset doesn’t include that position)

The two proof reports **clearly differ** (different paths, counts, first/last variant, and MTHFR presence/genotype). So when the program reads a given file, it displays that file’s data.

---

## 3. Implications

- **Data path:** One variant JSON path in → one report out. No shared state in the proof report.
- **Full report:** The same binary (`genetic-report-html`) reads one path and builds report, cascade, star alleles, etc. from that single in-memory variant set. So the full report is also file-driven.
- If the **full** report still looked like the wrong person, possible causes are: (1) the **script** passed the wrong path for that run, (2) **browser cache** (use the proof report’s “Generated” time and hard refresh), or (3) **expectation** (e.g. same condition names/“No matching variants” for both people when neither has a hit).

**Next step:** Use the proof report as the trusted baseline. For any run, confirm stderr shows `[report] Reading variants from: <path>` and that the report’s “Dataset” / “Generated at” match that run. Then grow back from the minimal report if needed.
