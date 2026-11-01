# Proposal

## Why

This fork is based on Brevier 0.18.0 and has since added public Cloud Mail and Yandex Disk reading. Bringing in upstream 0.19.2 adds the later notes experience and Android 16 target compatibility while keeping the existing cloud-share work and Android 12 support.

## What Changes

- Add the upstream notes margin, note pictures, Markdown export, and the related window and Android behavior.
- Target Android API 36 and handle system Back through the newer Android API where available, while preserving the existing back-stack order and `minSdk = 24`.
- Update the ZIP dependency and the associated lockfiles, Android build setup, CI, packaging, and release metadata to match upstream 0.19.2.
- Retain the plain Markdown archive support already present in this fork, along with Cloud Mail and Yandex Disk parsing and PDF reading.

## Capabilities

### New Capabilities

- `reading-notes`: Show notes in a margin, keep pictures with notes, and export notes as Markdown with their pictures.
- `android-compatibility`: Run with Android 12 and later supported by the existing minimum SDK, target API 36, and preserve Brevier's Back behavior across Android versions.

### Modified Capabilities

None.

## Impact

Affected areas include `src/notes.rs`, `src/media.rs`, `src/save.rs`, `src/store.rs`, the page model and GTK article/window code, Android article and activity code, ZIP dependencies and lockfiles, and CI and packaging metadata. Existing public cloud-share behavior and `minSdk = 24` are compatibility constraints for the integration.
