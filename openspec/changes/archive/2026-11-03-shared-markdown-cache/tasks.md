# Tasks

## 1. Android folder selection and settings

- [x] 1.1 Add an off-by-default shared-cache setting and folder selection flow in Android settings; persist the selected tree URI and granted access, and verify cancellation, app restart, disable/re-enable, and re-selection behavior.
- [x] 1.2 Add Android-side document-store operations for deterministic `.md` child lookup, bounded reads, replacement, and creation; verify unavailable permissions and provider errors are handled without crashing.

## 2. Markdown cache and core integration

- [x] 2.1 Add a URL identity and plain Markdown cache-record format with source URL, title, saved time, and validation; verify same URL/fragment resolution, same-title distinct URLs, replacement, and invalid-file-as-miss behavior with focused Rust tests.
- [x] 2.2 Extend the Kotlin/Rust open contract to accept a validated shared Markdown hit and return successfully fetched article Markdown for storage; verify hit avoids fetch/extraction, miss falls through to fetch, refresh bypasses the shared hit, and page/history identity remains the original article URL.
- [x] 2.3 Route opens through the selected shared cache only when enabled, preserving the existing private cache path when disabled; verify the shared folder receives only `.md` files and no image or asset files.

## 3. Validation and release artifact

- [x] 3.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and applicable `cargo test` targets (including `--features images,typeset` for page-model tests); record results and skip Cargo UI tests and Android emulator tests as requested.
- [x] 3.2 Run `corpus/frozen.py snapshot` and compare approved base/current output for the page model; run the relevant cache/parser fuzz target for a few minutes and record results.
- [x] 3.3 Build the Android release APK and verify the generated artifact path and package metadata; do not run emulator tests.
