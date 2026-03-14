# Privacy when running genomes (CLI)

**No genetic data is ever sent out.** All report generation runs entirely on your machine. The pipeline uses only local Rust binaries and local files; it has no network code and does not send VCFs, variant JSON, or report content to any server or third party. Your data stays on your device.

## How to protect privacy when running multiple genomes

1. **Run from the CLI on your machine only**  
   Use `scripts/run-report-and-open.sh` or `scripts/run-report-batch-privacy.sh`. No cloud or external services are used.

2. **Do not paste genomic data into chat or email**  
   Do not paste VCF contents, variant JSON, or report HTML that contains identifiable genetic findings into Cursor, email, or other tools. Share only paths, commands, or high-level outcomes (e.g. "run completed", "report_003.html generated").

3. **Use anonymized output paths**  
   Write reports to neutral filenames (e.g. `report_001.html`, `report_002.html`) in a folder only you can access, not to names that identify the person.

4. **Avoid leaving variant JSON in a shared place**  
   The batch script uses a temporary variants file per run and removes it after each report so the last run’s variants are not left in `target/variants_from_vcf.json` for the next run.

5. **Optional: disable opening the report in a browser**  
   For batch runs you can use the batch script, which does not open the browser, so reports stay in the output directory until you open them yourself.

## Quick reference

- **Single run (report opens in browser):**  
  `./scripts/run-report-and-open.sh /path/to/file.vcf.gz /path/to/output/report_anon.html`  
  Omit the second argument to use default output path.

- **Batch runs (anonymized, no browser open):**  
  `./scripts/run-report-batch-privacy.sh /path/to/output_dir /path/to/vcf1.vcf.gz /path/to/vcf2.vcf.gz ...`  
  Produces `report_001.html`, `report_002.html`, … in `output_dir`.  
  See script for usage.

- **Copy number (optional):**  
  If you have a copy-number JSON for a run, use the three-argument form of `genetic-report-html` (variants JSON, output HTML, copy-number JSON); the batch script does not currently pass copy-number files.
