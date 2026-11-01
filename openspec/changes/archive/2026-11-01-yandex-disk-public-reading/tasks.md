# Tasks

## 1. Yandex public-share adapter

- [x] 1.1 Add Yandex Disk public-link parsing and public-resource API URL construction for share roots and paths; verify accepted and rejected URL forms with focused unit tests.
- [x] 1.2 Parse Yandex metadata for files and folders, produce folder entries with links that preserve the public share and nested paths, and validate temporary PDF download links; verify saved API response fixtures and malformed or restricted responses with focused tests.
- [x] 1.3 Add a fuzz target for Yandex public-link and API-response parsing, seed it with the saved response fixtures, and run it for several minutes without a crash or invariant failure.

## 2. Core reading integration

- [x] 2.1 Dispatch recognized Yandex public shares through metadata, folder-feed conversion, and local PDF text extraction in the shared Rust reading path; verify CLI/core tests cover folders, nested entries, PDF content, unsupported files, and inaccessible shares.
- [x] 2.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`; resolve any failures. UI-feature checks are out of scope.
- [x] 2.3 Run `corpus/frozen.py snapshot`, approve the baseline on the base commit, and run `corpus/frozen.py` on the change; confirm no unintended page-model differences.
- [x] 2.4 Run the CLI against `https://disk.yandex.ru/d/iCbUDNZAMJYppQ`, inspect the console output for the expected share title and folder entries, and follow a listed PDF if the share contains one; record any access or service limitation in the result.

## 3. Android APK delivery

- [x] 3.1 Build the Android debug APK with `cd android && ./gradlew assembleDebug`; verify the APK exists and its package metadata identifies Brevier.
