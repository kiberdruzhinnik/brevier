# Tasks

## 1. Yandex Disk page metadata and listing

- [x] 1.1 Add configurable `limit` and `offset` to public-resource metadata requests and parse the folder page's `total`, `offset`, and item count; verify URL construction and malformed or inconsistent pagination metadata with focused Rust tests.
- [x] 1.2 Combine entries from ordered page responses into one Atom listing while preserving the share URL and child paths; verify a multi-page fixture emits every entry once in API order with correctly escaped links.

## 2. Fetch all pages safely

- [x] 2.1 Update the Yandex folder fetch path to request further pages only until the reported total is collected, reject non-progressing or inconsistent pages, and propagate request failures instead of returning partial results; verify single-page, multi-page, and later-page failure cases with deterministic tests.

## 3. Integration verification

- [x] 3.1 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and applicable `cargo test` targets; run the `yandex_disk` fuzz target for a few minutes and record results.
- [x] 3.2 Open `https://disk.yandex.ru/d/yuSVtZXooWpEOw` with the Linux CLI and verify the listing contains all 44 entries, including entries beyond the first 20-item API page.
