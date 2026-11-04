# Tasks

## 1. Remove app-owned note data on upgrade

- [x] 1.1 Add initialization cleanup for the app's `notes.tsv` and note-picture directory, and verify with a temporary data directory that both are removed while the internal article archive and unrelated app data remain.
- [x] 1.2 Exercise the upgrade cleanup path against existing note data and verify that cleanup happens before the updated app can expose note records or pictures.

## 2. Remove user-note behavior from the Rust core

- [x] 2.1 Remove the user annotation module, persistence model, image attachment, note export, archive retention, Android bridge actions, and note settings while preserving article footnote extraction; verify Rust library tests and footnote regression tests pass.
- [x] 2.2 Remove note attachment from article saving and note-only save APIs, and verify saved Markdown/ZIP outputs keep article content and article images but contain no appended user notes or note pictures.
- [x] 2.3 Remove the dedicated notes fuzz target and its fuzz-only dependencies if unused elsewhere; verify the remaining fuzz workspace builds and targets still reference supported modules only.

## 3. Remove Android note controls and current product descriptions

- [x] 3.1 Remove note state and controls from the reader, shelf, context menus, settings, picture picker integration, and share flows; verify the Android app compiles and no user-note action remains in the UI.
- [x] 3.2 Update current README and in-app descriptions to remove user-created notes and note exports while keeping article-footnote behavior accurately described; verify current product copy no longer promises note-taking.

## 4. Verify the integrated removal

- [x] 4.1 Run formatting, clippy, Rust tests, the frozen corpus comparison, and relevant remaining fuzz targets; verify article footnotes and article images remain unchanged and no user-note-specific module/API remains.
- [x] 4.2 Build the Android release APK and verify the artifact installs as an app build without relying on emulator tests.
