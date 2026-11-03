# Proposal

## Why

Readers need article copies in a folder that other Android apps can access. Brevier's current weekly cache and permanent archive are private to the app, and the weekly cache also stores downloaded images.

## What Changes

- Add an opt-in setting for selecting a shared Android folder for cached article Markdown.
- When enabled, read and write current cached article copies in that folder; refreshes replace the previous copy for the same article.
- Store plain Markdown only, without downloaded image or other asset files.
- Keep the existing private cache behavior when the shared folder is disabled.

## Capabilities

### New Capabilities
- `shared-markdown-cache`: Select and use an Android shared folder as the source of current cached article Markdown.

### Modified Capabilities

## Impact

Android settings and folder selection, the Kotlin-to-Rust core interface, and the article cache/read path. The Android Storage Access Framework provides access to the selected folder; cache records need a stable URL-derived identity and plain Markdown content. Existing private cache and permanent reading archive behavior remain available when the feature is off.
