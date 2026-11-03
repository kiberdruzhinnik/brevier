# Tasks

## 1. Skladchik detection and scoped cleanup

- [x] 1.1 Add positive fixtures based on the supplied Marahovsky, SNMI, and Crimson shares for the combined trailing `skladchik.org`, club-heading, and purchase-notice signature, and negative cases for isolated phrases; verify uncertain and non-Skladchik PDFs retain ordinary extraction.
- [x] 1.2 Update the shared PDF path to classify content before selecting specialized extraction; verify ordinary PDFs bypass Skladchik cleanup and positively identified PDFs use it.
- [x] 1.3 For detected Skladchik PDFs, retain page boundaries and reflow line wraps while preserving paragraph breaks; verify page-edge numbers disappear and in-body numbered content remains.
- [x] 1.4 For detected Skladchik PDFs, remove the complete trailing advertisement beginning at its `skladchik.org` branded heading after confirming the purchase notice; verify prior article text is retained, wrapped markers are recognized, and body mentions do not truncate the article.
- [x] 1.5 Add regression fixtures or generated PDF cases for positive and negative classification, both public share providers, paragraph flow, page numbers, promotional tails, and empty extraction; verify provider-specific title and existing failure behavior remain covered.

## 2. Parser robustness and integration checks

- [x] 2.1 Extend the PDF cleanup fuzz target to exercise classification and both ordinary and specialized text paths, including page normalization and promotional-tail detection; run the target for a few minutes and record the result.
- [x] 2.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --features images,typeset`; record the outcome of each command.
- [x] 2.3 Run the extraction frozen-corpus workflow (`corpus/frozen.py snapshot`, approve on the base commit, then run on this change) and confirm every output difference is intended.
- [x] 2.4 Run the Linux CLI on representative Skladchik PDFs from Yandex Disk and Cloud Mail; confirm the full branded footer is removed. Verify ordinary PDF extraction remains unchanged with the synthetic PDF regression case because the supplied public shares contain only Skladchik PDFs.
