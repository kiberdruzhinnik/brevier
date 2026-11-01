# Proposal

## Why

Cloud Mail public folders and PDFs are already readable, but file recognition depends on the share URL suffix and API failures can fall through to the JavaScript shell. The provider parser also has no focused fixtures, unit tests, or fuzz target, leaving behavior less predictable and less exercised than Yandex Disk.

## What Changes

- Classify Cloud Mail public resources from the provider's API metadata and use the API-reported filename as the PDF signal when MIME metadata is absent.
- Preserve public folder listings and nested links, and route non-PDF files and inaccessible or malformed shares through Brevier's existing unsupported-content or read-failure behavior instead of silently falling through to the JavaScript shell.
- Add saved, minimized API response fixtures, parser unit tests, and a seeded libFuzzer target for Cloud Mail URL and response parsing.
- Keep the shared local PDF text extraction path and the same behavior in CLI and Android through the Rust core; build and provide a debug APK.

## Capabilities

### New Capabilities

None.

### Modified Capabilities

- `public-cloud-shares`: specify Cloud Mail folder navigation, metadata-based PDF reading, and safe failures for unsupported or inaccessible public shares alongside the existing Yandex Disk requirements.

## Impact

- `src/hosts/cloud_mail.rs` and `src/fetch.rs` for resource classification and read failure handling.
- `tests/fixtures/` and focused host tests for Cloud Mail API response shapes.
- `fuzz/fuzz_targets/` and `fuzz/Cargo.toml` for a Cloud Mail parser target and corpus seeds.
- No new runtime dependency; reuse the existing JSON parser, network limits, TLS verifier, and local PDF extractor.
- Validate with default Rust checks, the frozen page-model comparison, a console run of the supplied Cloud Mail folder and one PDF, and an Android debug APK build. UI-feature checks and emulator/device testing are out of scope.
