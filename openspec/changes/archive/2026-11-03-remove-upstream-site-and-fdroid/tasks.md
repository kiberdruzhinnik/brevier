# Tasks

## 1. Retire the project site and roadmap

- [x] 1.1 Remove `site/` and `.github/workflows/pages.yml`; remove the obsolete Cargo homepage and point repository, release, and issue links at `kiberdruzhinnik/brevier`, with one upstream fork-attribution link in the README. Verify no `brevier.bizibah.com` reference remains and active GitHub links target this fork.
- [x] 1.2 Delete `ROADMAP.md` and remove its links and source comments from the README, contributor/agent guidance, manifesto, and code. Verify maintained files no longer refer to the deleted roadmap.

## 2. Remove F-Droid publishing support

- [x] 2.1 Delete the full `fastlane/` tree and remove Fastlane store-icon generation from `packaging/icons.py`, retaining Android launcher icon generation. Verify the build helper has no Fastlane output path.
- [x] 2.2 Remove F-Droid-only Android Gradle configuration and rationale while retaining the release signing configuration, package ID, pinned toolchain, generic reproducibility build, and runtime certificate-verifier module. Verify the Android app and verifier module still build.

## 3. Verify distribution and project cleanup

- [x] 3.1 Update README, contributor/agent/security guidance, Android build comments, historical release notes, and the archived Android-only change to remove F-Droid references and describe GitHub Releases as the only public APK channel. Add one upstream fork attribution and point active repository, release, and issue links at this fork; verify install guidance links to this fork's releases.
- [x] 3.2 Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, and `cargo test --features images,typeset`; build the Android app without emulator tests. Verify CI retains signed APK publishing and the unsigned reproducibility comparison.
- [x] 3.3 Search the repository for `brevier.bizibah.com`, `ROADMAP.md`, and F-Droid names; validate the change and the updated product-distribution spec, confirming no retired site or store link remains.

## Workflow follow-up

- Sync the product-distribution delta into the main spec and archive the change after implementation review.
