# Design

## Context

See proposal.md for motivation and specs for the user-visible behavior. The Rust core is shared with the Android app; Cargo currently supplies the Android version name while Gradle holds the Android version code. This fork already contains its own cloud-share parsers, shared Markdown cache, toolbar-position setting, and note-removal decisions, which must survive the merge.

## Goals / Non-Goals

**Goals:**
- Port the compatible core and Android behavior from upstream releases 0.20.0 through 0.22.0.
- Preserve the Android API 24 minimum, so Android 12 remains supported.
- Align Rust and Android release metadata at 0.22.0.

**Non-Goals:**
- Port Windows, GTK, desktop, website, F-Droid, or public CLI distribution work.
- Restore notes, annotations, or note-attached pictures.
- Replace fork-specific cloud-share, shared-cache, or toolbar-position behavior.
- Rework the already-present script gate or Wayback fallback.

## Decisions

### Port behavior by subsystem, preserving fork-specific code

Use the upstream release diff as a source for individual changes, then port the reading controls, recovery and image handling, regional homepage correction, request identity, saved article pictures, and version metadata into their corresponding existing Rust and Android components. Do not merge upstream files wholesale: that could reintroduce notes or unrelated distribution and branding changes. Keep core extraction deterministic and keep Android rendering native.

### Keep Android settings opt-in where upstream behavior was opt-in

Persist the full-screen and page-turn preferences as disabled by default. Preserve existing defaults for appearance and keep existing toolbar placement untouched. Tab swipe-close should provide a short undo opportunity; opening a link in a new tab should retain the originating tab as the Back destination.

### Bound and isolate image work

Reduce large decoded images before final resampling. Keep the per-article saved-picture cap at 32 MB and only persist images already downloaded for the article. Exclude these cached pictures from Android cloud backup. Recovery state should identify an interrupted page render so startup can avoid replaying that page, without discarding unrelated saved articles or tabs.

### Keep platform compatibility at the existing minimum

Correct the image transfer contract so Android 9 bitmap copying does not depend on an unsupported buffer offset or length assumption. Keep minSdk 24 and the API 36 target policy already recorded in android-compatibility; do not adopt upstream changes that require a newer Android release.

### Set one release identity

Set the Cargo package version and lockfile package entry to 0.22.0, and set the Android version code to 2200 while retaining the version name derived from Cargo. Validate the generated APK metadata so Rust and Android cannot silently diverge.

## Risks / Trade-offs

- [Upstream changes overlap fork-specific edits] → Port by behavior and inspect each upstream patch against current code; preserve local share/cache/toolbar features and the note-removal contract.
- [Image retention increases local storage] → Enforce the 32 MB per-article cap and exclude retained images from cloud backup.
- [Recovery markers can suppress a page after an unclean shutdown] → Clear the marker after successful rendering and keep explicit navigation available to reopen the page.
- [Gesture controls conflict with existing navigation] → Make each page-turn input independently configurable and disabled by default; verify Back and toolbar gestures remain available.
- [New presentation settings may not map directly to current typesetting options] → Treat them as explicit persisted preferences and verify layout at small and maximum text sizes.

## Migration Plan

1. Port each compatible upstream behavior into the current fork and add targeted regression coverage without changing the Android API minimum.
2. Keep existing stored articles and preferences intact; introduce defaults for new preferences without resetting current toolbar or theme choices.
3. Bump Cargo and Android release metadata to 0.22.0 / 2200 and verify APK metadata.
4. Build a release APK and check installation and core reading flows on Android 12 or later. If release validation fails, revert the version metadata with the implementation changes; no user-data migration is required.
