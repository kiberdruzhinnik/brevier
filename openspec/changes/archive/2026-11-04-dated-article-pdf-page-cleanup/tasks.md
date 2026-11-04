# Tasks

## 1. Dated article-series detection

- [x] 1.1 Add a normalized text signature for `ув. друзья` and `автор этих взволнованных строк`, gated by a `YYYYMMDD -` PDF filename prefix; verify positive and negative classifier tests, including that a date prefix or phrase alone does not classify a PDF.
- [x] 1.2 Route matching PDFs through the existing positioned extraction path and remove integer-only first or last nonblank lines per page; verify page-edge numbers disappear while body numbers and nonmatching PDFs remain unchanged.
- [x] 1.3 For PDFs already classified as Skladchik-origin, remove the inserted `Кримсон Аналитика – CrimsonAnalytics` phrase while preserving adjacent article text; do not use the phrase to classify PDFs.
- [x] 1.4 Join visual wraps of the initial headline in identified Skladchik PDFs with spaces into one logical heading, without merging later headings or body text.

## 2. Provider and regression coverage

- [x] 2.1 Verify Yandex Disk and Cloud Mail PDFs use the same dated-series classification and cleanup; run their focused adapter and PDF extraction tests.
- [x] 2.2 Re-run extraction on the supplied April and October PDFs and verify page numbers are absent, article numbers remain, and paragraph and heading spacing is readable; check a nonmatching PDF retains ordinary output.
- [x] 2.3 Add regression coverage for the supplied Cloud Mail PDF: the inserted branding disappears from the Skladchik article while surrounding text remains intact.
- [x] 2.4 Verify that a non-Skladchik PDF containing the same phrase retains it unchanged.
- [x] 2.5 Add a regression for the supplied Cloud Mail PDF: the five visual headline lines render as one heading with a clear break before the body.

## 3. Integration checks

- [x] 3.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, relevant Rust tests, and a short PDF/extraction fuzzer run; record results in the change summary. Checks passed: formatting, clippy, focused PDF/Yandex Disk/Cloud Mail tests, supplied April and October PDF regressions, and 61-second PDF fuzz run (494,018 executions, no crash).
- [x] 3.2 Re-run focused PDF and provider tests and the PDF fuzzer after implementing the branding cleanup. Checks passed: 16 PDF tests, 8 Yandex Disk tests, 4 Cloud Mail tests, full Rust suite (485 library, 9 CLI, 20 wire tests), `cargo fmt --check`, clippy, and 121-second PDF fuzz run (1,116,096 executions, no crash).
- [x] 3.3 Re-run focused PDF/provider tests and the PDF fuzzer after implementing headline joining. Checks passed: 17 PDF tests, 8 Yandex Disk tests, 4 Cloud Mail tests, `cargo fmt --check`, clippy, and 121-second PDF fuzz run (1,205,535 executions, no crash).
