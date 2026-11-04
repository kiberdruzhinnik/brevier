# Proposal

## Why

The fork is still at version 0.19.2 and lacks useful core-reading and Android features shipped upstream through 0.22.0. Bringing over the compatible parts as one planned update keeps the reader current while preserving this fork's Android 12 support, Android-only distribution, and prior product decisions.

## What Changes

- Add Android reading controls from upstream: paper and paragraph presentation, larger text sizes, optional full-screen reading, tab swipe-close with undo, and configurable page-turn gestures and controls.
- Improve reading reliability with safer large-image decoding, recovery after a failed page render, Android 9 bitmap handling, and correct regional homepage extraction.
- Add an opt-in Android custom User-Agent setting and improve address/search suggestion behavior when the keyboard is open.
- Preserve already downloaded article pictures with saved articles, without extra network requests and within a per-article storage limit.
- Align the shared Rust and Android app release version to 0.22.0.
- Keep existing Android 12 compatibility and the fork's existing toolbar-position, cloud-share, and shared Markdown cache behavior.
- Exclude upstream Windows/GTK/desktop packaging, website and publishing work, F-Droid support, and removed notes functionality. Do not repeat the script-gate and Wayback behavior already present in this fork.

## Capabilities

### New Capabilities
- `android-reading-experience`: Android reading appearance, immersive mode, tab management, page turning, and address/search interaction behavior.
- `core-reading-reliability`: bounded image processing, failed-page recovery, Android bitmap transport compatibility, regional homepage extraction, and configurable request identity.
- `archived-article-pictures`: retention of fetched article pictures in saved article copies.
- `release-versioning`: consistent core and Android app version metadata.

### Modified Capabilities
- `android-compatibility`: Specify Android 9 bitmap rendering compatibility while retaining the existing API 24 minimum and Android 12 support.

## Impact

Likely changes span Rust extraction, image decoding, fetch configuration, Android core protocol handling, Android settings and navigation, saved-article storage, and Cargo/Gradle version metadata. The Android app remains the only end-user application; Linux CLI remains internal for parser checks. No new runtime dependency or desktop product is proposed.
