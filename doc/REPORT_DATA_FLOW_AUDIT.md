# Genetic report data flow — detailed audit

**Goal:** Find why the main (full) genetic conditions report can show Person A’s data when generating a report for Person B (e.g. Henry’s data when generating for Lisa). The proof report correctly differs; the full report does not.

This doc traces every step from “run for Lisa” to “HTML shows Henry’s data” and lists the only possible causes.

---

## 1. Single process, single read

The report binary (`genetic-report-html`) has **one** place where variant data is read:

| File | Lines | What happens |
|------|--------|----------------|
| `crates/genetic_conditions/src/bin/report_html.rs` | 61–65 | `read_path` = `GENETIC_REPORT_INPUT_FILE` (env) **or** `args[1]`. No other source. |
| same | 67–104 | `match args.len()`: for 2–5 args, we `File::open(path)` where `path = read_path.ok_or(...)?`. So we open exactly one path. |
| same | 105–113 | `reader.read_to_string(&mut raw)` then `serde_json::from_str(&raw)` → one `Vec<VariantInput> variants`. |
| same | 169, 252 | `report = check_variants_against_all(&variants)`; then `all_conditions_to_html(..., Some(&variants), ..., variant_file_path_display)`. |

So the HTML is built from the single `variants` Vec that came from the single file we opened. There is **no** second read, no fallback path, no global cache of variants in this binary.

---

## 2. Where does `read_path` come from?

```text
read_path = env::var("GENETIC_REPORT_INPUT_FILE").ok().as_deref()
            .or_else(|| args.get(1).map(|s| s.as_str()))
```

- If **GENETIC_REPORT_INPUT_FILE** is set → we use that path (and ignore args[1] for **reading**; we still use args[2], args[3], args[4] for output/copy_number/report_name).
- Else → we use **args[1]**.

So the **only** way the report can show Henry’s data when “running for Lisa” is:

1. The path we use (env or args[1]) **points to a file that contains Henry’s variants**, or  
2. The user is **not** running the command they think (e.g. opening an old HTML, or a different script), or  
3. The **input file** they think is Lisa’s was actually built from Henry’s data (upstream mistake).

---

## 3. Script-by-script: what path is passed?

### run-report-from-lisa-folder.sh (Lisa’s report from her folder)

| Step | What happens |
|------|-------------------------------|
| 1 | `VARIANTS_JSON="$LISA_FOLDER/.variants_lisa_$$.json"` (run-unique file in Lisa’s folder). |
| 2 | Wipes `target/variants_*.json` so no stale Henry file. |
| 3 | `txt-to-variants "$INPUT_TXT"` → stdout to `$VARIANTS_JSON`. `INPUT_TXT` is from Lisa’s folder (ULTIMATE-COMPATIBILITY*Lisa*.txt or genome_lisa_haskell*.txt). So **file content = Lisa’s**. |
| 4 | `export GENETIC_REPORT_INPUT_FILE="$VARIANTS_JSON"`. |
| 5 | `cargo run ... genetic-report-html -- "$VARIANTS_JSON" "$OUT" "Lisa Haskell"`. So args[1] is also the same path. |
| 6 | Binary: `read_path` = GENETIC_REPORT_INPUT_FILE = that path → open it → read Lisa’s JSON → report = Lisa’s. |
| 7 | Script deletes `$VARIANTS_JSON` after the run. |

So when this script runs correctly, the report **must** be from Lisa’s data. The only way Henry’s data appears is if:

- The script wasn’t used (user ran something else or opened an old file), or  
- `GENETIC_REPORT_INPUT_FILE` was not inherited (e.g. run from an environment that strips env), and at the same time args[1] pointed to Henry’s file — but the script passes args[1] = `$VARIANTS_JSON` (Lisa’s temp file), so that would only happen if the user modified the script or ran the binary manually with a different first arg.

### run-report-from-mdng.sh

- Writes variants to a run-unique file next to the output (e.g. `.variants_<basename>_$$.json`).  
- Sets `GENETIC_REPORT_INPUT_FILE` to that file, then runs genetic-report-html with that path as args[1].  
- Same conclusion: the only path the binary can read is the one just written for this run.

### run-report-batch-privacy.sh

- Per person: writes `target/variants_batch_$$_${n}.json`, then (after fix) sets `GENETIC_REPORT_INPUT_FILE="$VARIANTS_TMP"` and runs genetic-report-html with that path.  
- So each report is forced to read from that run’s temp file only.

### run-report-and-open.sh

- Converts one VCF to `target/variants_from_vcf.json`, clears target variant files at start.  
- Now sets `GENETIC_REPORT_INPUT_FILE="$VARIANTS_FROM_VCF"` and runs genetic-report-html with that path.  
- So the report reads only from the VCF-derived file from this run.

---

## 4. What the binary does *not* do

- It does **not** read from a fixed path (e.g. `target/variants_from_mdng.json`) unless that path is given as env or args[1].  
- It does **not** have a fallback like “if file missing, use another path”.  
- It does **not** load or embed any other person’s variant set.  
- **html_report.rs** does not read any file; it only renders the `report` and `variants` passed in.

So “wrong person” cannot come from inside the report binary or HTML layer. It must come from **which path** is passed in or **which file** that path points to.

---

## 5. Checklist when the main report shows the wrong person

1. **Stderr when you ran the report**  
   Look for:
   - `[report] INPUT SOURCE: GENETIC_REPORT_INPUT_FILE (ignoring args) -> <path>`  
   - `[report] Reading variants from: <path> (N variants)`  
   If the path is in Lisa’s folder (e.g. `.variants_lisa_*.json`) and N matches Lisa’s variant count, the binary **did** read Lisa’s file. If the path is `target/...` or a Henry path, the wrong path was used.

2. **Report header**  
   - “Variant file used (this report only):” should show the path that was used.  
   - “Dataset: N variants • first chr:pos • last chr:pos” should match the intended genome (e.g. Lisa’s N and positions). If it matches Henry’s, the same variant set was used.

3. **How you ran the report**  
   - Did you run `./scripts/run-report-from-lisa-folder.sh` (and the correct folder)?  
   - Or did you run the binary directly, or another script, with a different first argument?  
   - Are you opening an **old** `Lisa_Haskell_report.html` (e.g. from before a fix)? Try Cmd+Shift+R or a private window and check “Generated at” and “Dataset”.

4. **Upstream input**  
   - If the “Lisa” MDNG or text export was built from Henry’s VCF by mistake, the report will show Henry’s results even though the pipeline is correct. Rebuild the input from Lisa’s data only.

---

## 6. Data tracker (REPORT_DATA_TRACKER=1)

With `REPORT_DATA_TRACKER=1`, the binary writes `report_data_tracker_<pid>.txt` with tap points (stage, count, first/last chr:pos, 100 sample points) at each pipeline stage.  

- Run once for Lisa (with her input path) and once for Henry (with his).  
- Diff the two tracker files. The **first** line/section where they differ (or where they unexpectedly match) shows where the data diverges (or is wrongly shared).  
- If tap 01_after_parse already differs, the input file was different. If it’s the same for both runs when it shouldn’t be, the same input path was used for both.

---

**100-tap result (Lisa run):** All taps showed 631991 variants, first 1:69869 last MT:16526 — no stage swaps data. The bug is not in the pipeline. The issue is which file you open: the Lisa script writes to Lisa's folder; opening ~/Downloads/genetic_report.html (e.g. from a prior Henry run) shows Henry's report. The Lisa script now opens the report it just wrote by default.

---

## 6.1 Intercept + 1/2 split (find corruption stage)

When the report shows the wrong person’s data, use **intercept** to see the variant set **just before the input to each stage**, then compare two runs to find the first stage where data is wrong.

1. **Run once per person** with intercept on (from the **project root** or the dir where you want the intercept files):
   ```bash
   REPORT_INTERCEPT=1 REPORT_RUN_ID=Siva GENETIC_REPORT_INPUT_FILE=/path/to/siva_variants.json cargo run -p genetic_conditions --bin genetic-report-html -- /path/to/siva_variants.json /tmp/siva_report.html
   REPORT_INTERCEPT=1 REPORT_RUN_ID=Henry GENETIC_REPORT_INPUT_FILE=/path/to/henry_variants.json cargo run -p genetic_conditions --bin genetic-report-html -- /path/to/henry_variants.json /tmp/henry_report.html
   ```
   This writes `report_intercept_Siva_01_after_parse.txt` … `report_intercept_Siva_12_final_variants.txt` and the same for Henry.

2. **Compare stage by stage** (e.g. in a shell):
   ```bash
   for s in 01_after_parse 02_after_gene_annotation 03_after_clinvar 04_before_check_variants 05_after_check_variants 06_before_survival 07_before_mcas_integrated 08_before_exercise_ammonia 09_before_star_alleles 10_before_parity 11_before_ref_check 12_final_variants; do
     echo "=== $s ==="
     diff report_intercept_Siva_$s.txt report_intercept_Henry_$s.txt
   done
   ```
   Each file is one line: `count first chr:pos last chr:pos`.

3. **1/2 split:** The **first** stage where Siva’s fingerprint **equals** Henry’s (when they should be different) is at or after the corruption. If they differ at 01–05 and match at 06, the corruption is between stage 05 and 06. If they differ at every stage, the pipeline kept the data correct and the bug is elsewhere (e.g. which HTML file is opened).

---

## 7. Summary

| Question | Answer |
|----------|--------|
| Can the report binary read two different variant files in one run? | No. One path (env or args[1]), one read, one `variants` Vec. |
| Can the HTML template pull in another person’s data? | No. HTML is built only from the `report` and `variants` passed to `all_conditions_to_html`. |
| Can a script pass the “wrong” path? | Yes. If the script (or the user running the binary manually) passes Person A’s path when generating Person B’s report, the report will show Person A’s data. |
| Mitigation | All scripts that invoke genetic-report-html now set **GENETIC_REPORT_INPUT_FILE** to the path they intend, so the binary ignores args[1] for reading and uses that path only. Stderr logs “INPUT SOURCE: GENETIC_REPORT_INPUT_FILE -> path” or “args[1] -> path” so you can confirm which file was used. |

**Conclusion:** The only way the main report shows Henry’s data when generating for Lisa is (1) the path used for reading pointed to Henry’s file, or (2) the user is viewing an old/cached report or ran a different command, or (3) the “Lisa” input file was built from Henry’s data. There is no in-process mixing or caching of two people’s variant sets in the report binary or HTML layer.
