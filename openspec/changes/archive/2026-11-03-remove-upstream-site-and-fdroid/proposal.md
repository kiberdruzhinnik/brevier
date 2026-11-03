# Proposal

## Why

This fork should describe and distribute itself through its own GitHub repository instead of linking to the upstream project's site and stores. Removing the obsolete site, roadmap, and F-Droid channel will make the fork's identity and supported installation path clear.

## What Changes

- **BREAKING:** Stop publishing through F-Droid and remove its Fastlane metadata and build-specific accommodations; GitHub Releases remain the Android APK distribution channel.
- Remove the project website and its GitHub Pages deployment, including all references to `brevier.bizibah.com`.
- Delete `ROADMAP.md` and remove references to it from current documentation and code comments.
- Point active repository, release, and issue links to this fork, with one explicit attribution that it was forked from `https://github.com/gurov/brevier`.
- Preserve the Android application ID and the generic unsigned build used to verify release reproducibility.

## Capabilities

### New Capabilities

### Modified Capabilities
- `product-distribution`: Specify GitHub Releases as the Android app's sole public distribution channel.

## Impact

README and contributor/agent documentation; `Cargo.toml`; Android Gradle and Rust build comments/configuration; `packaging/icons.py` and release notes; the `fastlane/` directory; `site/` and `.github/workflows/pages.yml`; runtime issue links and their test; `ROADMAP.md`; and the `product-distribution` specification.
