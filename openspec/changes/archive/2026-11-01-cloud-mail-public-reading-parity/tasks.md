# Tasks

## 1. Cloud Mail parsing and fixtures

- [x] 1.1 Add minimized Cloud Mail folder, file metadata, dispatcher, and error fixtures without temporary download URLs or share secrets; verify fixture parsers accept supported responses and reject malformed or unsafe inputs.
- [x] 1.2 Add unit coverage for public URL decoding, resource classification, nested Atom links, metadata-only PDF recognition, and HTTPS download-host validation; verify with the focused Cloud Mail tests.

## 2. Resource dispatch and reading behavior

- [x] 2.1 Implement metadata-first folder/PDF/unsupported classification and route recognized public-share failures to existing read errors; verify tests cover extensionless PDF shares, non-PDF files, and metadata/download failures without shell fallback.
- [x] 2.2 Preserve folder navigation and pass Cloud Mail PDF bytes through the shared extractor; verify title/text and no-text extraction behavior with focused tests and the supplied public folder/PDF in the CLI.

## 3. Fuzz coverage

- [x] 3.1 Add and seed a Cloud Mail libFuzzer target covering URL, metadata, Atom, and download resolver parsing; verify it builds and runs for several minutes without a crash, hang, or sanitizer finding.

## 4. Integration checks and Android artifact

- [x] 4.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`; record results, excluding UI-feature checks as requested.
- [x] 4.2 Run `corpus/frozen.py snapshot` once, approve the base commit output, and compare the change output; verify any differences are intended and saved-page rendering completes.
- [x] 4.3 Run the console reader against the supplied Cloud Mail folder and one nested PDF; verify entries are listed and the PDF title/text are readable, with no JavaScript shell output on inaccessible/unsupported cases.
- [x] 4.4 Build the Android debug APK using the repository's documented command and provide the resulting APK path; verify the APK file is produced, excluding emulator/device checks as requested.
