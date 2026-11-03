# Tasks

## 1. Extend notes in the shared core

- [x] 1.1 Add persistent picture attachments to the existing quote-anchored note model while keeping existing `notes.tsv` entries readable; verify old and picture-bearing notes round-trip in core tests.
- [x] 1.2 Add page and all-notes Markdown exports with passage links and images in ZIP or Markdown-plus-`images/` form; verify paths, content, and image bytes in save/export tests.
- [x] 1.3 Include note pictures when saving an article and retain images while pruning or removing notes; verify archive contents and note lifecycle in tests.
- [x] 1.4 Fuzz note parsing, placement, and export inputs with the existing `notes` fuzz target; verify a bounded fuzz run completes without a crash or hang.

## 2. Integrate native notes interfaces

- [x] 2.1 Add the responsive GTK notes margin and narrow-window fallback, including the 0.19.1 layout fixes; verify the GTK feature compiles and the frozen corpus comparison shows the intended page layout changes.
- [x] 2.2 Add Android note pictures, system photo picker, inline note placement, visibility setting, and note sharing; verify the Android debug APK builds and its native integrations compile, with export content covered by task 1.2.
- [x] 2.3 Add the upstream notes-margin screenshot and update reader-facing help and release text without removing Cloud Mail or Yandex Disk documentation; verify those host descriptions remain present.

## 3. Adopt upstream Android and dependency updates

- [x] 3.1 Set compile and target SDK to API 36 while keeping `minSdk = 24`, and route Back through the invoked dispatcher on Android 13+ with the legacy callback on older versions; verify with Gradle build and APK metadata inspection.
- [x] 3.2 Upgrade ZIP writing to version 9 and reconcile Cargo, fuzz, Gradle, and packaging locks/source lists; verify locked dependency resolution and existing ZIP writer tests.
- [x] 3.3 Set the release version to 0.19.2 and Android version code to 1902, then update package release metadata; verify Gradle's version-code consistency check and metadata validation.

## 4. Reconcile and verify the fork

- [x] 4.1 Merge the remaining upstream v0.19.0–v0.19.2 changes from the shared archive commit, resolving overlapping README, Cargo, fuzz lock, and packaging metadata while retaining Cloud Mail and Yandex Disk code and fixtures; verify the final diff contains both upstream changes and downstream host support.
- [x] 4.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo clippy --all-targets --features ui -- -D warnings`, `cargo test`, and `cargo test --features images,typeset`; record outcomes without running Cargo UI tests or Android emulator tests.
- [x] 4.3 Run the frozen corpus comparison for the notes/page-model changes and build the Android debug APK; verify the corpus result and APK SDK metadata (`minSdk 24`, `targetSdk 36`).
