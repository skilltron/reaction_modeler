# Build profiles and optimization

## Profile

**release** (`cargo build --release`): `opt-level = 3`, **full LTO** (`lto = true`), `codegen-units = 1`, `strip = true`, `panic = "unwind"`. Cold build ~7–8 s on this workspace (one crate).

## Code-level throughput practices (already applied)

- Preallocated `Vec::with_capacity(variants.len())` for findings in immune, exposure, inflammation, sulfur, rare.
- Preallocated `Vec::with_capacity(64 * 1024)` for the HTML report buffer.
- Exposure reports built with `Vec::with_capacity(5)` and a single loop (no iterator chain).
- `#[inline(always)]` on `check_variants_against_all` for hot-path inlining.

## Panic

Release uses **panic = "unwind"** so backtraces (e.g. `RUST_BACKTRACE=1`) work.
