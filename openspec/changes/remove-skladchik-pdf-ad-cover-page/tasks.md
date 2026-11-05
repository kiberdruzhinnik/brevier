# Tasks

## 1. Identify and specify the cover signature

- [x] 1.1 Inspect the supplied PDF's first page and identify the additional page-local cues that distinguish its Skladchik ad from article text; verify the purchase phrase alone does not satisfy the classifier.
- [x] 1.2 Add focused regression inputs for the supplied ad cover, a first page with only the purchase phrase, and an article page containing Skladchik references; verify only the ad cover is classified.

## 2. Remove only a confirmed ad cover

- [x] 2.1 Add first-page Skladchik ad detection to the shared PDF extraction path and omit only page one on a positive page-local match; verify ordinary or uncertain first pages remain in output.
- [x] 2.2 Verify the supplied PDF output starts with the article after cover removal and preserves every subsequent page's text and order.
- [x] 2.3 Verify Cloud Mail and Yandex Disk PDFs both use the shared behavior, with their existing metadata and title handling unchanged.

## 3. Regression and parser checks

- [x] 3.1 Compare the base and changed binaries on the same supplied PDF and inspect every output difference; verify only the detected cover page is removed.
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo test`, and `cargo test --features images,typeset`; record results.
- [x] 3.3 Run the `pdf` fuzz target in `fuzz/` for a few minutes and verify it reports no crash or hang.
