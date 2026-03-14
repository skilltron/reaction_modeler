# Display and data audit: genetic conditions report

**Purpose:** Confirm that the report display is 100% driven by the variant set passed into the run — no global state, no cached content, no wrong-person data.

---

## 1. Single source of truth: the variant file path

- **genetic-report-html** takes the variant set from **exactly one place:** when **GENETIC_REPORT_INPUT_FILE** is set, that path (and only that path) is used; otherwise `args[1]` (or stdin when no args).
- It opens that path (or stdin), reads once, parses JSON into a single `Vec<VariantInput>` in memory.
- There is **no** second read, no fallback to another file.
- On each run the binary logs: `[report] INPUT SOURCE: GENETIC_REPORT_INPUT_FILE -> <path>` or `args[1] -> <path>`, then `[report] Reading variants from: <path> (N variants)` so you can verify which file was used.

---

## 2. No global or shared variant state

- **No `static` or `lazy_static`** in the crate holds variant data or report content.
- **check_variants_against_all(variants)** — takes `&[VariantInput]`, returns a new `AllConditionsReport`. All submodules (immune, inflammation, sulfur, rare, cancer, disorders, exposure) receive only that slice and build their reports from it.
- **cascade, survival, mcas_integrated, exercise_ammonia, star_alleles, sequencing_parity, reference_check** — all take `&variants` or a report derived from it. None read from disk or a global.
- **gene_annotation** and **clinvar_lookup** — only add fields to the same in-memory variants; they do not replace the set with data from another source.

---

## 3. HTML is built only from passed-in data

- **all_conditions_to_html(...)** receives:
  - `report: &AllConditionsReport` (from `check_variants_against_all(&variants)`)
  - `variants_with_clinvar`, `star_alleles`, `dataset_fingerprint`, etc.
- Every section (MCAS, immune, inflammation, cascade, star alleles, etc.) is rendered from these arguments. Reference content (e.g. condition cards, stabilizer lists) is **static text** (what each condition is); **findings** and **your result** come from `report.*.findings` and the passed variant-derived data.
- There is no code path that says “if no findings, show another person’s data”. Empty findings produce “No matching variants” and the fingerprint still reflects the variant set that was passed in.

---

## 4. Scripts must pass the correct path

- If the **script** passes the wrong path to genetic-report-html (e.g. a path to Person A’s variants when generating Person B’s report), the report will show Person A’s data. The pipeline itself does not “mix” or “cache” people; the bug would be in **which file path** the script gives.
- **run-report-from-lisa-folder.sh** — writes variants to a **run-unique** file in Lisa’s folder (`.variants_lisa_$$.json`) from **only** the ULTIMATE-COMPATIBILITY or 23andMe file in that folder, then passes that path to genetic-report-html, then deletes the temp file. No shared `target/` file is used.
- **run-report-from-mdng.sh** — writes variants to a run-unique file **next to the output HTML** (`.variants_<basename>_$$.json`), passes that path, then deletes it. It also wipes `target/` variant files at start so no stale file exists.
- **run-report-batch-privacy.sh** — uses a unique `variants_batch_$$_${n}.json` per person and clears all batch/variant temp files at start and end.

---

## 5. How to verify a report is from the right genome

1. **Dataset line** in the report header: `N variants • first chr:pos • last chr:pos`. Compare N and first/last to the variant file you intended (e.g. from `txt-to-variants` or `mdng-to-variants` on the intended source).
2. **Generated at:** timestamp in the header. Confirms the report was built in this run (and HTTP no-cache meta reduces browser caching).
3. **Stderr when running:** `[report] Reading variants from: <path> (N variants)`. Confirm the path is the one you expect (e.g. in Lisa’s folder when generating Lisa’s report).

---

## 6. If the display still looks like the wrong person

- **Provenance of the input file:** If you use an MDNG or a text export, that file must have been built from **that person’s** data. If someone mistakenly built “Lisa_Haskell.mdng” from Henry’s VCF, then the report will show Henry’s results even though the pipeline is correct. Fix: rebuild the MDNG (or use the correct export) from that person’s own data.
- **Browser cache:** Force reload (e.g. Cmd+Shift+R) or open the report in a private window. The HTML now includes no-cache meta and a “Generated at” time so you can confirm you’re viewing the latest run.
- **Wrong script or path:** Ensure you’re running the script that uses the correct source (e.g. run-report-from-lisa-folder.sh for Lisa’s folder, not a script that points at Henry’s variant file).

---

## 7. Summary

| Layer | Uses only passed/local data? |
|-------|------------------------------|
| report_html.rs | Yes: reads once from args[1] (or stdin), one in-memory `variants` Vec |
| check_variants_against_all + all submodules | Yes: take `&[VariantInput]`, no globals |
| all_conditions_to_html | Yes: takes report and optional sections, no file read, no static report content |
| Scripts | Must pass the path to the variant file for the intended person; run-unique temp files and wipes prevent accidental reuse |

The display is not “100% broken” by design: it is fully driven by the single variant set given to the binary. Any wrong-person display must come from (1) the wrong file path being passed, or (2) the input file itself containing the wrong person’s data.
