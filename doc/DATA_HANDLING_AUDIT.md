# Data handling audit — where data is read/written and how it can break

This doc lists every place the report pipeline reads or writes variant/report data, so we can fix "multiple data handling breaks" in one place.

## Data flow (batch report)

1. **Input:** One VCF path per person (e.g. `/path/to/Lisa.vcf.gz`).
2. **Script** clears all shared temp files in `target/` (see below).
3. **Script** runs `vcf-to-variants` with that VCF only; stdout is redirected to a **unique** temp file: `target/variants_batch_$$_${n}.json`.
4. **Script** checks the temp file: non-empty and contains `"chromosome"` (real variant data). If not, skip and do not call the report binary.
5. **Script** runs `genetic-report-html` with exactly: `$VARIANTS_TMP` (that temp path), `$out_path`, and optional name. No other variant source.
6. **Report binary** reads variant data **only** from the first argument (the JSON path). It never reads from a fixed path or env.
7. **Report binary** writes HTML to the second argument. It embeds a **dataset fingerprint** (variant count + first/last chr:pos) so the report proves which genome it was built from.
8. **Script** deletes the temp file; then repeats for the next person.

## Touch points (where things can break)

| Location | What it does | Failure mode | Mitigation |
|----------|--------------|--------------|------------|
| `run-report-batch-privacy.sh` | Writes to `target/variants_batch_$$_${n}.json` | Same path reused, or stale file from previous run | Unique path per run; clear all `variants_batch_*.json` at start and end |
| `run-report-batch-privacy.sh` | Reads VCF, pipes to vcf-to-variants | VCF missing, or header-only, or gzip -dc fails silently | Check file exists; check output has `"chromosome"` and -s |
| `run-report-and-open.sh` | Writes to `target/variants_from_vcf.json` | Next run could read this if script not used in batch mode | Script now clears this and batch temp files at start |
| `run-report-from-mdng.sh` | Writes to `target/variants_from_mdng.json` | Same | Script now clears at start |
| `genetic-report-html` (binary) | Reads JSON from **args[1]** only | Bug could read from wrong path | No other read path in code; fingerprint in report verifies dataset |
| `vcf-to-variants` (binary) | Reads from path or stdin | Returns [] for header-only VCF | Script rejects empty array via "chromosome" check |
| Env vars | `INCLUDE_SEQUENCING_PARITY`, `BATCH_PRIVACY` | Stale from previous shell | Batch script sets them at start of script |

## Shared temp files (must not persist between runs)

All of these are cleared at the start of the batch script and (where applicable) run-report-and-open and run-report-from-mdng:

- `target/variants_batch_tmp.json`
- `target/variants_batch_*.json`
- `target/variants_from_vcf.json`
- `target/variants_from_mdng.json`

## Verification

- **Dataset fingerprint in report:** Each HTML report includes a line like  
  `Dataset: N variants • first chr:pos • last chr:pos (verifies this report is from this genome only)`.  
  Different genomes must show different N and positions. If two reports show the same fingerprint for different people, that indicates a bug.
- **No report for 0 variants:** Script refuses to generate a report when the variant JSON is empty or has no `"chromosome"` (header-only VCF).

## If "everyone's data looks the same"

1. Check the **Dataset** line in each report. If they differ, the reports are from different inputs; the similarity may be template/structure, not variant data.
2. If the Dataset line is identical for two different people, then the same variant file was used twice — check that the batch script is using unique temp paths and that no other process overwrites them.
3. Ensure you are not passing the same VCF path for every run (e.g. typo or same path in a loop).

---

## Fixes applied (data handling)

- **Report binary** refuses to write any HTML when the variant set is empty: it exits with code 1 and prints an error. So even if a caller passes an empty or wrong file, no report is written.
- **Batch script** removes the temp variant file on every skip path (no variant data, or 0 variants / no `"chromosome"`), so no invalid JSON is left in `target/`.
- **Batch script** captures `vcf-to-variants` stderr in `target/vcf_err_$$.txt` and prints the first 3 lines when a run is skipped, so failures (parse error, I/O) are visible.
- **Cleanup** at start and end of batch: all `variants_batch_*.json`, `variants_from_*.json`, and `vcf_err_*.txt` in `target/` are deleted.
- **Dataset fingerprint** in every report: variant count plus first/last chr:pos so you can confirm which genome the report used.
