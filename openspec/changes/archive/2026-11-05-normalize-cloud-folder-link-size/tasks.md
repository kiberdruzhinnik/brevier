# Tasks

## 1. Render public cloud folder links at regular size

- [x] 1.1 Render Cloud Mail and Yandex Disk folder entries as regular paragraph links while retaining the folder title heading; add focused tests for both providers and an ordinary feed, and verify them with the targeted test command. `cargo test public_cloud_folder_entries_are_regular_links` passed.
- [x] 1.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test`; record the results. Formatting, Clippy, and the full test suite passed. `cargo test --features images,typeset --lib` also passed (492 tests).
